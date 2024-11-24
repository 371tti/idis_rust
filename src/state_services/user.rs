use std::sync::Arc;
use flurry::HashMap as FlurryHashMap;
use actix_web::web::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::sys::init::AppConfig;
use crate::db_handlers::mongo_client::MongoClient;

use super::err_set::ErrState;
use crate::utils::base64;

use serde_with::{serde_as};
use crate::utils::custom_serializers_adapters::{Hex, Base64};

#[serde_as]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UserData {
    #[serde_as(as = "Hex")]
    pub ruid: u128,
    pub user_id: String,
    pub account_level: i32,
    #[serde_as(as = "Vec<Hex>")]
    pub perm: Vec<u128>,
    #[serde_as(as = "Vec<Base64>")]
    pub active_session: Vec<Vec<u8>>,
    pub latest_access_time: i64, // UTCのミリ秒を格納
}

pub struct User {
    pub users: FlurryHashMap<u128, Arc<UserData>>,
    pub id_to_ruid: FlurryHashMap<String, u128>,
    pub db: Arc<MongoClient>,
    pub user_collection_name: String,
    pub user_data_timeout: i64,
}

impl User {
    pub fn new(app_config: &AppConfig, db: Arc<MongoClient>) -> Self {
        Self {
            users: FlurryHashMap::new(),
            id_to_ruid: FlurryHashMap::new(),
            db,
            user_collection_name: app_config.db_user_collection_name.clone(),
            user_data_timeout: app_config.user_data_timeout,
        }
    }

    pub async fn update_last_access_time(&self, ruid: &u128) -> Result<(), ErrState> {
        let guard = self.users.guard();

        // 現在のUTC時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis();

        // ユーザーデータを取得
        if let Some(user_data) = self.users.get(ruid, &guard) {
            // 新しい UserData を作成し、最新アクセス時間を更新
            let new_user_data = Arc::new(UserData {
                latest_access_time,
                ..(**user_data).clone()
            });

            // 更新されたデータを再度挿入
            self.users.insert(*ruid, new_user_data, &guard);
            Ok(())
        } else {
            Err(ErrState::new(902, "指定された RUID が存在しません".to_string(), None))
        }
    }

    pub async fn set(&self, ruid: &u128, user_id: &str, account_level: &i32, perm: &Vec<u128>, session: &Vec<u8>) -> Result<(), ErrState> {
        let guard = self.users.guard();
        let id_guard = self.id_to_ruid.guard();

        let latest_access_time = Utc::now().timestamp_millis();

        let user_data = Arc::new(UserData {
            ruid: *ruid,
            user_id: user_id.to_string(),
            account_level: *account_level,
            perm: perm.clone(),
            active_session: vec![session.to_vec()],
            latest_access_time,
        });

        self.users.insert(*ruid, user_data.clone(), &guard);

        if *account_level != 0 {
            self.id_to_ruid.insert(user_id.to_string(), *ruid, &id_guard);
        }

        Ok(())
    }

    pub async fn remove(&self, ruid: &u128) -> Result<(), ErrState> {
        let guard = self.users.guard();
        let id_guard = self.id_to_ruid.guard();

        if let Some(user_data) = self.users.remove(ruid, &guard) {
            if user_data.account_level != 0 {
                self.id_to_ruid.remove(&user_data.user_id, &id_guard);
                // データベースから削除
                self.db_remove(ruid).await?;
            }
            Ok(())
        } else {
            Err(ErrState::new(907, "指定された RUID が存在しません".to_string(), None))
        }
    }

    pub async fn get(&self, ruid: &u128) -> Result<Arc<UserData>, ErrState> {
        let guard = self.users.guard();

        if let Some(user_data) = self.users.get(ruid, &guard) {
            return Ok(user_data.clone());
        }

        // データベースから取得
        let user_data = self.db_get(ruid).await?;
        let user_data_arc = Arc::new(user_data);

        // キャッシュに挿入
        self.users.insert(*ruid, user_data_arc.clone(), &guard);

        Ok(user_data_arc)
    }

    pub async fn db_get(&self, ruid: &u128) -> Result<UserData, ErrState> {
        let query = json!({"ruid": ruid});
        let data = match self.db.d_get(&self.user_collection_name, &query, None).await {
            Ok(Some(data)) => data,
            Ok(None) => return Err(ErrState::new(912, "ユーザーデータが見つかりません".to_string(), None)),
            Err(e) => return Err(ErrState::new(911, "ユーザーデータの取得に失敗しました".to_string(), Some(e))),
        };

        let user_data = match serde_json::from_value::<UserData>(data) {
            Ok(user_data) => user_data,
            Err(_) => return Err(ErrState::new(913, "ユーザーデータのデシリアライズに失敗しました".to_string(), None)),
        };

        Ok(user_data)
    }

    pub async fn create(&self, ruid: u128, user_id: &str, account_level: &i32, perm: &Vec<u128>, session: &Vec<u8>) -> Result<(), ErrState> {
        // 現在のUTC時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis();

        let user_data = UserData {
            ruid,
            user_id: user_id.to_string(),
            account_level: *account_level,
            perm: perm.clone(),
            active_session: vec![session.to_vec()], // アクティブセッションを1つだけ設定
            latest_access_time, // 最新のアクセス時間を設定
        };
        self.db_create(&user_data).await?;
        self.set(&ruid, user_id, account_level, perm, session).await?;

        Ok(())
    }

    pub async fn db_create(&self, user_data: &UserData) -> Result<(), ErrState> {
        let data = match serde_json::to_value(user_data) {
            Ok(data) => data,
            Err(_) => return Err(ErrState::new(913, "ユーザーデータのシリアライズに失敗しました".to_string(), None)),
        };

        self.db.d_new(&self.user_collection_name, &data).await.map_err(|e| {
            ErrState::new(914, "ユーザーデータのデータベースへの保存に失敗しました".to_string(), Some(e))
        })?;
        Ok(())
    }

    pub async fn db_save(&self, ruid: &u128) -> Result<(), ErrState> {
        let user_data = self.get(ruid).await?;
        let data = match serde_json::to_value(&*user_data) {
            Ok(data) => data,
            Err(_) => return Err(ErrState::new(915, "ユーザーデータのシリアライズに失敗しました".to_string(), None)),
        };

        let query = json!({"ruid": ruid});
        self.db.d_set(&self.user_collection_name, &query, &data).await.map_err(|e| {
            ErrState::new(916, "ユーザーデータのデータベースへの保存に失敗しました".to_string(), Some(e))
        })?;
        Ok(())
    }

    pub async fn db_remove(&self, ruid: &u128) -> Result<(), ErrState> {
        let query = json!({"ruid": ruid});
        self.db.d_del(&self.user_collection_name, &query).await.map_err(|e| {
            ErrState::new(918, "ユーザーデータのデータベースからの削除に失敗しました".to_string(), Some(e))
        })?;
        Ok(())
    }

    pub async fn tick(&self) -> Result<(), ErrState> {
        let guard = self.users.guard();
        let now = Utc::now().timestamp_millis();
        let timeout = self.user_data_timeout; // 例：3600000ミリ秒（1時間）

        let mut timed_out_ruids = Vec::new();

        // イテレータで RUID を収集
        for ruid in self.users.iter(&guard).map(|(ruid, _)| *ruid) {
            if let Some(user_data) = self.users.get(&ruid, &guard) {
                if user_data.account_level >= 1 && now - user_data.latest_access_time > timeout {
                    timed_out_ruids.push(ruid);
                }
            }
        }

        // タイムアウトしたユーザーを削除
        for ruid in timed_out_ruids {
            self.users.remove(&ruid, &guard);
            self.db_save(&ruid).await?;
        }

        Ok(())
    }
}
