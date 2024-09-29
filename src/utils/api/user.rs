use std::sync::Mutex;
use std::collections::HashMap;

use actix_web::web::Json;
use chrono::Utc;
use serde_json::json;

use crate::sys::init::AppConfig;
use crate::utils::api::mongo_client::MongoClient;

#[derive(Clone, Default)]
pub struct UserData {
    pub user_id: String,
    pub account_level: i32,
    pub perm: Vec<u128>,
    pub latest_access_time: u64, // UTCのミリ秒を格納
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

    pub fn update_last_access_time(&self, ruid: &u128) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock on users data.".to_string())?;

        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis() as u64;

        // 指定された RUID が存在する場合に `latest_access_time` を更新
        if let Some(user_data_option) = users.get_mut(ruid) {
            if let Some(user_data) = user_data_option {
                user_data.latest_access_time = latest_access_time;
            }
        }

        Ok(())
    }

    pub fn set(&self, ruid: &u128, user_id: &str, account_level: &i32, perm: &Vec<u128>) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock on users data.".to_string())?;
        let mut id_to_ruid = self.id_to_ruid.lock().map_err(|_| "Failed to acquire lock on id_to_ruid data.".to_string())?;
        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis() as u64;

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

    pub fn remove(&self, ruid: &u128) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock on users data.".to_string())?;
        let mut id_to_ruid = self.id_to_ruid.lock().map_err(|_| "Failed to acquire lock on id_to_ruid data.".to_string())?;
    
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
            Err("User not found.".to_string())
        }
    }

    pub fn get(&self, ruid: &u128) -> Result<UserData, String> {
        // ロックを取得
        let users = self.users.lock().map_err(|_| "Failed to acquire lock on users data.".to_string())?;

        // ユーザーが存在すればそれを返し、存在しなければデフォルト値を返す
        if let Some(user_data_option) = users.get(ruid) {
            if let Some(user_data) = user_data_option {
                return Ok(user_data.clone()); // クローンを返す
            }
        }
        
        // ユーザーセッションが解放されている場合
        Ok(UserData::default()) // 存在しない場合はデフォルト値を返す
    }
    

}
