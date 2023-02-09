use std::borrow::Borrow;
use clap::{Args, Subcommand, ValueEnum};
use crate::util::error::Result;
use config::{Config, File};
use std::sync::{Arc, atomic::AtomicBool, RwLock};
use super::traits::SignCommand;
use std::path::PathBuf;
use tokio::runtime;
use crate::client::sign_identity;
use std::collections::HashMap;
use std::fmt::{Display, format, Formatter, Result as fmtResult, write};
use crate::util::error;
use async_channel::{bounded, RecvError};
use crate::client::load_balancer::{traits::DynamicLoadBalancer, single::SingleLoadBalancer};
use crate::client::load_balancer::factory::ChannelFactory;
use crate::client::worker::assembler::Assembler;
use crate::client::worker::signer::RemoteSigner;
use crate::client::worker::splitter::Splitter;
use crate::client::worker::traits::SignHandler;

const MAX_MESSAGES: usize = 1000;

lazy_static! {
    pub static ref FILE_EXTENSION: HashMap<sign_identity::FileType, Vec<&'static str>> = HashMap::from([
        (sign_identity::FileType::RPM, vec!["rpm", "srpm"]),
        (sign_identity::FileType::CheckSum, vec!["txt", "sha256sum", "rpm"]),
    ]);
}

#[derive(Args)]
pub struct CommandAdd {
    #[clap(value_enum)]
    #[arg(help = "specify the file type for signing, currently support checksum and rpm")]
    file_type: sign_identity::FileType,
    #[clap(value_enum)]
    #[arg(help = "specify the key type for signing, currently support pgp and x509")]
    key_type: sign_identity::KeyType,
    #[arg(long)]
    #[arg(help = "specify the path which will be used for signing file and directory are supported")]
    path: String,
    #[arg(long)]
    #[arg(help = "specify the key id for signing")]
    key_id: String,
}


#[derive(Clone)]
pub struct CommandAddHandler {
    worker_threads: usize,
    working_dir: String,
    file_type: sign_identity::FileType,
    key_type: sign_identity::KeyType,
    key_id: String,
    path: PathBuf,
    buffer_size: usize,
    signal: Arc<AtomicBool>,
    config:  Arc<RwLock<Config>>
}

impl CommandAddHandler {
    fn collect_file_candidates(&self) -> Result<Vec<sign_identity::SignIdentity>> {
        if self.path.is_dir() {
            let mut container = Vec::new();
            for entry in walkdir::WalkDir::new(self.path.to_str().unwrap()) {
                match entry {
                    Ok(en)=> {
                        if en.metadata()?.is_dir() {
                            continue
                        }
                        if let Some(extension) = en.path().extension() {
                            if self.file_candidates(extension.to_str().unwrap())? {
                                container.push(
                                    sign_identity::SignIdentity::new(
                                        self.file_type.clone(),
                                        en.path().to_path_buf(),
                                        self.key_type.clone(), self.key_id.clone())
                                );
                            }
                        }
                    },
                    Err(err)=> {
                        error!("failed to scan file {}, will be skipped", err);
                    }
                }
            }
            return Ok(container);
        } else {
            if self.file_candidates(self.path.extension().unwrap().to_str().unwrap())? {
                return Ok(vec![sign_identity::SignIdentity::new(
                    self.file_type.clone(), self.path.clone(), self.key_type.clone(), self.key_id.clone())]);
            }
        }
        return Err(error::Error::NoFileCandidateError);
    }

    fn file_candidates(&self, extension: &str) -> Result<bool> {
        let collections = FILE_EXTENSION.get(
            &self.file_type).ok_or(
            error::Error::FileNotSupportError(format!("{}", self.file_type)))?;
        if collections.contains(&extension) {
            return Ok(true)
        }
        Ok(false)
    }
}


impl SignCommand for CommandAddHandler {
    type CommandValue = CommandAdd;

    fn new(signal: Arc<AtomicBool>, config: Arc<RwLock<Config>>, command: Self::CommandValue) -> Result<Self> {
        let mut worker_threads = config.read()?.get_string("worker_threads")?.parse()?;
        if worker_threads == 0 {
            worker_threads = num_cpus::get() as usize;
        }
        Ok(CommandAddHandler{
            worker_threads,
            buffer_size: config.read()?.get_string("buffer_size")?.parse()?,
            working_dir: config.read()?.get_string("working_dir")?,
            file_type: command.file_type,
            key_type: command.key_type,
            key_id: command.key_id,
            path: std::path::PathBuf::from(&command.path),
            signal,
            config: config.clone(),
        })
    }

    fn validate(&self) -> Result<bool> {
        //considering the valid key type for specific file type
        Ok(true)
    }

    //Signing process are described below.
    //1. fetch all file candidates by walk through the specified path and filter by file extension.
    //2. split files via file handler
    //3. send split content to signer handler which will do remote sign internally
    //4. send encrypted content to file handler for assemble
    //5. collect sign result and print
    //6. wait for async task finish
    //7. all of the worker will not *raise* error but record error inside of object
    //            vector                sign_chn                      assemble_chn             collect_chn
    //  fetcher-----------splitter * N----------remote signer * N---------------assembler * N--------------collector * N
    fn handle(&self) -> Result<()> {
        let files = self.collect_file_candidates()?;
        let runtime = runtime::Builder::new_multi_thread()
            .worker_threads(self.worker_threads)
            .enable_io()
            .enable_time()
            .build().unwrap();
        let (sign_s, sign_r) = bounded::<sign_identity::SignIdentity>(MAX_MESSAGES);
        let (assemble_s, assemble_r) = bounded::<sign_identity::SignIdentity>(MAX_MESSAGES);
        let (collect_s, collect_r) = bounded::<sign_identity::SignIdentity>(MAX_MESSAGES);
        info!("starting to sign {} files", files.len());
        let lb_config = self.config.read()?.get_table("server")?;
        runtime.block_on(async {
            let channel = ChannelFactory::new(
                &lb_config).await.unwrap().get_channel().unwrap();
            let mut signer = RemoteSigner::new(channel, self.buffer_size);
            //split file
            let split_handlers = files.into_iter().map(|file|{
                let task_sign_s = sign_s.clone();
                tokio::spawn(async move {
                    info!("starting to sign file: {}", file.file_path.as_path().display());
                    let mut splitter = Splitter::new();
                    splitter.handle(file, task_sign_s).await;
                })
            }).collect::<Vec<_>>();
            //do remote sign
            let task_assemble_s = assemble_s.clone();
            let sign_handler = tokio::spawn(async move {
                loop {
                    let sign_identity = sign_r.recv().await;
                    match sign_identity {
                        Ok(identity) => {
                            signer.handle(identity, task_assemble_s.clone()).await;
                        },
                        Err(e) => {
                            info!("sign channel closed");
                            return
                        }
                    }
                }
            });
            //assemble file
            let working_dir = self.working_dir.clone();
            let task_collect_s = collect_s.clone();
            let assemble_handler = tokio::spawn(async move {
                loop {
                    let sign_identity = assemble_r.recv().await;
                    match sign_identity {
                        Ok(identity) => {
                            let mut assembler = Assembler::new( working_dir.clone());
                            assembler.handle(identity, task_collect_s.clone()).await;
                        },
                        Err(e) => {
                            info!("assemble channel closed");
                            return
                        }
                    }
                }
            });
            // collect result
            let collect_handler = tokio::spawn(async move {
                loop {
                    let sign_identity = collect_r.recv().await;
                    match sign_identity {
                        Ok(identity) => {
                            if identity.error.borrow().clone().is_err() {
                                error!("failed to sign file {} due to error {:?}",
                                    identity.file_path.as_path().display(),
                                    identity.error.borrow().clone().err())
                            } else {
                                info!("successfully signed file {}", identity.file_path.as_path().display())
                            }
                        },
                        Err(e) => {
                            info!("collect channel closed");
                            return
                        }
                    }
                }
            });
            // wait for finish
            for h in split_handlers {
                h.await.unwrap();
            }
            drop(sign_s);
            sign_handler.await.expect("sign worker finished correctly");
            drop(assemble_s);
            assemble_handler.await.expect("assemble worker finished correctly");
            drop(collect_s);
            collect_handler.await.expect("collect worker finished correctly");
            info!("sign files process finished");
        });
        Ok(())
    }
}
