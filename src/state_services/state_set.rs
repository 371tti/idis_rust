use actix_web::http::uri;
use base64::engine::general_purpose::NO_PAD;
use serde::Serialize;
use serde_json::{json, Value};

use super::reqest_set::Request;



#[derive(Serialize)]
pub struct State {
    pub user_ruid: String, // user id
    pub user_perm: Vec<String>, // user permission list
    pub cookies: Value,
    pub session_id: Option<String>,
    pub api_key: Option<String>,
    pub status: u32, // status like http status code
    pub stage: u32, // 0: instance, 1: session, 2: parsing, 3: auth, 4: processing, 5: build
    pub reqest: Request,
}

impl State {
    pub fn new() -> Self {
        Self {
            user_ruid: "".to_string(),
            user_perm: Vec::new(),
            cookies: json!({}),
            session_id: None,
            api_key: None,
            status: 100,
            stage: 0,
            reqest: Request::new(),
        }
    }
}
