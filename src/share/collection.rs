use std::sync::Arc;

use crate::{actix_middleware::{self, status_page::middleware}, config::{self, Configuration}};

#[derive(Clone)]
pub struct Collection {
    pub middleware: actix_middleware::handler::CustomMiddleware,
    pub config: config::Configuration,
}

impl Collection {
    pub fn new(config: Configuration) -> Arc<Self> {
        let midware = match actix_middleware::handler::CustomMiddleware::new(&config) {
            Ok(m) => m,
            Err(e) => panic!("Error: {}", e),
        };

        let collection = Self {
            middleware: midware,
            config: config,
        };

        Arc::new(collection)
    }
}