// src/state_services/analyze.rs
use std::collections::HashMap;
use actix_web::{web, HttpRequest};
use mime_guess::Mime;
use serde_json::{json, Value};
use woothee::parser::Parser;
use super::processor::Processor;
use crate::{state_services::user_agent_set::UserAgent, state_services::err_set::ErrState, utils::base64};  // ErrState をインポート

pub trait Analyze {
    fn analyze_http(&mut self) -> Result<&mut Self, ErrState>;
}

impl Analyze for Processor {
    fn analyze_http(&mut self) -> Result<&mut Self, ErrState> {
        self.state.stage = 1;
        let method = self.req.method().as_str();
        
        let mut lock_this_server = false;
        let mut session_id = None;
        let mut api_key = None;
        let mut referer = None;
        let mut content_type = None;
        let mut accept_type = Vec::new();
        let mut user_agent = UserAgent::new(None, None, None, None, None, None, None);

        // POST, PUT, PATCHリクエストの場合のサーバーロック処理
        if matches!(method, "POST" | "PUT" | "PATCH") {
            if let Some(content_length) = self.req.headers().get("Content-Length")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.parse::<u64>().ok())
            {
                lock_this_server = content_length > self.app_set.config.server_cluster_lock_content_len;
            } else {
                return Err(ErrState::new(101, "Content-Lengthがない".to_string(), None));  // HTTP 400 Bad Request
            }
        }

        // Cookieからsession idを取得
        if let Some(session) = self.req.cookie("session_id") {
            let session_str = session.value();
            if let Ok(session_vec) = base64::decode_base64(session_str) {
                if session_vec.len() == self.app_set.config.session_len_byte {
                    session_id = Some(session_vec);
                }
            }
        }

        // Authorization ヘッダーの解析
        if let Some(auth_header) = self.req.headers().get("Authorization") {
            if let Ok(key) = auth_header.to_str() {
                match base64::decode_base64(key) {
                    Ok(key_vec) => {
                        if key_vec.len() == self.app_set.config.api_key_len_byte {
                            api_key = Some(key_vec);
                        } else {
                            return Err(ErrState::new(102, "APIキーの長さが不正".to_string(), None));  // HTTP 400 Bad Request
                        }
                    }
                    Err(e) => {
                        return Err(ErrState::new(102, "APIキーのデコードに失敗".to_string(), Some(e)));  // HTTP 400 Bad Request
                    }
                }
            } else {
                return Err(ErrState::new(102, "APIキーが不正".to_string(), None));  // HTTP 400 Bad Request
            }
        }

        // その他のヘッダー情報の解析
        let path = self.req.path().to_string();
        referer = self.req.headers().get("Referer")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string());

        // クエリ解析
        let qery = json!(web::Query::<HashMap<String, String>>::from_query(self.req.query_string())
            .unwrap_or_else(|_| web::Query(HashMap::new()))
            .into_inner());

        // User-Agent解析
        let user_agent_str = self.req.headers().get("User-Agent")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("");
        if let Some(ua) = Parser::new().parse(user_agent_str) {
            user_agent = UserAgent::new(
                Some(ua.name.to_string()),
                Some(ua.version.to_string()),
                Some(ua.os.to_string()),
                Some(ua.os_version.to_string()),
                Some(ua.category.to_string()),
                Some(ua.vendor.to_string()),
                Some(ua.browser_type.to_string())
            );
        }

        // Content-Type解析
        content_type = self.req.headers().get("Content-Type")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string());

        // Acceptヘッダー解析
        let server_support_type: Vec<Mime> = self.app_set.config.server_supported_content_types.clone();
        let accept_header = self.req.headers().get("Accept")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("*/*");
        
        let mut accept_map: Vec<(Mime, f32)> = Vec::new();
        for item in accept_header.split(',') {
            let parts: Vec<&str> = item.split(';').collect();
            let mime_str = parts[0].trim();
            let quality = parts.iter()
                .find(|p| p.starts_with("q="))
                .and_then(|q| q[2..].parse::<f32>().ok())
                .unwrap_or(1.0);
            if let Ok(mime) = mime_str.parse::<Mime>() {
                accept_map.push((mime, quality));
            }
        }

        // サポートされているタイプをフィルタリング
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

        accept_type = matching_types.into_iter()
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
        self.state.session_id = session_id;
        self.state.api_key = api_key;
        self.lock_this_server = lock_this_server;

        Ok(self)
    }
}
