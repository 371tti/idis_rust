use actix_web::http::uri;
use base64::engine::general_purpose::NO_PAD;
use serde::Serialize;
use serde_json::{json, Value};



#[derive(Serialize)]
pub struct State {
    pub user_ruid: u128, // user id
    pub user_perm: Vec<u128>, // user permission list
    pub session_id: Option<Vec<u8>>, // session id
    pub status: u32, // status like http status code
    pub stage: u32, // 0: instance, 1: session, 2: parsing, 3: auth, 4: processing, 5: build
    pub reqest: Request,
}

impl State {
    pub fn new() -> Self {
        Self {
            user_ruid: 0,
            user_perm: Vec::new(),
            session_id: None,
            status: 100,
            stage: 0,
            reqest: Request::new(),
        }
    }

}

#[derive(Serialize)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub url_query: Value,  // Option型として不確定ヘッダ
    pub user_agent: Value,
    pub referer: Option<String>,   // Option<String>で参照を持たない
    pub content_type: Option<String>,
    pub accept: Value,
}

impl Request {
    pub fn new() -> Self {
        Self {
            path: "".to_string(),
            method: "".to_string(),
            url_query: json!({}),
            user_agent: json!({}),
            referer: None,
            content_type: None,
            accept: json!([]),
        }
    }
}