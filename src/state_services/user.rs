use std::fmt;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use actix::fut::future::result;
use actix_web::web::Json;
use chrono::Utc;
use mongodb::change_stream::session;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use serde::ser::{SerializeStruct, Serializer};

use crate::sys::init::AppConfig;
use crate::db_handlers::mongo_client::MongoClient;

use super::err_set::ErrState;
use crate::utils::base64;

#[derive(Clone, Default)]
pub struct UserData {
    pub ruid: u128,
    pub user_id: String,
    pub account_level: i32,
    pub perm: Vec<u128>,
    pub active_session: Vec<Vec<u8>>,
    pub latest_access_time: i64, // UTCのミリ秒を格納
}

// シリアライズの実装
impl Serialize for UserData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("UserData", 7)?;
        state.serialize_field("ruid", &self.ruid)?;
        state.serialize_field("user_id", &self.user_id)?;
        state.serialize_field("account_level", &self.account_level)?;

        // `perm`を16進数文字列に変換してシリアライズ
        let perm_hex: Vec<String> = self.perm.iter().map(|p| format!("{:x}", p)).collect();
        state.serialize_field("perm", &perm_hex)?;

        // `active_session`の各セッションIDをBase64エンコードしてシリアライズ
        let session_ids_base64: Vec<String> = self.active_session.iter()
            .map(|session| base64::encode_base64(session))
            .collect();
        state.serialize_field("active_session", &session_ids_base64)?;

        // `latest_access_time`を16進数文字列に変換してシリアライズ
        state.serialize_field("latest_access_time", &format!("{:x}", self.latest_access_time))?;

        state.end()
    }
}

// デシリアライズの実装
impl<'de> Deserialize<'de> for UserData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field { Ruid, UserId, AccountLevel, Perm, ActiveSession, LatestAccessTime }

        struct UserDataVisitor;

        impl<'de> Visitor<'de> for UserDataVisitor {
            type Value = UserData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct UserData")
            }

            fn visit_map<V>(self, mut map: V) -> Result<UserData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut ruid = None;
                let mut user_id = None;
                let mut account_level = None;
                let mut perm = None;
                let mut active_session = None;
                let mut latest_access_time = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Ruid => {
                            ruid = Some(map.next_value()?);
                        }
                        Field::UserId => {
                            user_id = Some(map.next_value()?);
                        }
                        Field::AccountLevel => {
                            account_level = Some(map.next_value()?);
                        }
                        Field::Perm => {
                            let perm_hex: Vec<String> = map.next_value()?;
                            perm = Some(perm_hex.into_iter()
                                .map(|hex| u128::from_str_radix(&hex, 16).map_err(|_| de::Error::custom("Invalid perm format")))
                                .collect::<Result<Vec<_>, _>>()?);
                        }
                        Field::ActiveSession => {
                            let session_ids_base64: Vec<String> = map.next_value()?;
                            active_session = Some(session_ids_base64.into_iter()
                                .map(|s| base64::decode_base64(&s).map_err(|_| de::Error::custom("Invalid active_session format")))
                                .collect::<Result<Vec<_>, _>>()?);
                        }
                        Field::LatestAccessTime => {
                            let hex_time: String = map.next_value()?;
                            latest_access_time = Some(i64::from_str_radix(&hex_time, 16).map_err(|_| de::Error::custom("Invalid latest_access_time format"))?);
                        }
                    }
                }

                // すべてのフィールドが存在しなければエラーを返す
                Ok(UserData {
                    ruid: ruid.ok_or_else(|| de::Error::custom("Missing ruid"))?,
                    user_id: user_id.ok_or_else(|| de::Error::custom("Missing user_id"))?,
                    account_level: account_level.ok_or_else(|| de::Error::custom("Missing account_level"))?,
                    perm: perm.ok_or_else(|| de::Error::custom("Missing perm"))?,
                    active_session: active_session.ok_or_else(|| de::Error::custom("Missing active_session"))?,
                    latest_access_time: latest_access_time.ok_or_else(|| de::Error::custom("Missing latest_access_time"))?,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "ruid", "user_id", "account_level", "perm", "active_session", "latest_access_time"
        ];
        deserializer.deserialize_struct("UserData", FIELDS, UserDataVisitor)
    }
}

pub struct User {
    pub users: Mutex<HashMap<u128, Option<UserData>>>,
    pub id_to_ruid: Mutex<HashMap<String, u128>>,
    pub db: Arc<MongoClient>,
    pub user_collection_name: String,
}

impl User {
    pub async fn new(app_config: &AppConfig, db: Arc<MongoClient>) -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            id_to_ruid: Mutex::new(HashMap::new()),
            db: db,
            user_collection_name: app_config.db_user_collection_name.clone(),
        }
    }

    pub async fn update_last_access_time(&self, ruid: &u128) -> Result<(), ErrState> {
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
    pub async fn set(&self, ruid: &u128, user_id: &str, account_level: &i32, perm: &Vec<u128>, session: &Vec<u8>) -> Result<(), ErrState> {
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
                ruid: *ruid,
                user_id: user_id.to_string(),
                account_level: *account_level,
                perm: perm.clone(),
                active_session: vec![session.to_vec()], // アクティブセッションを1つだけ設定
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

    pub async fn remove(&self, ruid: &u128) -> Result<(), ErrState> {
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

    pub async fn get(&self, ruid: &u128) -> Result<UserData, ErrState> {
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
    
    pub async fn db_get(&self, ruid: &u128) -> Result<UserData, ErrState> {
        let query = json!({"ruid": ruid});
        let data = match  self.db.d_get(&self.user_collection_name, &query, None).await {
            Ok(Some(data)) => data,
            Ok(None) => return Err(ErrState::new(912, "ユーザーデータが見つかりません".to_string(), None)),
            Err(e) => return Err(ErrState::new(911, "ユーザーデータの取得に失敗".to_string(), Some(e))),
        };

        let user_data = match serde_json::from_value::<UserData>(data) {
            Ok(user_data) => user_data,
            Err(_) => return Err(ErrState::new(913, "ユーザーデータのデシリアライズに失敗".to_string(), None)),
        };

        Ok(user_data)
    }

    pub async fn create(&self, ruid: u128, user_id: &str, account_level: &i32, perm: &Vec<u128>, session: &Vec<u8>) -> Result<(), ErrState> {
        // UTCの現在時刻をミリ秒で取得
        let latest_access_time = Utc::now().timestamp_millis();

        let user_data = UserData {
            ruid,
            user_id: user_id.to_string(),
            account_level: *account_level,
            perm: perm.clone(),
            active_session: vec![session.to_vec()], // アクティブセッションは空で初期化
            latest_access_time: latest_access_time, // 最新のアクセス時間を設定
        };
        self.db_create(&user_data).await?;
        self.set(&ruid, user_id, account_level, perm, session).await?;
        
        Ok(())
    }

    pub async fn db_create(&self, user_data: &UserData) -> Result<(), ErrState> {
        let data = match serde_json::to_value(user_data) {
            Ok(data) => data,
            Err(_) => return Err(ErrState::new(913, "ユーザーデータのシリアライズに失敗".to_string(), None)),
        };

        self.db.d_new(&self.user_collection_name, &data).await.map_err(|e| {
            ErrState::new(914, "ユーザーデータのデータベースへの保存に失敗".to_string(), Some(e))
        })?;
        Ok(())
    }

    pub async fn tick() {

    }
}
