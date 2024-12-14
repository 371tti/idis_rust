use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MiddlewareConfig {
    
    pub status_page: super::status_page::config::Config,
}