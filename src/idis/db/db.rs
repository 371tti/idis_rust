use super::mongo_client::MongoClient;

pub struct DB {
    instance: MongoClient,
    db_addr: String,
    db_name: String,
}

trait DBtrait {
    pub fn async get(&self, )
}

impl DB {
    pub async fn new(db_addr: &str, db_name: &str) -> Self {
        let instance = MongoClient::new().await;
        Self { instance }
    }
}