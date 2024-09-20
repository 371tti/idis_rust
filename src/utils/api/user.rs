use std::sync::Mutex;
use std::collections::HashMap;

use chrono::Utc;

use crate::sys::init::AppConfig;
use crate::utils::api::mongo_client::MongoClient;


#[derive(Clone)]
pub struct UserData {
    user_id: String,
    account_level: i32,
    perm: Vec<u128>,
    latest_access_time: u64, // UTCのミリ秒を格納
}

pub struct User {
    users: Mutex<HashMap<u128, Option<UserData>>>,
    id_to_ruid: Mutex<HashMap<String, u128>>,
    db: MongoClient,
}

impl User {
    pub fn new(app_config: &AppConfig, db: &MongoClient) -> Self {
        let mut result = Self {
            users: Mutex::new(HashMap::new()),
            id_to_ruid: Mutex::new(HashMap::new()),
            db: db.clone(),
        };
        result
    }

    pub fn update_last_access_time(&mut self, ruid: &u128) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock on users data:".to_string())?;
        
        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis() as u64;

        // 指定された RUID が存在する場合に `latest_access_time` を更新
        if let Some(user_data) = users.get_mut(ruid) {
            if let Some(user_data) = user_data {
                user_data.latest_access_time = latest_access_time;
                return Ok(());
            }
        }

        // 指定された RUID が存在しない場合
        Err("User not found.".to_string())
    }

    pub fn set(&mut self, ruid: &u128, user_id: &str, account_level: &i32, perm: &Vec<u128>) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().unwrap();
        let mut id_to_ruid = self.id_to_ruid.lock().unwrap();
        
        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis() as u64;

        // データの追加
        users.insert(
            *ruid, 
            Some(UserData {
                user_id: user_id.to_string(),
                account_level: *account_level,
                perm: perm.to_vec(),
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

    pub fn remove(&mut self, ruid: &u128) -> Result<(), String> {
        // ロック取得時のエラーハンドリング
        let mut users = self.users.lock().map_err(|_| "Failed to acquire lock on users data:".to_string())?;
        let mut id_to_ruid = self.id_to_ruid.lock().map_err(|_| "Failed to acquire lock on id_to_ruid data:".to_string())?;
    
        // ユーザーの削除
        if let Some(user_data) = users.remove(ruid) {
            // account_level が 0 でない場合のみ id_to_ruid から削除
            if let Some(user_data) = user_data {
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

    pub fn get(&self, ruid: &u128) -> Option<UserData> {
        let users = self.users.lock().unwrap();

        if let Some(user_data) = users.get(ruid) {
            user_data.clone()
        } else {
            None
        }      
    }



    // pub fn db_create(&self) -> Result<Value, Box<dyn Error>> {
    //     let user_system_data = json_f::db_user_system(0, "@xxxx", "xxxx", "xxxx", 0);
    //     match block_on(self.db.d_new("user", &user_system_data)) {
    //         Ok(_) => Ok(user_system_data),
    //         Err(e) => Err(e),
    //     }
    // }

}
