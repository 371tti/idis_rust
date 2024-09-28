use std::{collections::{HashMap, HashSet}, error::Error};

use actix_web::{cookie::time::format_description::well_known::iso8601::Config, http::Method, web::{self, Json}, HttpRequest, HttpResponse};

use base64::engine::general_purpose;
use base64::Engine;
use mime_guess::Mime;
use serde_json::{json, Value};

use woothee::parser::Parser;

use crate::{
    utils::{api::user::UserData, json_f, state::State},
    AppMod,
};

pub struct Processor {
    pub app: web::Data<AppMod>,
    pub state: State,
    pub lock_this_server: bool,
}

impl Processor {
    pub fn new(app: web::Data<AppMod>) -> Self {
        Self {
            app: app,
            state: State::new(),
            lock_this_server: false,
        }
    }

    
    pub fn analyze(&mut self, req: HttpRequest) -> &mut Self {
        self.state.stage = 1;
        let method = req.method().as_str();
    
        // 大きなPOSTリクエストの場合クラスタリングをしないようにlock
        if method == "POST" || method == "PUT" || method == "PATCH" {
            if let Some(content_length) = req.headers().get("Content-Length")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.parse::<u64>().ok())
            {
                self.lock_this_server = content_length > self.app.config.server_cluster_lock_content_len;
            }
        }
    
        // クッキーの解析
        if let Some(session_id) = req.cookie("session_id") {
            if let Ok(session_vec) = self.app.session.base64_to_vec(&session_id.to_string()) {
                self.state.session_id = Some(session_vec);
            } else {
                self.state.session_id = None;
            }
        } else {
            self.state.session_id = None;
        }
    
        // Authorizationヘッダーの解析
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(base64_key) = auth_header.to_str() {
                if let Ok(decoded_key) = general_purpose::STANDARD.decode(base64_key.trim()) {
                    self.state.api_key = Some(decoded_key);
                } else {
                    self.state.api_key = None;  // デコードに失敗した場合
                }
            } else {
                self.state.api_key = None;  // 文字列に変換できなかった場合
            }
        } else {
            self.state.api_key = None;  // Authorizationヘッダーが存在しない場合
        }

        // ヘッダ郡の解析
        let path = req.path().to_string();
        let referer: Option<String> = req.headers()
            .get("Referer")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string());
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
        let server_support_type: Vec<Mime> = self.app.config.server_supported_content_types.clone();
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
        // jsonクエリにリクエストを解析変換
    }
    




    pub fn session_check(&mut self) -> &mut Self {
        let result: Result<(), Box<dyn Error>> = (|| {
        self.state.stage = 2;
        if let Some(session_vec) = self.state.session_id.clone() {
            // session_vec の長さを確認
            if self.app.config.session_len_byte == session_vec.len() {
                // ユーザーセッションが存在するか確認
                if let Some(user_ruid) = self.app.session.user(session_vec.clone())? {
                    // 最終アクセス時間を更新
                    self.app
                        .session
                        .update_last_access_time(session_vec.clone())?;
                    self.app.user.update_last_access_time(&user_ruid)?;
                    // ユーザー情報を取得
                    let user_data = self.app.user.get(&user_ruid)?;
                    // 情報をステートにコピー
                    self.state.user_ruid = format!("{:x}", user_ruid);
                    return Ok(());
                } else {
                    // ユーザーセッションがない場合
                }
            } else {
                // session_vec の長さが無効な場合の処理
            }
        } else {
            // セッションを持ってない
        }

        // セッションを生成
        let new_session_vec = self.app.session.set()?;
        // ユーザーセッションを生成
        let guest_user_ruid = self.app.ruid.generate(0x0000, None);
        self.app.user.set(
            &guest_user_ruid.to_u128(),
            &format!("@guest{}", guest_user_ruid.to_string()),
            &0,
            &vec![0u128],
        )?;
        // 情報をステートにコピー
        self.state.user_ruid = format!("{:x}", guest_user_ruid.to_u128());
        self.state.session_id = Some(new_session_vec);

        return Ok(());
    })();
    if let Err(e) = result {
        // ロールバック処理
        if let Some(session_id) = self.state.session_id.clone() {
            self.app.session.unset(session_id).unwrap_or_default();
        }
        if self.state.user_ruid != "" {
            self.app.user.remove( &u128::from_str_radix(&self.state.user_ruid, 16).unwrap_or(0)).unwrap_or_default();
        }

        self.state.status = 500;

    } else {
    }
    self
    }



    // pub fn auth() {
    //     // 権限確認
    // }

    // pub fn endpoint() {
    //     // 内部エンドポイント郡
    // }

    // pub fn build() {
    //     // リクエストビルダー
    // }


}
