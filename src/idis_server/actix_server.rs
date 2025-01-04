use std::sync::Arc;

use actix_web::{dev::Server, middleware::{self}, web, App, HttpServer};
use log::error;


use crate::{actix_middleware::status_page, config::ServerConfig, server::server_trait::WkServer, share::collection::Collection, utils};

use super::actix_server_config::ServiceConfig;

pub struct IndexServer {
    pub config: ServerConfig<ServiceConfig>,
    pub share: Arc<Collection>,
}

impl IndexServer {
    pub fn new(config: ServerConfig<ServiceConfig>, share: Arc<Collection>) -> Self {
        Self {
            config: config,
            share: share,
        }
    }
    
    pub fn create_server(&self) -> Result<Server, std::io::Error> {
        let server_name = self.server_name().to_string();
        let share_clone = web::Data::new(Arc::clone(&self.share));
        let server = HttpServer::new(move || {
            let custom_logger = utils::logger::custom_actix_logger(&server_name);
            App::new()
                .app_data(share_clone.clone())
                .wrap(custom_logger)
                .wrap(middleware::ErrorHandlers::new().default_handler(status_page::middleware::Handler::err_handler))
                // 他のミドルウェアやデータをここに追加可能
        })
        .bind(self.config.server_bind.clone())?
        .workers(self.config.server_workers)
        .backlog(self.config.server_backlog)
        .run();

        Ok(server)
    }   
}

impl WkServer<ServiceConfig> for IndexServer {
    fn config(&self) -> &ServerConfig<ServiceConfig> {
        &self.config
    }

    fn create_server(&self) -> Result<Server, std::io::Error> {
        IndexServer::create_server(self)
    }

    fn server_name(&self) -> &str {
        "IDIS_SERVER"
    }

    fn failed_report(&mut self, e: std::io::Error, failure_count: u32, start_time: tokio::time::Instant) {
        error!("{} failed to start. Error: {}. Failure count: {}. Elapsed time: {:?}", self.server_name(), e, failure_count, start_time.elapsed());
    }  
}