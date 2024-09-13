// src/utils/api/mongo_client.rs

use mongodb::{Client, bson::doc};
use mongodb::options::ClientOptions;
use serde::de::IntoDeserializer;
use tokio;
use crate::sys::init::AppConfig;
use crate::utils::ruid::Ruid;

use crate::utils::api::cache::Cache;


pub struct MongoClient {
    db: Database,
    // cache: Cache
    
}

impl  MongoClient {
    pub fn new(app_config: AppConfig) -> Self{
    pub async fn new(app_config: &AppConfig) -> Self{
        let db_addr = app_config.mongoDB_addr;
        let db_name = app_config.mongoDB_name.clone().into();

        let client_option = ClientOptions::parse(db_addr).await?;
        let client = Client::with_options(client_option)?;
        let db = client.database(db_name);
        Ok(Self {
            db: db,
        })
    }

    pub async fn d_new(&self, collection: &str, data: &Value) -> Result<String, Box<dyn Error>> {
        let coll = self.db.collection::<Document>(collection);
        let bson_data: Document = bson::from_bson(bson::Bson::from(data.clone()))?;
        let result = coll.insert_one(bson_data, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap().to_hex())
    }

    // ドキュメントをJSONで更新
    pub async fn d_add(&self, collection: &str, query: impl Into<Document>, data: &Value) -> Result<Option<ObjectId>, Box<dyn Error>> {
        let coll = self.db.collection::<Document>(collection);
        let bson_data: Document = bson::from_bson(bson::Bson::from(data.clone()))?;
        let did = self.d_fud(collection, query.clone()).await?;
        if let Some(id) = did {
            let update_result = coll.update_one(doc! { "_id": id }, doc! { "$set": bson_data }, None).await?;
            if update_result.matched_count > 0 {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    // ドキュメントをJSONで削除、指定フィールドのみ削除も可能
    pub async fn d_rem(&self, collection: &str, query: impl Into<Document>, data: Option<Vec<&str>>) -> Result<bool, Box<dyn Error>> {
        let coll = self.db.collection::<Document>(collection);
        let did = self.d_fud(collection, query).await?;
        if let Some(id) = did {
            if data.is_none() {
                let delete_result = coll.delete_one(doc! { "_id": id }, None).await?;
                return Ok(delete_result.deleted_count > 0);
            } else {
                let unset_fields: Document = data.unwrap().iter().map(|key| (key.to_string(), "")).collect();
                let update_result = coll.update_one(doc! { "_id": id }, doc! { "$unset": unset_fields }, None).await?;
                return Ok(update_result.modified_count > 0);
            }
        }
        Ok(false)
    }

    // ドキュメントをJSONで取得、全項目または特定のフィールド
    pub async fn d_get(&self, collection: &str, query: impl Into<Document>, data: Option<Vec<&str>>) -> Result<Option<Document>, Box<dyn Error>> {
        let coll = self.db.collection::<Document>(collection);
        let did = self.d_fud(collection, query).await?;
        if let Some(id) = did {
            if data.is_none() {
                return coll.find_one(doc! { "_id": id }, None).await;
            } else {
                let fields: Document = data.unwrap().iter().map(|key| (key.to_string(), 1)).collect();
                return coll.find_one(doc! { "_id": id }, Some(mongodb::options::FindOneOptions::builder().projection(fields).build())).await;
            }
        }
        Ok(None)
    }

    // ドキュメントIDの取得
    pub async fn d_fud(&self, collection: &str, query: impl Into<Document>) -> Result<Option<ObjectId>, Box<dyn Error>> {
        let coll = self.db.collection::<Document>(collection);
        if let Some(result) = coll.find_one(query.into(), Some(mongodb::options::FindOneOptions::builder().projection(doc! { "_id": 1 }).build())).await? {
            if let Some(id) = result.get_object_id("_id").ok() {
                return Ok(Some(*id));
            }
        }
        Ok(None)
    }
}