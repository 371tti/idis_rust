use async_trait::async_trait;
use futures::StreamExt;
use serde_json::{Value};
use ruid_set::ruid::Ruid;
use mongodb::{options::ClientOptions, Client};
use mongodb::bson::{self, bson, doc, Bson, Document};

use std::str::FromStr;

use crate::utils::err_set::{ErrMsg, ErrState};

use super::db_trait::Database;
use super::query::{self, FeatureQuery, Index, InsertQuery, LocationQuery, QueryType};

pub struct MongoDB {
    instance: mongodb::Database,
}

#[async_trait]
impl Database for MongoDB {
    /// DB のインスタンスを作成する -> DB
    async fn new(db_addr: &str, db_name: &str) -> Result<Box<dyn Database>, ErrState> {
        let client_options = ClientOptions::parse(db_addr).await.map_err(|_| {
            ErrState::new(600, None)
                .add_message(ErrMsg::ERROR("MongoDBクライアントの初期化に失敗".to_string()))
        })?;
        let client = Client::with_options(client_options).map_err(|_| {
            ErrState::new(601, None)
                .add_message(ErrMsg::ERROR("MongoDBクライアントの作成に失敗".to_string()))
        })?;
        let db = client.database(db_name);
        Ok(Box::new(Self { instance: db }))
    }

    /// get:
    /// r: collection ID,
    /// d: document ID,
    /// feeld_qery: フィールドのクエリ(1以上可能) Nullの場合は全てのフィールドを取得
    /// -> json
    async fn get(&self, r: &Ruid, d: &Ruid, feeld_qerys: &Value) -> Result<Value, ErrState> {
        let collection = r.to_string();
        let filter = doc! { "ruid": d.to_string() };

    // プロジェクションを BSON に変換
    let projection = if feeld_qerys.is_null() {
        // デフォルトで "_id" を除外
        let mut doc = Document::new();
        doc.insert("_id", Bson::Int32(0)); // "_id" を除外
        Some(mongodb::options::FindOneOptions::builder().projection(doc).build())
    } else {
        // BSON ドキュメントへの変換
        let mut doc = bson::to_document(feeld_qerys).map_err(|_| {
            ErrState::new(602, None)
                .add_message(ErrMsg::ERROR("フィールドクエリを BSON ドキュメントに変換できませんでした".to_string()))
        })?;
        
        // デフォルトで "_id" を除外
        doc.insert("_id", Bson::Int32(0)); // "_id" を除外
        
        // プロジェクションオプションを構築
        Some(mongodb::options::FindOneOptions::builder().projection(doc).build())
    };

        let result = self.instance.collection::<Document>(&collection)
            .find_one(filter, projection)
            .await
            .map_err(|_| ErrState::new(602, None).add_message(ErrMsg::ERROR("ドキュメントの取得に失敗".to_string())))?;

        match result {
            Some(doc) => bson::from_document(doc)
                .map_err(|_| ErrState::new(603, None).add_message(ErrMsg::ERROR("BSONからJSONへの変換に失敗".to_string()))),
            None => Err(ErrState::new(404, None).add_message(ErrMsg::ERROR("ドキュメントが見つかりません".to_string()))),
        }
    }

/// set:
/// r: collection ID,
/// d: document ID,
/// feeld_qery: フィールドのクエリ(1以下) Nullの場合は新しいドキュメントを作成,
/// value: セットするjson
/// -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
async fn set(&self, r: &Ruid, d: &Ruid, feeld_qery: &Value, value: &Value) -> Result<Ruid, ErrState> {
    let collection = r.to_string();
    let filter = doc! { "ruid": d.to_string() };

    // `feeld_qery` が Null の場合は新規作成またはアップサート
    if feeld_qery.is_null() {
        let mut doc = bson::to_document(value)
            .map_err(|_| ErrState::new(604, None).add_message(ErrMsg::ERROR("JSONからBSONへの変換に失敗".to_string())))?;
        doc.insert("ruid", Bson::String(d.to_string()));

        self.instance.collection::<Document>(&collection)
            .replace_one(filter, doc, mongodb::options::ReplaceOptions::builder().upsert(true).build())
            .await
            .map_err(|_| ErrState::new(605, None).add_message(ErrMsg::ERROR("ドキュメントの作成に失敗".to_string())))?;
        return Ok(d.clone());
    }

    // `feeld_qery` を検証してフィールド名を取得
    let field_name = {
        let obj = feeld_qery
            .as_object()
            .ok_or_else(|| ErrState::new(610, None).add_message(ErrMsg::ERROR("無効なフィールドクエリです".to_string())))?;

        // 複数フィールドの場合はエラー
        if obj.len() != 1 {
            return Err(ErrState::new(608, None).add_message(ErrMsg::ERROR("フィールドクエリは1つのみ指定してください".to_string())));
        }

        // 最初のキーを取得し検証
        let (key, _) = obj.iter().next().unwrap();
        if key.contains('*') || key.contains('[') || key.contains(']') {
            return Err(ErrState::new(609, None).add_message(ErrMsg::ERROR("範囲クエリや複数フィールド指定は許可されていません".to_string())));
        }

        key.clone()
    };

    // `$set` ドキュメントを作成して更新
    let update_doc = doc! { "$set": { field_name: bson::to_bson(value).unwrap_or(Bson::Null) } };

    self.instance.collection::<Document>(&collection)
        .update_one(filter, update_doc, None)
        .await
        .map_err(|_| ErrState::new(607, None).add_message(ErrMsg::ERROR("フィールドの更新に失敗".to_string())))?;

    Ok(d.clone())
}


    /// del:
    /// r: collection ID,
    /// d: document ID,
    /// feeld_qery: フィールドのクエリ(1以上可能) Nullの場合はドキュメントを削除
    /// -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
    async fn del(&self, r: &Ruid, d: &Ruid, feeld_qerys: &Value) -> Result<Ruid, ErrState> {
        let collection = r.to_string();
        let filter = doc! { "ruid": d.to_string() };

        if feeld_qerys.is_null() {
            // ドキュメントごと削除
            self.instance.collection::<Document>(&collection)
                .delete_one(filter, None)
                .await
                .map_err(|_| ErrState::new(608, None).add_message(ErrMsg::ERROR("ドキュメントの削除に失敗".to_string())))?;
        } else {
            // 特定フィールドの削除
            let fields = feeld_qerys.as_array()
                .ok_or_else(|| ErrState::new(609, None).add_message(ErrMsg::ERROR("フィールドのリストが無効です".to_string())))?;
            let unset_doc: Document = fields.iter()
                .filter_map(|field| field.as_str().map(|f| (f.to_string(), Bson::Null)))
                .collect();

            let update_doc = doc! { "$unset": unset_doc };
            self.instance.collection::<Document>(&collection)
                .update_one(filter, update_doc, None)
                .await
                .map_err(|_| ErrState::new(610, None).add_message(ErrMsg::ERROR("フィールドの削除に失敗".to_string())))?;
        }

        Ok(d.clone())
    }

    /// del_many:
    /// r: collection ID,
    /// qery: クエリ
    /// -> 成功した場合は削除したドキュメントのRuidの配列, 失敗した場合はErrorState
    async fn del_many(&self, r: &Ruid, qery: &Value) -> Result<Vec<Ruid>, ErrState> {
        let collection = r.to_string();
        let filter = bson::to_document(qery)
            .map_err(|_| ErrState::new(611, None).add_message(ErrMsg::ERROR("JSONからBSONへの変換に失敗".to_string())))?;

        let mut cursor = self.instance.collection::<Document>(&collection)
            .find(filter.clone(), None)
            .await
            .map_err(|_| ErrState::new(612, None).add_message(ErrMsg::ERROR("ドキュメントの検索に失敗".to_string())))?;

        let mut deleted_ids = Vec::new();
        while let Some(doc) = cursor.next().await {
            let doc = doc.map_err(|_| ErrState::new(613, None).add_message(ErrMsg::ERROR("カーソル操作中にエラーが発生".to_string())))?;
            if let Ok(id_str) = doc.get_str("ruid") {
                let rid = Ruid::from_str(id_str)
                    .map_err(|_| ErrState::new(614, None).add_message(ErrMsg::ERROR("Ruidのパースに失敗".to_string())))?;
                deleted_ids.push(rid);
            }
        }

        self.instance.collection::<Document>(&collection)
            .delete_many(filter, None)
            .await
            .map_err(|_| ErrState::new(615, None).add_message(ErrMsg::ERROR("複数削除に失敗".to_string())))?;

        Ok(deleted_ids)
    }

    /// fnd_one:
    /// r: collection ID,
    /// qery: クエリ
    /// -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
    async fn fnd_one(&self, r: &Ruid, qery: &Value) -> Result<Ruid, ErrState> {
        let collection = r.to_string();
        let filter = bson::to_document(qery)
            .map_err(|_| ErrState::new(616, None).add_message(ErrMsg::ERROR("JSONからBSONへの変換に失敗".to_string())))?;

        let result = self.instance.collection::<Document>(&collection)
            .find_one(filter, None)
            .await
            .map_err(|_| ErrState::new(617, None).add_message(ErrMsg::ERROR("ドキュメントの検索に失敗".to_string())))?;

        if let Some(doc) = result {
            let id_str = doc.get_str("ruid")
                .map_err(|_| ErrState::new(618, None).add_message(ErrMsg::ERROR("ドキュメントのruid取得に失敗".to_string())))?;
            Ruid::from_str(id_str)
                .map_err(|_| ErrState::new(619, None).add_message(ErrMsg::ERROR("Ruidのパースに失敗".to_string())))
        } else {
            Err(ErrState::new(404, None).add_message(ErrMsg::ERROR("ドキュメントが見つかりません".to_string())))
        }
    }

    /// fnd_many:
    /// r: collection ID,
    /// qery: クエリ
    /// -> 成功した場合はそのドキュメントのRuidの配列, 失敗した場合はErrorState
    async fn fnd_many(&self, r: &Ruid, qery: &Value) -> Result<Vec<Ruid>, ErrState> {
        let collection = r.to_string();
        let filter = bson::to_document(qery)
            .map_err(|_| ErrState::new(620, None).add_message(ErrMsg::ERROR("JSONからBSONへの変換に失敗".to_string())))?;

        let mut cursor = self.instance.collection::<Document>(&collection)
            .find(filter, None)
            .await
            .map_err(|_| ErrState::new(621, None).add_message(ErrMsg::ERROR("複数ドキュメントの検索に失敗".to_string())))?;

        let mut ids = Vec::new();
        while let Some(doc_result) = cursor.next().await {
            let doc = doc_result
                .map_err(|_| ErrState::new(622, None).add_message(ErrMsg::ERROR("カーソル操作中にエラーが発生".to_string())))?;
            if let Ok(id_str) = doc.get_str("ruid") {
                let rid = Ruid::from_str(id_str)
                    .map_err(|_| ErrState::new(623, None).add_message(ErrMsg::ERROR("Ruidのパースに失敗".to_string())))?;
                ids.push(rid);
            }
        }

        Ok(ids)
    }

    /// list:
    /// r: collection ID
    /// -> そのコレクションの全てのドキュメントのRuidの配列
    async fn list(&self, r: &Ruid) -> Result<Vec<Ruid>, ErrState> {
        let collection = r.to_string();

        let mut cursor = self.instance.collection::<Document>(&collection)
            .find(doc! {}, None)
            .await
            .map_err(|_| ErrState::new(624, None).add_message(ErrMsg::ERROR("コレクションリストの取得に失敗".to_string())))?;

        let mut ids = Vec::new();
        while let Some(doc_result) = cursor.next().await {
            let doc = doc_result
                .map_err(|_| ErrState::new(625, None).add_message(ErrMsg::ERROR("カーソル操作中にエラーが発生".to_string())))?;
            if let Ok(id_str) = doc.get_str("ruid") {
                let rid = Ruid::from_str(id_str)
                    .map_err(|_| ErrState::new(626, None).add_message(ErrMsg::ERROR("Ruidのパースに失敗".to_string())))?;
                ids.push(rid);
            }
        }

        Ok(ids)
    }
}

impl MongoDB {
    // pub fn to_mongo_query(query: &QueryType) -> bson::Document {
    //     match query {
    //         QueryType::None => doc! {},
    //         QueryType::Set(r, d, loc_query) => Self::set_query_builder(loc_query),
    //         QueryType::Add(r, d, insert_query) => Self::add_query_builder(insert_query),
    //         QueryType::Del(r, d, loc_query) => Self::del_query_builder(loc_query),
    //         QueryType::Get(r, d, loc_query) => Self::get_query_builder(loc_query),
    //         QueryType::DelMany(r, feat_query) => Self::del_many_query_builder(feat_query),
    //         QueryType::Find(r, feat_query) => Self::find_query_builder(feat_query),
    //         QueryType::List(r) => Self::list_query_builder(r),
    //     }
    // }

    // fn set_query_builder(collection_id: &Ruid, docment_id: &Ruid, q: &LocationQuery, set_docment: bson::Document) -> bson::Document {
    //     let mut query = q.clone();

    //     while  {
            
    //     }
    // }

    // fn add_query_builder(q: &InsertQuery) -> bson::Document {

    // }

    // fn del_query_builder(q: &LocationQuery) -> bson::Document {

    // }

    // fn get_query_builder(q: &LocationQuery) -> bson::Document {

    // }

    // fn del_many_query_builder(q: &FeatureQuery) -> bson::Document {

    // }
    fn feature_query_to_mongo_while(query: &FeatureQuery) -> Document {
        enum Type {
            None,
            And,
            Not,
            Or,
        }
        // スタック構造を準備 (現在のクエリと出力用の文書を保持)
        let mut stack_2d: Vec<(Vec<(Bson, Option<FeatureQuery>)>, Type, usize)> = Vec::new();
        let mut compleat_doc: Option<Document> = None;
        
        stack_2d.push((
            vec![(bson!({}), Some((*query)))],
            Type::None,
            0,
        ));

        while let Some(stack) = stack_2d.pop() {
            match stack.1 {
                Type::And => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリ（compleat_doc）を現在のスタックにマージ
                    if let Some(cdoc) = compleat_doc {
                        let marge_doc = stack.0.get_mut(now_index).unwrap().0;
                        marge_doc = Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次のクエリが存在する場合は、新しいスタックを作成して処理を続ける
                    if let Some(cqp) = stack.0.get(now_index) {
                        let mut cvec: (Vec<(Bson, Option<FeatureQuery>)>, Type, usize) = (vec![(bson!(None), cqp.1)], Type::None, 0);
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // すべてのクエリが処理済みの場合、$and ドキュメントを作成
                        let mut and_doc = Document::new();
                        let mut and_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let Bson::Document(doc) = bson_doc {
                                and_array.push(Bson::Document(doc));
                            }
                        }
                        and_doc.insert("$and", Bson::Array(and_array));
                        compleat_doc = Some(and_doc);
                    }

                },
                Type::Or => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリ（compleat_doc）を現在のスタックにマージ
                    if let Some(cdoc) = compleat_doc {
                        let marge_doc = stack.0.get_mut(now_index).unwrap().0;
                        marge_doc = Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次のクエリが存在する場合は、新しいスタックを作成して処理を続ける
                    if let Some(cqp) = stack.0.get(now_index) {
                        let mut cvec: (Vec<(Bson, Option<FeatureQuery>)>, Type, usize) = (vec![(bson!(None), cqp.1)], Type::None, 0);
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // すべてのクエリが処理済みの場合、$or ドキュメントを作成
                        let mut or_doc = Document::new();
                        let mut or_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let Bson::Document(doc) = bson_doc {
                                or_array.push(Bson::Document(doc));
                            }
                        }
                        or_doc.insert("$or", Bson::Array(or_array));
                        compleat_doc = Some(or_doc);
                    }
                },
                Type::Not => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリ（compleat_doc）を現在のスタックにマージ
                    if let Some(cdoc) = compleat_doc {
                        let marge_doc = stack.0.get_mut(now_index).unwrap().0;
                        marge_doc = Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次のクエリが存在する場合は、新しいスタックを作成して処理を続ける
                    if let Some(cqp) = stack.0.get(now_index) {
                        let mut cvec: (Vec<(Bson, Option<FeatureQuery>)>, Type, usize) = (vec![(bson!(None), cqp.1)], Type::None, 0);
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // すべてのクエリが処理済みの場合、$not ドキュメントを作成
                        let mut not_doc = Document::new();
                        let mut not_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let Bson::Document(doc) = bson_doc {
                                not_array.push(Bson::Document(doc));
                            }
                        }
                        not_doc.insert("$not", Bson::Array(not_array));
                        compleat_doc = Some(not_doc);
                    }
                },
                Type::None => {
                    match q {
                        FeatureQuery::Any => todo!(),
                        FeatureQuery::None => todo!(),
                        FeatureQuery::Less(_) => todo!(),
                        FeatureQuery::Greater(_) => todo!(),
                        FeatureQuery::MatchNum(_) => todo!(),
                        FeatureQuery::MatchStr(_) => todo!(),
                        FeatureQuery::MatchBool(_) => todo!(),
                        FeatureQuery::Range(_, _, feature_query) => todo!(),
                        FeatureQuery::Index(_, feature_query) => todo!(),
                        FeatureQuery::IndexBack(_, feature_query) => todo!(),
                        FeatureQuery::Nested(index, feature_query) => todo!(),
                        FeatureQuery::And(vec) => {
                            let mut c_vec: Vec<(Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::And, vec.len()));
                        },
                        FeatureQuery::Or(vec) => {
                            let mut c_vec: Vec<(Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::Or, vec.len()));
                        },
                        FeatureQuery::Not(vec) => {
                            let mut c_vec: Vec<(Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::Not, vec.len()));
                        },
                    }
                },
            }

        }

        compleat_doc.unwrap_or(Document::new())
    }

}

