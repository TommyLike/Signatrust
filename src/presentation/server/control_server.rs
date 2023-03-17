use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use actix_web::{App, HttpServer, middleware, web, cookie::Key};
use config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use time::Duration as timeDuration;

use crate::infra::database::model::datakey::repository as datakeyRepository;
use crate::infra::database::pool::{create_pool, get_db_pool};

use crate::presentation::handler::control::*;

use crate::util::error::Result;
use openidconnect::core::{
    CoreClient,
};
use openidconnect::{JsonWebKeySet, ClientId, AuthUrl, UserInfoUrl, TokenUrl, RedirectUrl, ClientSecret, IssuerUrl};
use crate::application::datakey::DBKeyService;
use crate::infra::database::model::token::repository::TokenRepository;
use crate::infra::database::model::user::repository::UserRepository;
use crate::infra::sign_backend::factory::SignBackendFactory;
use crate::application::user::DBUserService;
use crate::domain::sign_service::SignBackend;

pub struct OIDCConfig {
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub user_info_url: String
}

pub struct ControlServer {
    server_config: Arc<RwLock<Config>>,
}

impl ControlServer {
    pub async fn new(server_config: Arc<RwLock<Config>>) -> Result<Self> {
        let database = server_config.read()?.get_table("database")?;
        create_pool(&database).await?;
        let server = ControlServer {
            server_config,
        };
        Ok(server)
    }

    pub fn initialize_oidc_info(&self) -> Result<OIDCConfig> {
        Ok(OIDCConfig{
            client_id: self.server_config.read()?.get_string("oidc.client_id")?,
            client_secret: self.server_config.read()?.get_string("oidc.client_secret")?,
            token_url: self.server_config.read()?.get_string("oidc.token_url")?,
            redirect_uri: self.server_config.read()?.get_string("oidc.redirect_url")?,
            user_info_url: self.server_config.read()?.get_string("oidc.userinfo_url")?,
        })
    }

    pub fn initialize_oidc_client(&self) -> Result<CoreClient> {
        Ok(CoreClient::new(
            ClientId::new(self.server_config.read()?.get_string("oidc.client_id")?),
            Some(ClientSecret::new(self.server_config.read()?.get_string("oidc.client_secret")?)),
            IssuerUrl::new(self.server_config.read()?.get_string("oidc.auth_url")?)?,
            AuthUrl::new(self.server_config.read()?.get_string("oidc.auth_url")?)?,
            Some(TokenUrl::new(self.server_config.read()?.get_string("oidc.token_url")?)?),
            Some(UserInfoUrl::new(self.server_config.read()?.get_string("oidc.userinfo_url")?)?),
            JsonWebKeySet::default()).set_redirect_uri(RedirectUrl::new(self.server_config.read()?.get_string("oidc.redirect_url")?)?,
        ))
    }



    pub async fn run(&self) -> Result<()> {
        //start actix web server
        let addr: SocketAddr = format!(
            "{}:{}",
            self.server_config
                .read()?
                .get_string("control-server.server_ip")?,
            self.server_config
                .read()?
                .get_string("control-server.server_port")?
        )
            .parse()?;

        let key = self.server_config.read()?.get_string("control-server.cookie_key")?;

        //initialize oidc client
        let client = web::Data::new(self.initialize_oidc_client()?);
        //TODO: remove me when openid connect library is ready
        let oidc_config = web::Data::new(self.initialize_oidc_info()?);

        info!("control server starts");
        // Start http server
        let data_repository = datakeyRepository::DataKeyRepository::new(
            get_db_pool()?,
        );
        let sign_backend = SignBackendFactory::new_engine(
            self.server_config.clone(), get_db_pool()?).await?;
        //initialize user repo
        let user_repo = UserRepository::new(get_db_pool()?);

        //initialize token repo
        let token_repo = TokenRepository::new(get_db_pool()?);

        let user_service = web::Data::new(DBUserService::new(user_repo.clone(), token_repo));

        let key_service = web::Data::new(DBKeyService::new(data_repository, sign_backend));

        let http_server = HttpServer::new(move || {
            App::new()
                // enable logger
                .app_data(key_service.clone())
                .app_data(client.clone())
                .app_data(user_service.clone())
                .app_data(oidc_config.clone())
                .wrap(middleware::Logger::default())
                .wrap(IdentityMiddleware::default())
                .wrap(
                    SessionMiddleware::builder(
                        CookieSessionStore::default(), Key::from(key.as_bytes()))
                        .session_lifecycle(PersistentSession::default().session_ttl(timeDuration::hours(1)))
                        .cookie_name("signatrust".to_owned())
                        .cookie_secure(false)
                        .cookie_domain(None)
                        .cookie_path("/".to_owned())
                        .build(),
                )
                .service(web::scope("/api/v1")
                    .service(user_handler::get_scope())
                    .service(datakey_handler::get_scope()))
        });
        if self.server_config
            .read()?
            .get_string("tls_cert")?
            .is_empty()
            || self
            .server_config
            .read()?
            .get_string("tls_key")?
            .is_empty() {
            info!("tls key and cert not configured, control server tls will be disabled");
            http_server.bind(addr)?.run().await?;
        } else {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(
                    self.server_config.read()?.get_string("tls_key")?, SslFiletype::PEM).unwrap();
            builder.set_certificate_chain_file(
                self.server_config.read()?.get_string("tls_cert")?).unwrap();
            http_server.bind_openssl(addr, builder)?.run().await?;
        }
        Ok(())
    }
}
