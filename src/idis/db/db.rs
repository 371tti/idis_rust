use std::fmt::Result;

use ruid_set::ruid::Ruid;
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

    }

    pub async fn get(&self, r: &Ruid, d: &Ruid, key: Option<Vec<&str>>) -> Result<Value, ErrState>{

    }

    pub async fn set(&self, r: &Ruid, d: &Ruid, key: Option<Vec<&str>>, value: Value) -> Result<Value, ErrState>{

    }

    pub async fn del(&self, r: &Ruid, d: &Ruid, key: Option<Vec<&str>>) -> Result<Value, ErrState>{

    }

    pub async fn fnd_one(&self, r: &Ruid, qery: &Value) -> Result<Ruid, ErrState>{

    }

    pub async fn fnd_many(&self, r: &Ruid, qery: &Value) -> Result<Vec<Ruid>, ErrState>{

    }

    pub async fn list(&self, r: &Ruid) -> Result<Vec<Ruid>, ErrState>{

    }
}