use std::{collections::{HashMap, HashSet}, error::Error};

use actix_web::{cookie::{time::format_description::well_known::iso8601::Config, Cookie}, dev::Response, http::{Method, StatusCode}, web::{self, Json}, HttpRequest, HttpResponse};

use base64::engine::general_purpose;
use base64::Engine;
use mime_guess::Mime;
use serde_json::{json, Value};

use woothee::parser::Parser;

use crate::{
    utils::{api::user::{self, UserData}, json_f, state::State},
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

    
    pub async fn analyze(&mut self, req: HttpRequest) -> &mut Self {
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
    




    pub async fn session_check(&mut self) -> &mut Self {
        self.state.stage = 2;
        let result: Result<(), Box<dyn Error>> = (|| {
        if let Some(session_id_str) = self.state.cookies.get("session_id").and_then(|v| v.as_str()) {
            println!("{}", session_id_str);
            // session_id_strをVecに変換
            if let Ok(session_vec) = self.app.session.base64_to_vec(&session_id_str) {
                // session_vec の長さを確認
                if self.app.config.session_len_byte == session_vec.len() {
                    // ユーザーセッションが存在するか確認
                    if let Some(user_ruid) = self.app.session.user(session_vec.clone())? {
                        println!("{}", user_ruid.to_string());
                        // 最終アクセス時間を更新
                        self.app
                            .session
                            .update_last_access_time(session_vec.clone())?;
                        self.app.user.update_last_access_time(&user_ruid)?;
                        // ユーザー情報を取得
                        let user_data = self.app.user.get(&user_ruid)?;
                        // 情報をステートにコピー
                        self.state.user_ruid = format!("{:x}", user_ruid);
                        self.state.session_id = Some(session_id_str.to_string());
                        self.state.user_perm = user_data.perm.iter().map(|&num| format!("{:x}", num)).collect();
                        return Ok(());
                    } else {
                        // ユーザーセッションがない場合
                        println!("waaa");
                    }
                } else {
                    // session_vec の長さが無効な場合の処理
                }
            } else {
            }
        } else {
            // セッションを持ってない
        }

        // セッションを生成
        let new_session_vec = self.app.session.set()?;
        // ユーザーセッションを生成
        let guest_user_ruid = self.app.ruid.generate(self.app.config.ruid_prefix.USER_EXAMPLE_ID, None);
        let everyoune_permission : u128 = (self.app.config.ruid_prefix.USER_EXAMPLE_ID as u128) << 112;
        self.app.session.add(new_session_vec.clone(), guest_user_ruid.to_u128())?;
        self.app.user.set(
            &guest_user_ruid.to_u128(),
            &format!("@guest{}", guest_user_ruid.to_string()),
            &0,
            &vec![everyoune_permission],
        )?;
        // 情報をステートにコピー
        self.state.user_ruid = format!("{:x}", guest_user_ruid.to_u128());
        self.state.session_id = Some(self.app.session.vec_to_base64(new_session_vec));
        let user_data = self.app.user.get(&guest_user_ruid.to_u128())?;
        self.state.user_perm = user_data.perm.iter().map(|&num| format!("{:x}", num)).collect();
        return Ok(());
    })();
    if let Err(e) = result {
        // ロールバック処理
        if let Some(session_id_str) = self.state.session_id.clone() {
            if let Ok(session_vec) = self.app.session.base64_to_vec(&session_id_str) {
                self.app.session.unset(session_vec).unwrap_or_default();
            }
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

    pub fn endpoint(&mut self) {
        match self.state.reqest.path.as_str() {
            "/" => {
                // ルートの処理
            }
            "/debug/state" => {
                // 
            }
            _ => {
                // 404エラーなど
            }
        }
    }

    // リクエストビルダー
    pub async fn build(&mut self) -> HttpResponse {
        self.state.stage = 5;
        let result: Result<HttpResponse, Box<dyn Error>> = (|| async {
            // 非同期処理 (await を使うためにクロージャを async にする)
            let mut response = self.app.json_api.stream(json!(self.state)).send().await;
                    // send の結果が Result であればエラーハンドリング
            if response.status().is_server_error() {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to stream data")) as Box<dyn Error>);
            }

            if let Some(session_id) = &self.state.session_id {
                let cookie = Cookie::build("session_id", session_id)
                    .path("/")
                    .http_only(true)
                    .finish();
                response.add_cookie(&cookie)?;  // クッキーを追加
            }
    
            Ok(response)
        })().await;
    
        // エラーが発生した場合の処理
        match result {
            Ok(response) => response,  // 正常にレスポンスが構築された場合
            Err(e) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("An error occurred while building the response")
            }
        }
    }
}
