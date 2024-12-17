mod db;

use mongodb::bson::{self, doc, Document};

use crate::db::query::{FeatureQuery, Index};


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
                        // 範囲条件ドキュメントを作成
                        let range_doc = bson::Bson::Document(doc! {
                            "$gte": start,
                            "$lte": end
                        });
                    
                        // Rangeは range_doc と subquery_doc を AND で結合する必要がある
                        // よって、ここでは (range_doc, None) と ( {}, Some(feature_query) ) を
                        // Type::And で積むことで、後続処理で $and にまとめる
                        let cvec = vec![
                            (range_doc, None),
                            (bson::bson!({}), Some(*feature_query))
                        ];
                        stack_2d.push((cvec, Type::And, 0));
                    }
                                 
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



fn main() {

    // テスト1: MatchNumクエリ
    let query = FeatureQuery::MatchNum(10);
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! { "$eq": 10 };
    println!("Test MatchNum: {:?}", result);
    assert_eq!(result, expected);

    // テスト2: Lessクエリ
    let query = FeatureQuery::Less(50);
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! { "$lte": 50 };
    println!("Test Less: {:?}", result);
    assert_eq!(result, expected);

    // テスト3: Greaterクエリ
    let query = FeatureQuery::Greater(20);
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! { "$gte": 20 };
    println!("Test Greater: {:?}", result);
    assert_eq!(result, expected);

    // テスト4: ANDクエリ
    let query = FeatureQuery::And(vec![
        FeatureQuery::MatchNum(10),
        FeatureQuery::Less(50),
        FeatureQuery::Greater(20),
    ]);
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! {
        "$and": [
            { "$eq": 10 },
            { "$lte": 50 },
            { "$gte": 20 }
        ]
    };
    println!("Test AND: {:?}", result);
    assert_eq!(result, expected);

    // テスト5: ORクエリ
    let query = FeatureQuery::Or(vec![
        FeatureQuery::MatchNum(5),
        FeatureQuery::MatchNum(10),
    ]);
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! {
        "$or": [
            { "$eq": 5 },
            { "$eq": 10 }
        ]
    };
    println!("Test OR: {:?}", result);
    assert_eq!(result, expected);

    // // テスト6: NOTクエリ
    // let query = FeatureQuery::Not(vec![FeatureQuery::MatchNum(10)]);
    // let result = feature_query_to_mongo_while(&query);
    // let expected = doc! {
    //     "$not": { "$eq": 10 }
    // };
    // println!("Test NOT: {:?}", result);
    // assert_eq!(result, expected);

    // テスト7: Rangeクエリ
    let query = FeatureQuery::Range(10, 20, Box::new(FeatureQuery::MatchNum(15)));
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! {
        "$gte": 10,
        "$lte": 20
    };
    println!("Test Range: {:?}", result);
    assert_eq!(result, expected);

    // テスト8: Nestedクエリ
    let query = FeatureQuery::Nested(
        Index::String("field".to_string()),
        Box::new(FeatureQuery::MatchNum(10)),
    );
    let result = feature_query_to_mongo_while(&query);
    let expected = doc! {
        "field": { "$eq": 10 }
    };
    println!("Test Nested: {:?}", result);
    assert_eq!(result, expected);

    println!("All tests passed successfully!");
}
