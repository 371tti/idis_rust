use std::{fs, path::PathBuf};

use actix_web::middleware::Logger;
use serde::Deserialize;
use log::{error, info};

use crate::{actix_middleware::config::MiddlewareConfig, idis_server, utils};

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig<ServiceConfig> {
    pub enable: bool,
    pub server_bind: String,
    pub server_workers: usize,
    pub server_backlog: u32,
    pub restart_on_panic: bool,
    pub max_failures: u32,
    pub failure_count_period_time: u32,
    pub restart_interval: u32,
    pub service_config: ServiceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub idis_server: ServerConfig<idis_server::actix_server_config::ServiceConfig>,
    pub path: String,
    pub logger_mode: String,
    pub middleware_config: MiddlewareConfig,
}

impl Configuration {
    pub fn loader(yaml_path_r: &str) -> Self {
        // バイナリのディレクトリから相対パスでファイルを指定
        let yaml_string = match utils::fs::get_file_string(yaml_path_r) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read config file: {}", e);
                panic!("Failed to read config file");
            }
        };

        let config = match serde_yaml::from_str::<Configuration>(&yaml_string) {
            Ok(config) => config,
            Err(e) => {
                match e.location() {
                    Some(location) => {
                        error!("Syntax error in YAML file at line {}, column {}: {}",
                            location.line(),
                            location.column(),
                            e
                    );
                    }
                    None => {
                        error!("Failed to parse config file: {}", e);
                    }
                    
                }
                panic!("Failed to parse config file");
            }
            
        };
        info!("Config file loaded successfully");
        config
    }
}