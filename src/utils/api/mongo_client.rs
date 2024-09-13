use mongodb::{Client, Database, bson, bson::{doc, Document, Bson, oid::ObjectId}};
use mongodb::options::{ClientOptions, FindOneOptions};
use serde_json::Value;
use tokio;
use crate::sys::init::AppConfig;
// use crate::utils::ruid::Ruid;
// use crate::utils::api::cache::Cache;
use std::{error::Error, sync::{Arc, Mutex}};

pub struct MongoClient {
    db: Arc<Mutex<Database>>,
    // cache: Cache
}

impl MongoClient {
    pub async fn new(app_config: &AppConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db_addr = &app_config.mongoDB_addr;
        let db_name = &app_config.mongoDB_name;

        let client_options = ClientOptions::parse(db_addr).await?;
        let client = Client::with_options(client_options)?;
        let db = Arc::new(Mutex::new(client.database(db_name)));
        Ok(Self {
            db: db,
        })
    }

    pub async fn d_new(&self, collection: &str, data: &Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let coll = self.db.lock().expect("db: Exclusive lock release failed").collection::<Document>(collection);
        let bson_data = bson::to_document(data)?;
        let result = coll.insert_one(bson_data, None).await?;  // 修正: None を削除
        if let Bson::ObjectId(oid) = result.inserted_id {
            Ok(oid.to_hex())
        } else {
            Err("Inserted ID is not an ObjectId".into())
        }
    }

    pub async fn d_add(&self, collection: &str, query: impl Into<Document> + Clone, data: &Value) -> Result<Option<ObjectId>, Box<dyn Error + Send + Sync>> {
        let coll = self.db.lock().expect("db: Exclusive lock release failed").collection::<Document>(collection);
        let bson_data = bson::to_document(data)?;
        let did = self.d_fud(collection, query.clone()).await?;
        if let Some(id) = did {
            let update_result = coll.update_one(doc! { "_id": id }, doc! { "$set": bson_data }, None).await?;  // 修正: None を削除
            if update_result.matched_count > 0 {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    pub async fn d_rem(&self, collection: &str, query: impl Into<Document>, data: Option<Vec<&str>>) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let coll = self.db.lock().expect("db: Exclusive lock release failed").collection::<Document>(collection);
        let did = self.d_fud(collection, query).await?;
        if let Some(id) = did {
            if data.is_none() {
                let delete_result = coll.delete_one(doc! { "_id": id }, None).await?;  // 修正: None を削除
                return Ok(delete_result.deleted_count > 0);
            } else {
                let unset_fields: Document = data.unwrap().iter().map(|key| (key.to_string(), Bson::Int32(1))).collect();
                let update_result = coll.update_one(doc! { "_id": id }, doc! { "$unset": unset_fields }, None).await?;  // 修正: None を削除
                return Ok(update_result.modified_count > 0);
            }
        }
        Ok(false)
    }

    pub async fn d_get(&self, collection: &str, query: impl Into<Document>, data: Option<Vec<&str>>) -> Result<Option<Document>, Box<dyn Error + Send + Sync>> {
        let coll = self.db.lock().expect("db: Exclusive lock release failed").collection::<Document>(collection);
        let did = self.d_fud(collection, query).await?;
        if let Some(id) = did {
            if data.is_none() {
                let result = coll.find_one(doc! { "_id": id }, None).await?;  // 修正: None を削除
                return Ok(result);
            } else {
                let fields: Document = data.unwrap().iter().map(|key| (key.to_string(), Bson::Int32(1))).collect();
                let options = FindOneOptions::builder().projection(fields).build();
                let result = coll.find_one(doc! { "_id": id }, Some(options)).await?;  // ここはオプションを渡す
                return Ok(result);
            }
        }
        Ok(None)
    }

    pub async fn d_fud(&self, collection: &str, query: impl Into<Document>) -> Result<Option<ObjectId>, Box<dyn Error + Send + Sync>> {
        let coll = self.db.lock().expect("db: Exclusive lock release failed").collection::<Document>(collection);
        let options = FindOneOptions::builder().projection(doc! { "_id": 1 }).build();
        if let Some(result) = coll.find_one(query.into(), Some(options)).await? {
            if let Ok(id) = result.get_object_id("_id") {
                return Ok(Some(id.clone()));
            }
        }
        Ok(None)
    }
    
}

 // 手動でCloneトレイトを実装
impl Clone for MongoClient {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),  // Arcのクローン
        }
    }
}