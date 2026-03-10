use tracing::level_filters::LevelFilter;

mod config;
mod db;
mod models;
mod repositories;
mod routes;
mod services;

fn install_rustls_crypto_provider() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
}

fn install_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let _ = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .json()
        .try_init();
}

#[tokio::main]
async fn main() {
    install_rustls_crypto_provider();
    install_tracing();

    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
}
