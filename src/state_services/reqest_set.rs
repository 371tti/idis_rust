use serde::Serialize;
use serde_json::{json, Value};


#[derive(Serialize)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub url_query: Value,
    pub user_agent: Value,
    pub referer: Option<String>,
    pub content_type: Option<String>,
    pub accept: Value,
    pub timestamp: u64,
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
            timestamp: 0,
        }
    }
}