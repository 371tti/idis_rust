// use crate::idis::utils::err_set::{ErrState, ErrMsg};
// use futures::TryStreamExt;
// use mongodb::{bson, bson::Document, options::{FindOneOptions, FindOptions}, Database};
// use serde_json::Value;
// use std::sync::Arc;

// pub struct MongoClient {
//     db: Database,
// }

// impl MongoClient {
//     pub async fn new(db_addr: &str, db_name: &str) -> Result<Self, ErrState> {
//         let client = mongodb::Client::with_uri_str(db_addr)
//             .await
//             .map_err(|_| Self::db_err("MongoDBクライアントの作成に失敗"))?;
//         if db_name.is_empty() {
//             return Err(Self::db_err("データベース名が空です"));
//         }
//         Ok(Self {
//             db: client.database(db_name),
//         })
//     }

//     pub async fn insert(&self, collection: &str, data: &Value) -> Result<String, ErrState> {
//         let coll = self.db.collection::<Document>(collection);
//         let bson_data = bson::to_document(data)
//             .map_err(|_| Self::db_err("データのBSON変換に失敗"))?;
//         let result = coll.insert_one(bson_data, None).await
//             .map_err(|_| Self::db_err("ドキュメントの挿入に失敗"))?;
//         if let bson::Bson::ObjectId(oid) = result.inserted_id {
//             Ok(oid.to_hex())
//         } else {
//             Err(Self::db_err("挿入されたIDがObjectIdではありません"))
//         }
//     }

//     pub async fn find_one(
//         &self,
//         collection: &str,
//         query: &Value,
//         fields: Option<Vec<&str>>,
//     ) -> Result<Option<Value>, ErrState> {
//         let coll = self.db.collection::<Document>(collection);
//         let bson_query = bson::to_document(query)
//             .map_err(|_| Self::db_err("クエリのBSON変換に失敗"))?;
//         let projection = fields.map(|field_list| {
//             FindOneOptions::builder()
//                 .projection(Self::build_projection(field_list))
//                 .build()
//         });
//         let result = coll.find_one(bson_query, projection).await
//             .map_err(|_| Self::db_err("ドキュメントの検索に失敗"))?;
//         result
//             .map(|doc| bson::from_document(doc).map_err(|_| Self::db_err("BSONからJSONへの変換に失敗")))
//             .transpose()
//     }

//     pub async fn find_many(
//         &self,
//         collection: &str,
//         query: &Value,
//         fields: Option<Vec<&str>>,
//     ) -> Result<Vec<Value>, ErrState> {
//         let coll = self.db.collection::<Document>(collection);
//         let bson_query = bson::to_document(query)
//             .map_err(|_| Self::db_err("クエリのBSON変換に失敗"))?;
//         let projection = fields.map(|field_list| {
//             FindOptions::builder()
//                 .projection(Self::build_projection(field_list))
//                 .build()
//         });
//         let mut cursor = coll.find(bson_query, projection).await
//             .map_err(|_| Self::db_err("ドキュメントの検索に失敗"))?;
//         let mut results = Vec::new();
//         while let Some(doc) = cursor.try_next().await
//             .map_err(|_| Self::db_err("カーソル操作中にエラーが発生"))?
//         {
//             let json_data = bson::from_document(doc)
//                 .map_err(|_| Self::db_err("BSONからJSONへの変換に失敗"))?;
//             results.push(json_data);
//         }
//         Ok(results)
//     }

//     fn build_projection(fields: Vec<&str>) -> Document {
//         fields
//             .into_iter()
//             .map(|field| (field.to_string(), bson::Bson::Int32(1)))
//             .collect()
//     }

//     fn db_err(message: &str) -> ErrState {
//         ErrState::new(500, None).add_message(ErrMsg::ERROR(message.to_string()))
//     }
// }
