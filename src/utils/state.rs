// sec/utils/state.rs

use super::ruid::Ruid;

pub struct  State {
    pub username: String,
    pub userRUID: Ruid,
    pub userACLV: i32, // アカウントレベル
}