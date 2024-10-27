use crate::state_services::err_set::ErrState;
use crate::sys::init::AppConfig;
use mongodb::options::{ClientOptions, FindOneOptions, UpdateOptions};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    Client, Database,
};
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub struct MongoClient {
    db: Arc<Mutex<Database>>,
}

impl MongoClient {
    pub async fn new(app_config: &AppConfig) -> Result<Self, ErrState> {
        let db_addr = &app_config.mongoDB_addr;
        let db_name = &app_config.mongoDB_name;

        let client_options = ClientOptions::parse(db_addr)
            .await
            .map_err(|_| ErrState::new(600, "MongoDBクライアントオプションの初期化に失敗".to_string(), None))?;

        let client = Client::with_options(client_options)
            .map_err(|_| ErrState::new(601, "MongoDBクライアントの作成に失敗".to_string(), None))?;

        if db_name.is_empty() {
            return Err(ErrState::new(602, "データベース名が空".to_string(), None));
        }

        let db = client.database(db_name);

        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    // JSON形式でドキュメントを挿入
    pub async fn d_new(&self, collection: &str, data: &Value) -> Result<String, ErrState> {
        let db_lock = self
            .db
            .lock()
            .map_err(|_| ErrState::new(603, "データベースロックの取得に失敗".to_string(), None))?;

        let coll = db_lock.collection::<Document>(collection);

        // JSON -> BSON 変換
        let bson_data = bson::to_document(data)
            .map_err(|_| ErrState::new(604, "データのBSON変換に失敗".to_string(), None))?;

        let result = coll
            .insert_one(bson_data, None)
            .await
            .map_err(|_| ErrState::new(605, "ドキュメントの挿入に失敗".to_string(), None))?;

        if let Bson::ObjectId(oid) = result.inserted_id {
            Ok(oid.to_hex())
        } else {
            Err(ErrState::new(606, "挿入されたIDがObjectIdではない".to_string(), None))
        }
    }

    // JSON形式でドキュメントを取得
    pub async fn d_get(
        &self,
        collection: &str,
        query: &Value,
        fields: Option<Vec<&str>>,
    ) -> Result<Option<Value>, ErrState> {
        let db_lock = self
            .db
            .lock()
            .map_err(|_| ErrState::new(607, "データベースロックの取得に失敗".to_string(), None))?;

        let coll = db_lock.collection::<Document>(collection);

        // JSONクエリを BSONドキュメント に変換
        let bson_query = bson::to_document(query)
            .map_err(|_| ErrState::new(608, "JSONクエリのBSON変換に失敗".to_string(), None))?;

        let projection = fields.map(|field_list| {
            let projection_fields: Document = field_list
                .iter()
                .map(|key| (key.to_string(), Bson::Int32(1)))
                .collect();
            FindOneOptions::builder()
                .projection(projection_fields)
                .build()
        });

        let result = coll
            .find_one(bson_query, projection)
            .await
            .map_err(|_| ErrState::new(609, "ドキュメントの検索に失敗".to_string(), None))?;

        if let Some(doc) = result {
            // BSON -> JSON 変換
            let json_data = bson::from_document(doc)
                .map_err(|_| ErrState::new(610, "BSONからJSONへの変換に失敗".to_string(), None))?;
            Ok(Some(json_data))
        } else {
            Ok(None)
        }
    }

    // JSON形式でドキュメントの存在確認（ID取得）
    pub async fn d_fud(&self, collection: &str, query: &Value) -> Result<Option<String>, ErrState> {
        let db_lock = self
            .db
            .lock()
            .map_err(|_| ErrState::new(611, "データベースロックの取得に失敗".to_string(), None))?;

        let coll = db_lock.collection::<Document>(collection);

        // JSONクエリを BSONドキュメント に変換
        let bson_query = bson::to_document(query)
            .map_err(|_| ErrState::new(612, "JSONクエリのBSON変換に失敗".to_string(), None))?;

        let options = FindOneOptions::builder()
            .projection(doc! { "_id": 1 })
            .build();

        let result = coll
            .find_one(bson_query, Some(options))
            .await
            .map_err(|_| ErrState::new(613, "ドキュメントの検索に失敗".to_string(), None))?;

        if let Some(doc) = result {
            if let Ok(id) = doc.get_object_id("_id") {
                return Ok(Some(id.to_hex()));
            }
        }

        Ok(None)
    }

    // JSON形式でドキュメントを更新
    pub async fn d_upd(
        &self,
        collection: &str,
        query: &Value,
        update: &Value,
    ) -> Result<u64, ErrState> {
        let db_lock = self
            .db
            .lock()
            .map_err(|_| ErrState::new(614, "データベースロックの取得に失敗".to_string(), None))?;

        let coll = db_lock.collection::<Document>(collection);

        // JSONクエリを BSONドキュメント に変換
        let bson_query = bson::to_document(query)
            .map_err(|_| ErrState::new(615, "JSONクエリのBSON変換に失敗".to_string(), None))?;

        // JSON更新データを BSONドキュメント に変換
        let bson_update = bson::to_document(update)
            .map_err(|_| ErrState::new(616, "JSON更新データのBSON変換に失敗".to_string(), None))?;

        let result = coll
            .update_one(bson_query, bson_update, None)
            .await
            .map_err(|_| ErrState::new(617, "ドキュメントの更新に失敗".to_string(), None))?;

        Ok(result.modified_count)
    }

    // 特定フィールドの部分更新（$set演算子を使用）
    pub async fn d_set(
        &self,
        collection: &str,
        query: &Value,
        update: &Value,
    ) -> Result<u64, ErrState> {
        let db_lock = self.db.lock()
            .map_err(|_| ErrState::new(614, "データベースロックの取得に失敗".to_string(), None))?;
        
        let coll= db_lock.collection::<Document>(collection);

        // JSONクエリを BSON ドキュメントに変換
        let bson_query = bson::to_document(query)
            .map_err(|_| ErrState::new(615, "JSONクエリのBSON変換に失敗".to_string(), None))?;

        // $set演算子を使用して部分更新のBSONドキュメントを作成
        let bson_update = doc! {
            "$set": bson::to_document(update)
                .map_err(|_| ErrState::new(616, "JSON更新データのBSON変換に失敗".to_string(), None))?
        };

        // 更新を実行
        let result = coll.update_one(bson_query, bson_update, None).await
            .map_err(|_| ErrState::new(617, "ドキュメントの更新に失敗".to_string(), None))?;

        Ok(result.modified_count)
    }

    // 特定フィールドを削除（$unset演算子を使用）
    pub async fn d_unset(
        &self,
        collection: &str,
        query: &Value,
        fields: &Value,
    ) -> Result<u64, ErrState> {
        let db_lock = self.db.lock()
            .map_err(|_| ErrState::new(614, "データベースロックの取得に失敗".to_string(), None))?;
        
        let coll= db_lock.collection::<Document>(collection);

        // JSONクエリを BSON ドキュメントに変換
        let bson_query = bson::to_document(query)
            .map_err(|_| ErrState::new(615, "JSONクエリのBSON変換に失敗".to_string(), None))?;

        // $unset演算子を使用して指定フィールドを削除
        let bson_update = doc! {
            "$unset": bson::to_document(fields)
                .map_err(|_| ErrState::new(616, "JSON更新データのBSON変換に失敗".to_string(), None))?
        };

        // 更新を実行
        let result = coll.update_one(bson_query, bson_update, None).await
            .map_err(|_| ErrState::new(617, "ドキュメントの削除に失敗".to_string(), None))?;

        Ok(result.modified_count)
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
