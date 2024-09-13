use mongodb::{Client, Database, bson, bson::{doc, Document, Bson, oid::ObjectId}};
use mongodb::options::{ClientOptions, FindOneOptions};
use serde_json::Value;
use std::{error::Error, sync::{Arc, Mutex}};

use crate::sys::init::AppConfig;

pub struct MongoClient {
    db: Arc<Mutex<Database>>,
}

impl MongoClient {
    pub async fn new(app_config: &AppConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let db_addr = &app_config.mongoDB_addr;
        let db_name = &app_config.mongoDB_name;

        // Initialize MongoDB client options and handle potential errors.
        let client_options = ClientOptions::parse(db_addr).await.map_err(|e| {
            format!("Failed to parse DB options: {}", e)
        })?;
        
        let client = Client::with_options(client_options).map_err(|e| {
            format!("Failed to create DB client: {}", e)
        })?;

        if db_name.is_empty() {
            return Err("Database name cannot be empty: None".into());
        }
        
        let db = client.database(db_name);

        Ok(Self {db:  Arc::new(Mutex::new(db)) })
    }

    pub async fn d_new(&self, collection: &str, data: &Value) -> Result<String, Box<dyn Error + Send + Sync>> {
        let db_lock = self.db.lock().map_err(|e| {
            format!("Failed to acquire database lock: {}", e)
        })?;
        
        let coll = db_lock.collection::<Document>(collection);

        let bson_data = bson::to_document(data).map_err(|e| {
            format!("Failed to convert data to BSON: {}", e)
        })?;
        
        let result = coll.insert_one(bson_data, None).await.map_err(|e| {
            format!("Failed to insert document into DB: {}", e)
        })?;
        
        if let Bson::ObjectId(oid) = result.inserted_id {
            Ok(oid.to_hex())
        } else {
            Err("Inserted ID is not an ObjectId".into())
        }
    }

    pub async fn d_get(&self, collection: &str, query: impl Into<Document>, fields: Option<Vec<&str>>) -> Result<Option<Document>, Box<dyn Error + Send + Sync>> {
        let db_lock = self.db.lock().map_err(|e| {
            format!("Failed to acquire database lock: {}", e)
        })?;
        
        let coll = db_lock.collection::<Document>(collection);
        
        let projection = fields.map(|field_list| {
            let projection_fields: Document = field_list.iter()
                .map(|key| (key.to_string(), Bson::Int32(1)))
                .collect();
            FindOneOptions::builder().projection(projection_fields).build()
        });

        let result = coll.find_one(query.into(), projection).await.map_err(|e| {
            format!("Failed to find document in DB: {}", e)
        })?;
        
        Ok(result)
    }

    pub async fn d_fud(&self, collection: &str, query: impl Into<Document>) -> Result<Option<ObjectId>, Box<dyn Error + Send + Sync>> {
        let db_lock = self.db.lock().map_err(|e| {
            format!("Failed to acquire database lock: {}", e)
        })?;
        
        let coll = db_lock.collection::<Document>(collection);
        let options = FindOneOptions::builder().projection(doc! { "_id": 1 }).build();
        
        let result = coll.find_one(query.into(), Some(options)).await.map_err(|e| {
            format!("Failed to find document by ID in DB: {}", e)
        })?;
        
        if let Some(doc) = result {
            if let Ok(id) = doc.get_object_id("_id") {
                return Ok(Some(id.clone()));
            }
        }
        
        Ok(None)
    }
}

// Manually implement Clone for MongoClient
impl Clone for MongoClient {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}
