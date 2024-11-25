use crate::{
    authentication::reject_anonymous_users,
    configuration::{DatabaseSettings, Settings},
    email_client::EmailClient,
    routes::{health_check, webhook_handler},
};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::Server, middleware::from_fn, web, App, HttpServer};
use num_cpus;
use secrecy::{ExposeSecret, SecretString};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");

        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;

        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.file_path,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_with(configuration.with_db())
        .await
}
#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);
#[derive(Debug)]
pub struct ApplicationFilePath(pub String);

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
    file_path: String,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let file_path = web::Data::new(ApplicationFilePath(file_path));

    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let governor_conf = GovernorConfigBuilder::default()
        .burst_size(5)
        .finish()
        .unwrap();

    #[allow(deprecated)]
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(Governor::new(&governor_conf))
            .wrap(cors())
            .route("/health_check", web::get().to(health_check))
            .route("/webhook", web::post().to(webhook_handler))
            .service(web::scope("/user").wrap(from_fn(reject_anonymous_users)))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(email_client.clone())
            .app_data(file_path.clone())
    })
    .workers(num_cpus::get())
    .listen(listener)?
    .run();

    Ok(server)
}

fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec!["Content-Type", "Authorization"])
        .supports_credentials()
        .max_age(3600)
}
