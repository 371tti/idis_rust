use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_with::{serde_as, DisplayFromStr};
use super::user_agent_set::UserAgent;
use crate::utils::custom_serializers_adapters::TimeStamp;

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub url_query: Value,
    pub user_agent: UserAgent,
    pub referer: Option<String>,
    pub content_type: Option<String>,
    pub accept: Option<String>,
    #[serde_as(as = "TimeStamp")]
    pub timestamp: i64,
}

impl Request {
    pub fn new() -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        Self {
            path: "".to_string(),
            method: "".to_string(),
            url_query: json!({}),
            user_agent: UserAgent::new(None, None, None, None, None, None, None),
            referer: None,
            content_type: None,
            accept: None,
            timestamp: utc_timestamp,
        }
    }
}