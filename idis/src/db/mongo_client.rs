use async_trait::async_trait;
use futures::StreamExt;
use serde_json::{Value};
use ruid_set::ruid::Ruid;
use mongodb::{options::ClientOptions, Client};
use mongodb::bson::{self, bson, doc, Bson, Document};

use std::str::FromStr;

// use super::db_trait::Database;
use super::query::{self, FeatureQuery, Index, InsertQuery, LocationQuery, QueryType};

pub struct MongoDB {
    instance: mongodb::Database,
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
 
    pub fn feature_query_to_mongo_while(query: &FeatureQuery) -> Document {
        enum Type {
            None,
            And,
            Not,
            Or,
        }
    
        // スタック構造を準備 (現在のクエリと出力用の文書を保持)
        let mut stack_2d: Vec<(Vec<(bson::Bson, Option<FeatureQuery>)>, Type, usize)> = Vec::new();
        let mut compleat_doc: Option<bson::Document> = None;
    
        // 初期化
        stack_2d.push((
            vec![(bson::bson!({}), Some(query.clone()))],
            Type::None,
            0,
        ));
    
        while let Some(mut stack) = stack_2d.pop() {
            match stack.1 {
                Type::And => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリ（compleat_doc）を現在のスタックにマージ
                    if let Some(cdoc) = compleat_doc.take() {
                        let marge_doc = &mut stack.0.get_mut(now_index).unwrap().0;
                        *marge_doc = bson::Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次のクエリが存在する場合は、新しいスタックを作成して処理を続ける
                    if let Some(cqp) = stack.0.get(now_index) {
                        let cvec: (Vec<(bson::Bson, Option<FeatureQuery>)>, Type, usize) =
                            (vec![(bson::bson!(Option::<i32>::None), cqp.1.clone())], Type::None, 0);
                        stack.2 = now_index;
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // すべてのクエリが処理済みの場合、$and ドキュメントを作成
                        let mut and_doc = bson::Document::new();
                        let mut and_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let bson::Bson::Document(doc) = bson_doc {
                                and_array.push(bson::Bson::Document(doc));
                            }
                        }
                        and_doc.insert("$and", bson::Bson::Array(and_array));
                        compleat_doc = Some(and_doc);
                    }
    
                },
                Type::Or => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリをマージ
                    if let Some(cdoc) = compleat_doc.take() {
                        let marge_doc = &mut stack.0.get_mut(now_index).unwrap().0;
                        *marge_doc = bson::Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次があれば続行
                    if let Some(cqp) = stack.0.get(now_index) {
                        let cvec: (Vec<(bson::Bson, Option<FeatureQuery>)>, Type, usize) =
                            (vec![(bson::bson!(Option::<i32>::None), cqp.1.clone())], Type::None, 0);
                        stack.2 = now_index;
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // すべて処理済みで$or作成
                        let mut or_doc = bson::Document::new();
                        let mut or_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let bson::Bson::Document(doc) = bson_doc {
                                or_array.push(bson::Bson::Document(doc));
                            }
                        }
                        or_doc.insert("$or", bson::Bson::Array(or_array));
                        compleat_doc = Some(or_doc);
                    }
                },
                Type::Not => {
                    let mut now_index = stack.2;
                    // 直前の完成したクエリをマージ
                    if let Some(cdoc) = compleat_doc.take() {
                        let marge_doc = &mut stack.0.get_mut(now_index).unwrap().0;
                        *marge_doc = bson::Bson::Document(cdoc);
                        now_index += 1;
                    }
                    // 次があれば続行
                    if let Some(cqp) = stack.0.get(now_index) {
                        let cvec: (Vec<(bson::Bson, Option<FeatureQuery>)>, Type, usize) =
                            (vec![(bson::bson!(Option::<i32>::None), cqp.1.clone())], Type::None, 0);
                        stack.2 = now_index;
                        stack_2d.push(stack);
                        stack_2d.push(cvec);
                    } else {
                        // 全処理済みで$not作成
                        // 注意: MongoDBでは$notは1条件だけを否定する形で使うのが一般的
                        // 複数条件を$notで包むには本来正規表現や単一条件を使いますが、ここでは配列を$notで包む実装とします。
                        let mut not_doc = bson::Document::new();
                        let mut not_array = Vec::new();
                        for (bson_doc, _) in stack.0 {
                            if let bson::Bson::Document(doc) = bson_doc {
                                not_array.push(bson::Bson::Document(doc));
                            }
                        }
                        not_doc.insert("$not", bson::Bson::Array(not_array));
                        compleat_doc = Some(not_doc);
                    }
                },
                Type::None => {
                    // シンプルな条件処理
                    // stack.0.last().unwrap().1 は現在のクエリ
                    let current_query = stack.0.last().unwrap().1.clone().unwrap();
                    match current_query {
                        FeatureQuery::Any => {
                            // Anyは条件なしを表すと仮定し、空ドキュメントを返す
                            compleat_doc = Some(bson::Document::new());
                        },
                        FeatureQuery::None => {
                            // Noneは常にマッチしない条件とするなら、$expr: falseなどを使う
                            let mut doc = bson::Document::new();
                            doc.insert("$expr", bson::Bson::Boolean(false));
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::Less(value) => {
                            let doc = doc! { "$lte": value };
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::Greater(value) => {
                            let doc = doc! { "$gte": value };
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::MatchNum(value) => {
                            let doc = doc! { "$eq": value };
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::MatchStr(value) => {
                            let doc = doc! { "$eq": value };
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::MatchBool(value) => {
                            let doc = doc! { "$eq": value };
                            compleat_doc = Some(doc);
                        },
                        FeatureQuery::Range(start, end, feature_query) => {
                            // 範囲条件を作成
                            let doc = doc! {
                                "$gte": start,
                                "$lte": end
                            };
                            // サブクエリがある場合はスタックに積む
                            let cvec = vec![(bson::Bson::Document(doc), Some(*feature_query))];
                            stack_2d.push((cvec, Type::None, 0));
                        },
                        FeatureQuery::Index(i, feature_query) => {
                            // インデックス指定。仮に"array.i"形式でアクセスするとする
                            let field = format!("array.{}", i);
                            let doc = doc! { field: {} };
                            let cvec = vec![(bson::Bson::Document(doc), Some(*feature_query))];
                            stack_2d.push((cvec, Type::None, 0));
                        },
                        FeatureQuery::IndexBack(i, feature_query) => {
                            // 末尾からのインデックス "array.-i" と仮定
                            let field = format!("array.-{}", i);
                            let doc = doc! { field: {} };
                            let cvec = vec![(bson::Bson::Document(doc), Some(*feature_query))];
                            stack_2d.push((cvec, Type::None, 0));
                        },
                        FeatureQuery::Nested(index, feature_query) => {
                            let field = match index {
                                Index::Number(num) => num.to_string(),
                                Index::String(s) => s,
                            };
                            let doc = doc! { field: {} };
                            let cvec = vec![(bson::Bson::Document(doc), Some(*feature_query))];
                            stack_2d.push((cvec, Type::None, 0));
                        },
                        FeatureQuery::And(vec) => {
                            let mut c_vec: Vec<(bson::Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson::bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::And, 0));
                        },
                        FeatureQuery::Or(vec) => {
                            let mut c_vec: Vec<(bson::Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson::bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::Or, 0));
                        },
                        FeatureQuery::Not(vec) => {
                            let mut c_vec: Vec<(bson::Bson, Option<FeatureQuery>)> = Vec::new();
                            for q in vec {
                                c_vec.push((bson::bson!({}), Some(q)));
                            }
                            stack_2d.push((c_vec, Type::Not, 0));
                        },
                    }
                },
            }
        }
    
        compleat_doc.unwrap_or_else(bson::Document::new)
    }


    
    
}

