// sec/utils/json_f.rs

use serde_json::json;
use chrono::Utc;

pub fn err(proses_num: u32, code: u32, message: &str) -> serde_json::Value {
    let utc_timestamp = Utc::now().timestamp_millis();

    json!({
        "version": 0,
        "type": 0,
        "success": false,
        "code": code,
        "message": message,
        "proses_num": proses_num,
        "UTC": utc_timestamp
    })
}

pub fn success(proses_num: u32, code: u32, message: &str) -> serde_json::Value {
    let utc_timestamp = Utc::now().timestamp_millis();

    json!({
        "version": 0,
        "type": 1,
        "success": true,
        "code": code,
        "message": message,
        "proses_num": proses_num,
        "UTC": utc_timestamp
    })
}
