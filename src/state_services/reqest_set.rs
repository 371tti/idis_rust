use chrono::Utc;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::{json, Value};

use super::user_agent_set::UserAgent;

pub struct Request {
    pub path: String,
    pub method: String,
    pub url_query: Value,
    pub user_agent: UserAgent,
    pub referer: Option<String>,
    pub content_type: Option<String>,
    pub accept: Value,
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
            accept: json!([]),
            timestamp: utc_timestamp,
        }
    }
}

impl Serialize for Request {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 10)?; // Updated to 10 fields
        state.serialize_field("path", &self.path)?;
        state.serialize_field("method", &self.method)?;
        state.serialize_field("url_query", &self.url_query)?;
        state.serialize_field("user_agent", &self.user_agent)?;
        state.serialize_field("referer", &self.referer)?;
        state.serialize_field("content_type", &self.content_type)?;
        state.serialize_field("accept", &self.accept)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("version", &"1.0.0")?; // Add version field
        state.serialize_field("type", &10)?; // Add type field
        state.end()
    }
}