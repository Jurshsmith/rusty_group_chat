use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let server_config = ServerConfig::get();

    tracing::info!(
        "Starting server at ws://{}:{}/",
        server_config.host,
        server_config.port
    );

    HttpServer::new(move || App::new().wrap(Logger::default()))
        .workers(2)
        .bind((server_config.host, server_config.port))?
        .run()
        .await
}

///////////////////////////////////
//    SERVER_CONFIG BOUNDARY    //
/////////////////////////////////
use dotenvy::dotenv;

#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get() -> Self {
        dotenv().ok();

        let server_host = std::env::var("SERVER_HOST").unwrap_or(Self::default_host());
        let server_port = std::env::var("SERVER_PORT").unwrap_or(Self::default_port());

        ServerConfig {
            host: server_host,
            port: server_port.parse().unwrap(),
        }
    }

    fn default_host() -> String {
        "127.0.0.1".to_string()
    }

    fn default_port() -> String {
        "4003".to_string()
    }
}
