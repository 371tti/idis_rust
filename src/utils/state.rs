// sec/utils/state.rs

use actix_web::HttpResponse;
use serde_json::Value;

use super::ruid::Ruid;

pub struct  State {
    pub userRUID: Ruid,
    pub user: i32, // アカウントレベル
    pub perm: Vec<u128>,
    pub err: Option<Value>,
    pub responce: Option<HttpResponse>,
}

