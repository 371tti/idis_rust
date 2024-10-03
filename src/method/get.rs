use actix_web::App;

use crate::{sys::init::AppConfig, utils::api::json_api::JsonApi, AppMod};

pub struct Get {
    pub app_config: AppConfig,
}

impl Get {
    pub fn new(app_config: &AppConfig) -> Self {
        Self {
            app_config: app_config.clone(),
        }
    }

    pub fn generate() -> {

    }
    
}