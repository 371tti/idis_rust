// src/utils/api/mongo_client.rs

use mongodb::{Client, bson::doc};
use mongodb::options::ClientOptions;
use tokio;
use crate::sys::init::AppConfig;
use crate::utils::ruid::Ruid;


pub struct MongoClient {

}

impl  MongoClient {
    pub fn new(app_config: AppConfig) -> Self{
        MongoClient {

        }
    }

}
