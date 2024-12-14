use std::io::Error;

use crate::config::Configuration;

use super::status_page;

#[derive(Clone)]
pub struct CustomMiddleware {
    pub status_page: status_page::middleware::Handler,
}

impl CustomMiddleware {
    pub fn new(config: &Configuration) -> Result<Self, Error> {
        let status_page = match status_page::middleware::Handler::new(config) {
            Ok(h) => h,
            Err(e) => return Err(e),
        };

        Ok(Self {
            status_page: status_page,
        })
    }
}