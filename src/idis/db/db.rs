use serde_json::Value;

use crate::idis::utils::err_set::ErrState;

use super::mongo_client::MongoClient;

pub struct DB {
    instance: MongoClient,
    db_addr: String,
    db_name: String,
}

impl DB {
    pub async fn new(db_addr: &str, db_name: &str) -> Result<Self, ErrState> {
        let instance = MongoClient::new(&db_addr, &db_name).await;
        match instance {
            Ok(instance) => Ok(
                Self {
                    instance,
                    db_addr: db_addr.to_string(),
                    db_name: db_name.to_string(),
                }),
            Err(err) => Err(err),
        }
    }

    pub async fn get(&self, r: u128, d: u128, key: Option<Vec<&str>>) -> Result<Value, ErrState>{

    }

    pub async fn set(&self, r: u128, d: u128, key: Option<Vec<&str>>, value: Value) -> Result<Value, ErrState>{

    }

    pub async fn del(&self, r: u128, d: u128, key: Option<Vec<&str>>) -> Result<Value, ErrState>{

    }

    
    
}