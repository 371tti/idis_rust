use std::{collections::HashMap, sync::{RwLock, Mutex}};
use chrono::{Duration, Utc};
use rand_chacha::ChaCha20Rng;
use rand::RngCore;
use base64::{engine::general_purpose, DecodeError, Engine as _};
use crate::sys::init::AppConfig;
use super::err_set::ErrState;

#[derive(Clone)]
pub struct SessionData {
    pub last_access_time: i64,
    pub generated_time: i64,
    pub access_count: u64,
    pub users: Vec<u128>,
}

pub struct Session {
    pub sessions: RwLock<HashMap<Vec<u8>, SessionData>>,  // RwLockで読み取り・書き込みを管理
    pub len: usize,
    pub life_time_server: Duration,
    pub life_time_client: Duration,
    pub rng: Mutex<ChaCha20Rng>,  // RNGは単純な排他アクセスで管理
}

impl Session {
    // コンストラクタ
    pub fn new(app_config: &AppConfig) -> Self {
        Session {
            sessions: RwLock::new(HashMap::with_capacity(app_config.session_default_capacity)),
            len: app_config.session_len_byte,
            life_time_server: app_config.session_life_time_server,
            life_time_client: app_config.session_life_time_client,
            rng: Mutex::new(app_config.session_rng.clone()),
        }
    }

    // 最終アクセス時間を更新
    pub async fn update_last_access_time(&self, session_vec: Vec<u8>) -> Result<(), ErrState> {
        let latest_access_time = Utc::now().timestamp_millis();
        let mut sessions = self.sessions.write().map_err(|_| {
            ErrState::new(400, "セッションのロックに失敗".to_string(), None)
        })?;
        
        match sessions.get_mut(&session_vec) {
            Some(session_data) => {
                session_data.last_access_time = latest_access_time;
                session_data.access_count += 1;
            }
            None => return Err(ErrState::new(401, "セッションが見つかりません".to_string(), None)),
        }
        Ok(())
    }

    // ユーザーを設定
    pub async fn set_now_user(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, ErrState> {
        let mut sessions = self.sessions.write().map_err(|_| {
            ErrState::new(402, "セッションのロックに失敗".to_string(), None)
        })?;
        
        if let Some(session_data) = sessions.get_mut(&session_vec) {
            if let Some(pos) = session_data.users.iter().position(|&x| x == ruid) {
                let user = session_data.users.remove(pos);
                session_data.users.insert(0, user);
                Ok(Some(session_data.clone()))
            } else {
                Err(ErrState::new(403, "ユーザーが見つかりません".to_string(), None))
            }
        } else {
            Err(ErrState::new(404, "セッションが見つかりません".to_string(), None))
        }
    }

    // ユーザーを取得
    pub async fn get_now_user(&self, session_vec: Vec<u8>) -> Result<Option<u128>, ErrState> {
        let sessions = self.sessions.read().map_err(|_| {
            ErrState::new(405, "セッションのロックに失敗".to_string(), None)
        })?;

        if let Some(session_data) = sessions.get(&session_vec) {
            Ok(session_data.users.first().copied())
        } else {
            Err(ErrState::new(406, "セッションが見つかりません".to_string(), None))
        }
    }

    // セッションデータを取得
    pub async fn get(&self, session_vec: Vec<u8>) -> Result<Option<SessionData>, ErrState> {
        let sessions = self.sessions.read().map_err(|_| {
            ErrState::new(407, "セッションのロックに失敗".to_string(), None)
        })?;

        if let Some(session_data) = sessions.get(&session_vec) {
            Ok(Some(session_data.clone()))
        } else {
            Err(ErrState::new(408, "セッションが見つかりません".to_string(), None))
        }
    }

    // ユーザーを追加
    pub async fn add_user(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, ErrState> {
        let mut sessions = self.sessions.write().map_err(|_| {
            ErrState::new(409, "セッションのロックに失敗".to_string(), None)
        })?;

        if let Some(session_data) = sessions.get_mut(&session_vec) {
            session_data.users.push(ruid);
            Ok(Some(session_data.clone()))
        } else {
            Err(ErrState::new(410, "セッションが見つかりません".to_string(), None))
        }
    }

    // ユーザーを削除
    pub async fn rem_user(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, ErrState> {
        let mut sessions = self.sessions.write().map_err(|_| {
            ErrState::new(411, "セッションのロックに失敗".to_string(), None)
        })?;

        if let Some(session_data) = sessions.get_mut(&session_vec) {
            session_data.users.retain(|&x| x != ruid);
            Ok(Some(session_data.clone()))
        } else {
            Err(ErrState::new(412, "セッションが見つかりません".to_string(), None))
        }
    }

    // 新しいセッションを設定
    pub async fn set(&self) -> Result<Vec<u8>, ErrState> {
        loop {
            let session_vec = match self.generate().await {
                Err(e) => return Err(ErrState::new(413, "セッションIDの生成に失敗".to_string(), Some(e))),
                Ok(session_vec) => session_vec,
            };
            let mut sessions = self.sessions.write().map_err(|_| {
                ErrState::new(414, "セッションのロックに失敗".to_string(), None)
            })?;

            if !sessions.contains_key(&session_vec) {
                let time = Utc::now().timestamp_millis();
                sessions.insert(
                    session_vec.clone(),
                    SessionData {
                        last_access_time: time,
                        generated_time: time,
                        access_count: 0,
                        users: Vec::new(),
                    },
                );
                return Ok(session_vec);
            }
            drop(sessions); // ロックを解放して再度ループ
        }
    }

    // セッションIDを生成
    pub async fn generate(&self) -> Result<Vec<u8>, ErrState> {
        let mut buffer = vec![0u8; self.len];
        let mut rng = self.rng.lock().map_err(|_| {
            ErrState::new(415, "乱数生成器のロックに失敗".to_string(), None)
        })?;
        rng.fill_bytes(&mut buffer);
        Ok(buffer)
    }

    // セッションを削除
    pub async fn unset(&self, session_vec: Vec<u8>) -> Result<Option<SessionData>, ErrState> {
        let mut sessions = self.sessions.write().map_err(|_| {
            ErrState::new(416, "セッションのロックに失敗".to_string(), None)
        })?;

        if let Some(session_data) = sessions.remove(&session_vec) {
            Ok(Some(session_data))
        } else {
            Err(ErrState::new(417, "セッションが見つかりません".to_string(), None))
        }
    }
}
