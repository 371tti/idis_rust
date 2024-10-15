use mongodb::{Client, Database, bson, bson::{doc, Document, Bson, oid::ObjectId}};
use mongodb::options::{ClientOptions, FindOneOptions};
use serde_json::Value;
use std::{error::Error, sync::{Arc, Mutex}};

// ErrStateのインポート
use crate::utils::err_set::ErrState;

use crate::sys::init::AppConfig;

pub struct MongoClient {
    db: Arc<Mutex<Database>>,
}

impl MongoClient {
    pub async fn new(app_config: &AppConfig) -> Result<Self, ErrState> {
        let db_addr = &app_config.mongoDB_addr;
        let db_name = &app_config.mongoDB_name;

        // Initialize MongoDB client options and handle potential errors.
        let client_options = ClientOptions::parse(db_addr).await.map_err(|e| {
            ErrState::new(300, 500) // MongoDBクライアントオプションの初期化に失敗
        })?;
        
        let client = Client::with_options(client_options).map_err(|e| {
            ErrState::new(301, 500) // MongoDBクライアントの作成に失敗
        })?;

        if db_name.is_empty() {
            return Err(ErrState::new(302, 400)); // データベース名が空
        }
        
        let db = client.database(db_name);

        Ok(Self {db:  Arc::new(Mutex::new(db)) })
    }

    pub async fn d_new(&self, collection: &str, data: &Value) -> Result<String, ErrState> {
        let db_lock = self.db.lock().map_err(|e| {
            ErrState::new(303, 500) // データベースロックの取得に失敗
        })?;
        
        let coll = db_lock.collection::<Document>(collection);

        let bson_data = bson::to_document(data).map_err(|e| {
            ErrState::new(304, 400) // データのBSON変換に失敗
        })?;
        
        let result = coll.insert_one(bson_data, None).await.map_err(|e| {
            ErrState::new(305, 500) // ドキュメントの挿入に失敗
        })?;
        
        if let Bson::ObjectId(oid) = result.inserted_id {
            Ok(oid.to_hex())
        } else {
            Err(ErrState::new(306, 500)) // 挿入されたIDがObjectIdではない
        }
    }

    pub async fn d_get(&self, collection: &str, query: impl Into<Document>, fields: Option<Vec<&str>>) -> Result<Option<Document>, ErrState> {
        let db_lock = self.db.lock().map_err(|e| {
            ErrState::new(307, 500) // データベースロックの取得に失敗
        })?;
        
        let coll = db_lock.collection::<Document>(collection);
        
        let projection = fields.map(|field_list| {
            let projection_fields: Document = field_list.iter()
                .map(|key| (key.to_string(), Bson::Int32(1)))
                .collect();
            FindOneOptions::builder().projection(projection_fields).build()
        });

        let result = coll.find_one(query.into(), projection).await.map_err(|e| {
            ErrState::new(308, 500) // ドキュメントの検索に失敗
        })?;
        
        if result.is_none() {
            return Err(ErrState::new(309, 404)); // ドキュメントが見つからない
        }

        Ok(result)
    }

    pub async fn d_fud(&self, collection: &str, query: impl Into<Document>) -> Result<Option<ObjectId>, ErrState> {
        let db_lock = self.db.lock().map_err(|e| {
            ErrState::new(310, 500) // データベースロックの取得に失敗
        })?;
        
        let coll = db_lock.collection::<Document>(collection);
        let options = FindOneOptions::builder().projection(doc! { "_id": 1 }).build();
        
        let result = coll.find_one(query.into(), Some(options)).await.map_err(|e| {
            ErrState::new(311, 500) // ドキュメントの検索に失敗
        })?;
        
        if let Some(doc) = result {
            if let Ok(id) = doc.get_object_id("_id") {
                return Ok(Some(id.clone()));
            }
        } else {
            return Err(ErrState::new(312, 404)); // ドキュメントが見つからない
        }
        
        Ok(None)
    }
}

// 手動でクローン実装
impl Clone for MongoClient {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}