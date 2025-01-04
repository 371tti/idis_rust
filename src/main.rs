use std::io::Error;
use std::sync::Arc;

use config::Configuration;
use server::server_trait::WkServer;
use share::collection::{self, Collection};
use tokio;
use env_logger::Env;

use log::{error, info};

mod idis_server;
mod config;
mod share;
mod actix_middleware;
mod utils;
mod server;

async fn server_start(config: Configuration, collection: Arc<Collection>) -> Result<(), Error> {
    std::env::set_var("RUST_LOG", config.logger_mode);
    env_logger::init();

    let idis_server = idis_server::actix_server::IndexServer::new(config.idis_server, Arc::clone(&collection)).run_with_restart();
    // 追加していくの

    let result = tokio::join!(
        idis_server
    );

    match result {
        (Ok(_),) => {
            info!("All servers have stopped.");
            Ok(())
        }
        (Err(e),) => {
            error!("An error occurred: {}", e);
            Err(e)
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = config::Configuration::loader("config.yaml");
    let collection = collection::Collection::new(config.clone());

    server_start(config, collection).await?;
    Ok(())
}