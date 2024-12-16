// sec/utils/json_f.rs

use actix_web::body::BodyLimitExceeded;
use serde::Serialize;
use serde_json::{json, to_value, Value};
use chrono::Utc;

pub fn err(proses_num: u32, code: u32, message: &str) -> Value {
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

pub fn success(proses_num: u32, code: u32, message: &str) -> Value {
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

pub fn db_user_system(account_type: i32, id: &str, name: &str, password: &str, ruid: u128) -> Value {

    json!({
            "account-type": account_type,
            "id": id,
            "name": name,
            "password": password,
            "RUID": ruid,
         })
}

