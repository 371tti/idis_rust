use std::collections::HashMap;

use super::perm_set::Perm;

pub struct MetaData {
    pub ruid: u128,
    pub name: String,
    pub path: String,
    pub data_type: String, //  MIME type | application/folder
    pub size: u64,
    pub links: Vec<u128> ,
    pub create_time: i64,
    pub update_time: i64,
    pub last_access_time: i64,
    pub viws: u64,
    pub reaction: HashMap<u128, u128>,
    pub description: String,
    pub perm: Perm,
    pub icon: u128,
}


impl MetaData {
    pub fn new(ruid: u128, name: String, path: String, data_type: String, size: u64, create_time: i64, update_time: i64, last_access_time: i64, viws: u64, reaction: HashMap<u128, u128>, description: String, perm: Perm, icon: u128) -> Self {
        Self {
            ruid,
            name,
            path,
            data_type,
            size,
            links: Vec::new(),
            create_time,
            update_time,
            last_access_time,
            viws,
            reaction,
            description,
            perm,
            icon,
        }
    }
}