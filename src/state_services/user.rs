use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::web::Json;
use chrono::Utc;
use serde_json::json;

use crate::sys::init::AppConfig;
use crate::db_handlers::mongo_client::MongoClient;

use super::err_set::ErrState;

#[derive(Clone, Default)]
pub struct UserData {
    pub user_id: String,
    pub account_level: i32,
    pub perm: Vec<u128>,
    pub latest_access_time: i64, // UTCのミリ秒を格納
}

pub struct User {
    pub users: Mutex<HashMap<u128, Option<UserData>>>,
    pub id_to_ruid: Mutex<HashMap<String, u128>>,
    pub db: MongoClient,
}

impl User {
    pub fn new(app_config: &AppConfig, db: &MongoClient) -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            id_to_ruid: Mutex::new(HashMap::new()),
            db: db.clone(),
        }
    }

    pub fn update_last_access_time(&self, ruid: &u128) -> Result<(), ErrState> {
        // ロック取得時のエラーハンドリング
        let mut users = match self.users.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(900, "ユーザーデータのロック取得に失敗".to_string(), None)),
        };

        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis();

        // 指定された RUID が存在する場合に `latest_access_time` を更新
        if let Some(user_data_option) = users.get_mut(ruid) {
            if let Some(user_data) = user_data_option {
            user_data.latest_access_time = latest_access_time;
            } else {
            return Err(ErrState::new(901, "ユーザーデータが存在しません".to_string(), None));
            }
        } else {
            return Err(ErrState::new(902, "指定されたRUIDが存在しません".to_string(), None));
        }

        Ok(())
    }

    pub fn set(&self, ruid: &u128, user_id: &str, account_level: &i32, perm: &Vec<u128>) -> Result<(), ErrState> {
        // ロック取得時のエラーハンドリング
        let mut users = match self.users.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(903, "ユーザーデータのロック取得に失敗".to_string(), None)),
        };
        let mut id_to_ruid = match self.id_to_ruid.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(904, "ユーザーIDディクショナリのロック取得に失敗".to_string(), None)),
        };
        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis();

        // データの追加
        users.insert(
            *ruid, 
            Some(UserData {
                user_id: user_id.to_string(),
                account_level: *account_level,
                perm: perm.clone(),
                latest_access_time, // 最新のアクセス時間を設定
            })
        );

        // account_level が 0 でない場合のみ id_to_ruid に登録
        if *account_level != 0 {
            id_to_ruid.insert(user_id.to_string(), *ruid);
        }

        // 正常終了
        Ok(())
    }

    pub fn remove(&self, ruid: &u128) -> Result<(), ErrState> {
        // ロック取得時のエラーハンドリング
        let mut users = match self.users.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(905, "ユーザーデータのロック取得に失敗".to_string(), None)),
        };
        let mut id_to_ruid = match self.id_to_ruid.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(906, "ユーザーIDディクショナリのロック取得に失敗".to_string(), None)),
        };

        // ユーザーの削除
        if let Some(user_data_option) = users.remove(ruid) {
            // account_level が 0 でない場合のみ id_to_ruid から削除
            if let Some(user_data) = user_data_option {
                if user_data.account_level != 0 {
                    id_to_ruid.remove(&user_data.user_id);
                }
            }
            // 削除成功
            Ok(())
        } else {
            // 指定された RUID が存在しない場合
            Err(ErrState::new(907, "指定されたRUIDが存在しません".to_string(), None))
        }
    }

    pub fn get(&self, ruid: &u128) -> Result<UserData, ErrState> {
        // ロックを取得
        let users = match self.users.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(ErrState::new(908, "ユーザーデータのロック取得に失敗".to_string(), None)),
        };

        // ユーザーが存在すればそれを返し、存在しなければエラーを返す
        if let Some(user_data_option) = users.get(ruid) {
            if let Some(user_data) = user_data_option {
                return Ok(user_data.clone()); // クローンを返す
            } else {
                return Err(ErrState::new(909, "ユーザーデータが存在しません".to_string(), None));
            }
        } else {
            return Err(ErrState::new(910, "指定されたRUIDが存在しません".to_string(), None));
        }
    }
    

}
