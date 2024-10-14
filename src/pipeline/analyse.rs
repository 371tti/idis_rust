// src/state_services/analyze.rs

use std::collections::HashMap;
use actix_web::{http::Method, web, HttpRequest};
use mime_guess::Mime;
use serde_json::{json, Value};
use woothee::parser::Parser;
use crate::{state_services::state_set::State, AppSet};

use super::processor::Processor;

pub trait Analyze {
    fn analyze_http(&mut self, req: HttpRequest) -> &mut Self;
}


impl Analyze for Processor {
    fn analyze_http(&mut self, req: HttpRequest) -> &mut Self {
        self.state.stage = 1;
        let method = req.method().as_str();
        
        // POST, PUT, PATCHリクエストの場合、コンテンツが大きいとサーバーロック
        if matches!(method, "POST" | "PUT" | "PATCH") {
            if let Some(content_length) = req.headers().get("Content-Length")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.parse::<u64>().ok())
            {
                self.lock_this_server = content_length > self.app_set.config.server_cluster_lock_content_len;
            }
        }

        // クッキー解析
        let mut cookies_map: HashMap<String, String> = HashMap::new();
        if let Ok(cookies) = req.cookies() {
            for cookie in cookies.iter() {
                cookies_map.insert(cookie.name().to_string(), cookie.value().to_string());
            }
        }
        self.state.cookies = json!(cookies_map);

        // Authorizationヘッダーの解析
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(key) = auth_header.to_str() {
                self.state.api_key = Some(key.to_string());
            } else {
                self.state.api_key = None;
            }
        } else {
            self.state.api_key = None;
        }

        // その他のヘッダー情報の解析
        let path = req.path().to_string();
        let referer = req.headers().get("Referer").and_then(|val| val.to_str().ok()).map(|s| s.to_string());
        let qery = json!(web::Query::<HashMap<String, String>>::from_query(req.query_string())
            .unwrap_or_else(|_| web::Query(HashMap::new()))
            .into_inner());
        let user_agent;
        let user_agent_str = req
            .headers()
            .get("User-Agent")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("");
        if let Some(ua) = Parser::new().parse(user_agent_str) {
            user_agent = json!({
                "browser_name": ua.name,
                "browser_version": ua.version,
                "os": ua.os,
                "os_version": ua.os_version,
                "category": ua.category,
                "vendor": ua.vendor,
                "browser_type": ua.browser_type,
            });
        } else {
            user_agent = json!({});
        }
        let content_type = req.headers().get("Content-Type").and_then(|val| val.to_str().ok()).map(|s| s.to_string());
        let server_support_type: Vec<Mime> = self.app_set.config.server_supported_content_types.clone();
        let accept_header = req.headers()
            .get("Accept")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("*/*");
        let mut accept_map: Vec<(Mime, f32)> = Vec::new();
        for item in accept_header.split(',') {
            let parts: Vec<&str> = item.split(';').collect();
            let mime_str = parts[0].trim();
            let quality = parts
                .iter()
                .find(|p| p.starts_with("q="))
                .and_then(|q| q[2..].parse::<f32>().ok())
                .unwrap_or(1.0);
            if let Ok(mime) = mime_str.parse::<Mime>() {
                accept_map.push((mime, quality));
            }
        }
        let mut matching_types: Vec<(Mime, f32)> = Vec::new();
        for (accepted_mime, q_value) in accept_map {
            for supported_mime in &server_support_type {
                if accepted_mime == *supported_mime
                    || (accepted_mime.type_() == "*" && accepted_mime.subtype() == "*")
                    || (accepted_mime.type_() == supported_mime.type_() && accepted_mime.subtype() == "*")
                {
                    if !matching_types.iter().any(|(m, _)| m == supported_mime) {
                        matching_types.push((supported_mime.clone(), q_value));
                    }
                }
            }
        }
        let accept_type: Vec<String> = matching_types
            .into_iter()
            .map(|(mime, _)| mime.to_string())
            .collect();
    
        // リクエストの各種データを保存
        self.state.reqest.method = method.to_string();
        self.state.reqest.path = path;
        self.state.reqest.url_query = qery;
        self.state.reqest.user_agent = user_agent;
        self.state.reqest.content_type = content_type;
        self.state.reqest.referer = referer;
        self.state.reqest.accept = json!(accept_type);
    
        self
    }
}