use std::{collections::HashMap, fs, io::{Error, ErrorKind}, sync::Arc};
use actix_web::{body::BoxBody, dev::ServiceResponse, middleware::ErrorHandlerResponse, web, HttpResponse};
use chrono::Utc;
use log::{error, info};
use serde::Deserialize;
use serde_json::{self};
use tera::{Context, Tera};

use crate::{config::Configuration, share::collection::{self, Collection}, utils};

#[derive(Clone, Deserialize)]
pub struct StatusMes {
    pub color: String,
    pub message: String,
    pub suggest: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct StatusSet {
    pub status: HashMap<u16, StatusMes>,
}

#[derive(Clone)]
pub struct Handler {
    pub status_set: StatusSet,
    pub template: Tera,
}

impl Handler {
    pub fn new(config: &Configuration) -> Result<Self, Error> {
        let status_json_string = match utils::fs::get_file_string(&config.middleware_config.status_page.status_mes_json_path) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read status message json file: {}", e);
                return Err(e);
            }
        };

        let status_set: HashMap<u16, StatusMes> = match serde_json::from_str(&status_json_string) {
            Ok(status_set) => status_set,
            Err(e) => {
                error!("Failed to parse status message json file: {}", e);
                return Err(Error::new(ErrorKind::Other, e.to_string()));
            }
        };
        info!("loaded status message json");

        let template_string = match utils::fs::get_file_string(&config.middleware_config.status_page.status_page_template_path) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read template file: {}", e);
                return Err(e);
            }
        };
        info!("loaded template");

        let mut template = Tera::default();
        template.add_raw_template("status_page", &template_string)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

        Ok(Handler {
            status_set: StatusSet { status: status_set },
            template,
        })
    }

    pub fn get_status_ms(&self, code: &u16) -> String {
        let status = self.status_set.status.get(code).unwrap();
        status.message.clone()
    }

    pub fn get_status_color(&self, code: &u16) -> String {
        let status = self.status_set.status.get(code).unwrap();
        status.color.clone()
    }

    pub fn get_status_solution(&self, code: &u16) -> Vec<String> {
        let status = self.status_set.status.get(code).unwrap();
        status.suggest.clone()
    }

    pub fn generate_page<B>(&self, res: &ServiceResponse<B>) -> HttpResponse<BoxBody> {
        let status_code = res.status().as_u16();

        let mut debug_info = HashMap::new();
        // Host, Path, Connection, User-Agent, Last-Time, Cf-Connecting-Ip, Accept-Encoding, Accept-Languageなどのヘッダー情報を追加
        debug_info.insert("Host".to_string(),
            res.request().headers().get("Host")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );
        debug_info.insert("Path".to_string(), res.request().path().to_string());
        debug_info.insert("Connection".to_string(),
            res.request().headers().get("Connection")
                .and_then(|c| c.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );
        debug_info.insert("User-Agent".to_string(),
            res.request().headers().get("User-Agent")
                .and_then(|ua| ua.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );
        debug_info.insert("Last-Time".to_string(), Utc::now().to_rfc3339());
        debug_info.insert("Cf-Connecting-Ip".to_string(),
            res.request().headers().get("Cf-Connecting-Ip")
                .and_then(|ip| ip.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );
        debug_info.insert("Accept-Encoding".to_string(),
            res.request().headers().get("Accept-Encoding")
                .and_then(|ae| ae.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );
        debug_info.insert("Accept-Language".to_string(),
            res.request().headers().get("Accept-Language")
                .and_then(|al| al.to_str().ok())
                .unwrap_or("Unknown").to_string()
        );

        let status_message = self.get_status_ms(&status_code);
        let status_color = self.get_status_color(&status_code);
        let suggestion_list = self.get_status_solution(&status_code);

        // Teraコンテキストを作成
        let mut context = Context::new();
        context.insert("code", &status_code.to_string());
        context.insert("ms", &status_message);
        context.insert("color", &status_color);
        context.insert("suggestions", &suggestion_list);
        context.insert("debug_info", &debug_info);

        let rendered = self.template.render("status_page", &context)
            .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
            .unwrap_or_else(|e| {
                error!("Failed to render template: {}", e);
                format!("code: {}, message: {}, color: {}, suggestions: {:?}, debug_info: {:?} - Failed to render err template", status_code, status_message, status_color, suggestion_list, debug_info)
            });

        HttpResponse::build(res.status()).body(rendered)
    }
    
    pub fn err_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
        let collection = res.request().app_data::<web::Data<Arc<Collection>>>().unwrap();
        let response = collection.middleware.status_page.generate_page(&res);
        Ok(ErrorHandlerResponse::Response(
            res.into_response(response.map_into_right_body()),
        ))
    }
}
