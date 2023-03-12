mod app_router;
mod app_server_config;
mod app_state;
mod handlers;

use app_router::AppRouter;
use app_server_config::AppServerConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_group_chat_web=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppServerConfig::get();

    tracing::info!("Join group chat at http://{}:{}/", config.host, config.port);

    axum::Server::bind(&config.socket_address())
        .serve(AppRouter::new().into_make_service())
        .await
        .unwrap();
}
