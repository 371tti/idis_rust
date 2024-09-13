
use std::collections::HashMap;

use crate::sys::init::AppConfig;

use super::api::mongo_client::MongoClient;


pub struct UserData {
    user_name: String,
    user_id: String,
    account_level: i32,
    password: String,

}

pub struct User {
    users: HashMap<u128, Option<UserData>>,
    id_to_ruid: HashMap<String, u128>,
    db: MongoClient,
}

impl User {
    pub fn new(app_config: &AppConfig, db: &MongoClient) -> Self {
        Self {
            users: HashMap::new(),
            id_to_ruid: HashMap::new(),
            db: db.clone(),
        }
    }


}
