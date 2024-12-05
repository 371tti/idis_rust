use std::fmt::Result;

use ruid_set::ruid::Ruid;
use serde_json::Value;

use crate::idis::utils::err_set::ErrState;

use super::mongo_client::MongoClient;

pub struct DB {
    instance: MongoClient,
    db_addr: String,
    db_name: String,
}

impl DB {
    pub async fn new(db_addr: &str, db_name: &str) -> Result<Self, ErrState> {
        /*
        DB のインスタンスを作成する
        -> DB
         */
    }

    pub async fn get(&self, r: &Ruid, d: &Ruid, feeld_qerys: &Value) -> Result<Value, ErrState>{
        /*
        r: collection ID, 
        d: document ID, 
        feeld_qery: フィールドのクエリ(1以上可能) Nullの場合は全てのフィールドを取得
        -> json
         */
    }

    pub async fn set(&self, r: &Ruid, d: &Ruid, feeld_qery: &Value, value: Value) -> Result<Ruid, ErrState>{
        /*
        r: collection ID, 
        d: document ID, 
        feeld_qery: フィールドのクエリ(1以下) Nullの場合は新しいドキュメントを作成, 
        value: セットするjson
        -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
         */
    }

    pub async fn del(&self, r: &Ruid, d: &Ruid, feeld_qerys: &Value) -> Result<Ruid, ErrState>{
        /*
        r: collection ID, 
        d: document ID, 
        feeld_qery: フィールドのクエリ(1以上可能) Nullの場合はドキュメントを削除
        -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
         */
    }

    pub async fn del_many(&self, r: &Ruid, qery: &Value) -> Result<Vec<Ruid>, ErrState>{
        /*
        r: collection ID, 
        qery: クエリ
        -> 成功した場合は削除したドキュメントのRuidの配列, 失敗した場合はErrorState
         */
    }

    pub async fn fnd_one(&self, r: &Ruid, qery: &Value) -> Result<Ruid, ErrState>{
        /*
        r: collection ID, 
        qery: クエリ
        -> 成功した場合はそのドキュメントのRuid, 失敗した場合はErrorState
         */
    }

    pub async fn fnd_many(&self, r: &Ruid, qery: &Value) -> Result<Vec<Ruid>, ErrState>{
        /*
        r: collection ID, 
        qery: クエリ
        -> 成功した場合はそのドキュメントのRuidの配列, 失敗した場合はErrorState
         */
    }

    pub async fn list(&self, r: &Ruid) -> Result<Vec<Ruid>, ErrState>{
        /*
        r: collection ID
        -> そのコレクションの全てのドキュメントのRuidの配列
         */
    }
}