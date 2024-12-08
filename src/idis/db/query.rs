
use std::{collections::HashMap, hash::Hash, vec};
use mongodb::results;
use ruid_set::ruid::Ruid;
use serde::de::value;
use serde_json::{Map, Value};

use crate::idis::utils::err_set::{ErrState, ErrMsg::ERROR};

/// Indexの種類
#[derive(Debug)]
pub enum Index {
    Number(i32),
    String(String),
}


/// データの場所を指定するクエリ
#[derive(Debug)]
pub enum LocationQuery {
    All,                            // 条件なし（全データ対象） (object, list)
    Key(String),                     // オブジェクトのキーで指定された場所 (object)
    Range(i32, i32),                 // リストの値が範囲内にある (-1, 0 は先頭, 0, -1 は末尾) (list)
    Index(i32),                      // インデックスで指定された場所 (list)
    IndexBack(i32),                  // 末尾からのインデックスで指定された場所 (list)
    // ネストの進行
    Nested(Index, Box<LocationQuery>), // ネストされたフィールドに対するクエリ (object, list)
}

/// データの挿入場所を指定するクエリ
#[derive(Debug)]
pub enum InsertQuery {
    AtHead(i32),                     // リストの先頭に挿入 0 は先頭 [<0>X, <1>X, ...] (list)
    AtBack(i32),                     // リストの末尾に挿入 0 は末尾 [...X<1>, X<0>] (list)
    Push,                            // リストの末尾に挿入もしくは単に挿入 (list, object)
    // ネストの進行
    Nested(Index, Box<InsertQuery>), // ネストされたフィールドの挿入場所 (object, list)
}

/// データの特徴を指定するクエリ
#[derive(Debug)]
pub enum FeatureQuery {
    Less(i32),                        // 数値が指定された値以下 (number)
    Greater(i32),                     // 数値が指定された値以上 (number)
    MatchNum(i32),                    // 値が一致 (number)
    MatchStr(String),           // 値が一致 (String)
    MatchBool(bool),             // 値が一致 (bool)
    // ネストの進行
    Range(i32, i32, Box<FeatureQuery>), // リストの値が範囲内にある (list)
    Nested(Index, Box<FeatureQuery>), // ネストされたフィールドの特徴 (object, list)
    // 論理操作
    And(Vec<FeatureQuery>),          // AND条件 (object, list)
    Or(Vec<FeatureQuery>),           // OR条件 (object, list)
    Not(Box<FeatureQuery>),          // NOT条件 (object, list)
}

/// 操作の種類
#[derive(Debug)]
pub enum QueryType {
    None,                        // 操作なし (object, list)
    Set(Box<Ruid>, Box<Ruid>, LocationQuery),  // 指定された場所に値を設定 (object, list)
    Add(Box<Ruid>, Box<Ruid>, InsertQuery),    // 指定された場所に値を挿入 (object, list)
    Del(Box<Ruid>, Box<Ruid>, LocationQuery),          // 指定された場所のデータを削除 (object, list)
    Get(Box<Ruid>, Box<Ruid>, LocationQuery),          // 指定された場所のデータを取得 (object, list)
    DelMany(Box<Ruid>, FeatureQuery),          // 指定された特徴を持つデータを削除 (object, list)
    Find(Box<Ruid>, FeatureQuery),          // 指定された特徴を持つデータを検索 (object, list)
    List(Box<Ruid>),                        // 全データをリスト取得 (object, list)
}



impl QueryType {
    pub fn new() -> QueryType {
        QueryType::None
    }

    pub fn from_json(json: &Value) -> Result<QueryType, ErrState> {
        if json.is_null() {
            return Ok(QueryType::None);
        }

        let mut flat_map = Vec::new();
        {
            let mut stack: Vec<(Vec<&str>, &Value)> = Vec::new();

            stack.push((vec![], json));

            while let Some((prefix, current)) = stack.pop() {
                match current {
                    Value::Object(map) => {
                        for (key, value) in map {
                            let new_prefix: Vec<&str> = if prefix.is_empty() {
                                key.split('.').collect()
                            } else {
                                prefix.iter().chain(key.split('.').collect::<Vec<&str>>().iter()).cloned().collect()
                            };
                            stack.push((new_prefix, value));
                        }
                    }
                    Value::Bool(b) => {
                        flat_map.push((prefix, *b));
                    }
                    _ => {
                        return Err(ErrState::new(0, None).add_message(ERROR("Invalid JSON format".to_string())));
                    }
                }
            }
        }

        // ここにflat_mapから重複するものや優先するもので 必要ないものを消す処理を書く

        Ok(QueryType::None)
    }


}


//////////
/// 
/// 
/// 


use mongodb::bson::{doc, Bson, Document};

/// LocationQuery を MongoDB クエリに変換
fn location_query_to_mongo(query: &LocationQuery) -> Document {
    match query {
        LocationQuery::All => doc! {}, // 全データ対象
        LocationQuery::Key(key) => doc! { key: { "$exists": true } },
        LocationQuery::Range(start, end) => doc! { "$slice": [start, end - start] },
        LocationQuery::Index(index) => doc! { "$arrayElemAt": ["$array", *index] },
        LocationQuery::IndexBack(index) => doc! { "$arrayElemAt": ["$array", -index] },
        LocationQuery::Nested(index, subquery) => match index {
            Index::Number(num) => {
                let mut sub_doc = location_query_to_mongo(subquery);
                sub_doc.insert("$arrayElemAt", num);
                sub_doc
            }
            Index::String(key) => {
                let mut sub_doc = location_query_to_mongo(subquery);
                sub_doc.insert(key, Bson::Document(sub_doc.clone()));
                sub_doc
            }
        },
    }
}

/// InsertQuery を MongoDB クエリに変換
fn insert_query_to_mongo(query: &InsertQuery) -> Document {
    match query {
        InsertQuery::AtHead(_) => doc! { "$push": { "$each": ["new_value"], "$position": 0 } },
        InsertQuery::AtBack(_) => doc! { "$push": "new_value" },
        InsertQuery::Push => doc! { "$push": "new_value" },
        InsertQuery::Nested(index, subquery) => match index {
            Index::Number(num) => {
                let mut sub_doc = insert_query_to_mongo(subquery);
                sub_doc.insert("$arrayElemAt", num);
                sub_doc
            }
            Index::String(key) => {
                let mut sub_doc = insert_query_to_mongo(subquery);
                sub_doc.insert(key, Bson::Document(sub_doc.clone()));
                sub_doc
            }
        },
    }
}

/// FeatureQuery を MongoDB クエリに変換
fn feature_query_to_mongo(query: &FeatureQuery) -> Document {
    match query {
        FeatureQuery::Less(value) => doc! { "$lt": value },
        FeatureQuery::Greater(value) => doc! { "$gt": value },
        FeatureQuery::MatchNum(value) => doc! { "$eq": value },
        FeatureQuery::MatchStr(value) => doc! { "$eq": value },
        FeatureQuery::MatchBool(value) => doc! { "$eq": value },
        FeatureQuery::Range(start, end, subquery) => {
            let mut sub_doc = feature_query_to_mongo(subquery);
            sub_doc.insert("$gte", start);
            sub_doc.insert("$lte", end);
            sub_doc
        }
        FeatureQuery::Nested(index, subquery) => match index {
            Index::Number(num) => {
                let mut sub_doc = feature_query_to_mongo(subquery);
                sub_doc.insert("$arrayElemAt", num);
                sub_doc
            }
            Index::String(key) => {
                let mut sub_doc = feature_query_to_mongo(subquery);
                sub_doc.insert(key, Bson::Document(sub_doc.clone()));
                sub_doc
            }
        },
        FeatureQuery::And(subqueries) => doc! {
            "$and": subqueries.iter().map(feature_query_to_mongo).collect::<Vec<Document>>()
        },
        FeatureQuery::Or(subqueries) => doc! {
            "$or": subqueries.iter().map(feature_query_to_mongo).collect::<Vec<Document>>()
        },
        FeatureQuery::Not(subquery) => doc! {
            "$not": feature_query_to_mongo(subquery)
        },
    }
}

/// QueryType を MongoDB クエリに変換
fn query_type_to_mongo(query: &QueryType) -> Document {
    match query {
        QueryType::None => doc! {},
        QueryType::Set(_, _, loc_query) => doc! {
            "$set": location_query_to_mongo(loc_query)
        },
        QueryType::Add(_, _, insert_query) => doc! {
            "$addToSet": insert_query_to_mongo(insert_query)
        },
        QueryType::Del(_, _, loc_query) => doc! {
            "$unset": location_query_to_mongo(loc_query)
        },
        QueryType::Get(_, _, loc_query) => location_query_to_mongo(loc_query),
        QueryType::DelMany(_, feat_query) => doc! {
            "$delete": feature_query_to_mongo(feat_query)
        },
        QueryType::Find(_, feat_query) => feature_query_to_mongo(feat_query),
        QueryType::List(_) => doc! {},
    }
}


#[cfg(test)]
mod mongo_query_tests {
    use super::*;
    use mongodb::bson::to_document;

    #[test]
    fn test_location_query_to_mongo() {
        let loc_query = LocationQuery::Nested(
            Index::String("field1".to_string()),
            Box::new(LocationQuery::Range(0, 10)),
        );

        let mongo_query = location_query_to_mongo(&loc_query);
        println!("LocationQuery to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("field1"));
    }

    #[test]
    fn test_insert_query_to_mongo() {
        let insert_query = InsertQuery::Nested(
            Index::String("array_field".to_string()),
            Box::new(InsertQuery::Push),
        );

        let mongo_query = insert_query_to_mongo(&insert_query);
        println!("InsertQuery to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("array_field"));
    }

    #[test]
    fn test_feature_query_to_mongo() {
        let feature_query = FeatureQuery::And(vec![
            FeatureQuery::Greater(10),
            FeatureQuery::Less(100),
        ]);

        let mongo_query = feature_query_to_mongo(&feature_query);
        println!("FeatureQuery to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("$and"));
    }

    #[test]
    fn test_query_type_to_mongo_set() {
        let loc_query = LocationQuery::Key("field_name".to_string());
        let query = QueryType::Set(
            Box::new(Ruid::new_random()),
            Box::new(Ruid::new_random()),
            loc_query,
        );

        let mongo_query = query_type_to_mongo(&query);
        println!("Set QueryType to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("$set"));
    }

    #[test]
    fn test_query_type_to_mongo_add() {
        let insert_query = InsertQuery::AtHead(0);
        let query = QueryType::Add(
            Box::new(Ruid::new_random()),
            Box::new(Ruid::new_random()),
            insert_query,
        );

        let mongo_query = query_type_to_mongo(&query);
        println!("Add QueryType to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("$addToSet"));
    }

    #[test]
    fn test_query_type_to_mongo_find() {
        let feature_query = FeatureQuery::MatchStr("example_value".to_string());
        let query = QueryType::Find(Box::new(Ruid::new_random()), feature_query);

        let mongo_query = query_type_to_mongo(&query);
        println!("Find QueryType to MongoDB: {:?}", mongo_query);
        assert!(mongo_query.contains_key("$eq"));
    }
}
