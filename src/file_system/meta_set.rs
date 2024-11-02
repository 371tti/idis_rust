use std::collections::HashMap;

pub struct MetaData {
    pub ruid: u128,
    pub path: String,
    pub data_type: String, //  MIME type | application/folder
    pub size: u64,
    pub create_time: i64,
    pub update_time: i64,
    pub viws: u64,
    pub point: i64,
    pub reaction: HashMap<u128, u128>,
}


