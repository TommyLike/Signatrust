use config::{Config, File};
use std::sync::{Arc, RwLock, atomic::Ordering, atomic::AtomicBool};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::Path;
use std::thread;

pub struct ServerConfig {
    pub config: Arc<RwLock<Config>>,
    path: String,
}

impl ServerConfig {
    pub fn new(path: String) -> ServerConfig {
        let mut config = Config::default();
        config.merge(File::with_name(path.as_str())).unwrap();
        ServerConfig {
            config: Arc::new(RwLock::new(config)),
            path
        }
    }
    pub fn watch(&self, signal: Arc<AtomicBool>) {
        let (tx, rx) = channel();
        let watch_file = self.path.clone();
        let config = self.config.clone();
        let mut watcher: RecommendedWatcher = Watcher::new(
            tx,
            notify::Config::default().with_poll_interval(Duration::from_secs(5)),
        ).unwrap();
        thread::spawn(move || {
            watcher.watch(Path::new(watch_file.as_str()),
                          RecursiveMode::NonRecursive, ).unwrap();
            //TODO: handle signal correctly
            while !signal.load(Ordering::Relaxed) {
                match rx.recv() {
                    Ok(Ok(Event {
                              kind: notify::event::EventKind::Modify(_),
                              ..
                          })) => {
                        info!("server configuration changed ...");
                        config.write().unwrap().refresh().unwrap();
                    }
                    Err(e) => error!("watch error: {:?}", e),
                    _ => {}
                }
            }
            info!("signal received, will quit");
        });
    }
}