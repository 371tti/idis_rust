use std::{collections::HashMap, sync::Mutex};

use chrono::{Duration, Utc};
use mongodb::change_stream::session;
use rand_chacha::ChaCha20Rng;
use rand::RngCore;

use base64::{engine::general_purpose, DecodeError, Engine as _}; 

use crate::sys::init::AppConfig;

#[derive(Clone)]  
pub struct SessionData {
    last_access_time: u64,
    users: Vec<u128>,
}

pub struct Session {
    sessions: Mutex<HashMap<Vec<u8>, SessionData>>,
    len: usize,
    life_time_server: Duration,
    life_time_client: Duration,
    rng: Mutex<ChaCha20Rng>,
    
}

impl Session {
    pub fn new(app_config: &AppConfig) -> Self{
        Session {
            sessions: Mutex::new(HashMap::with_capacity(app_config.session_default_capacity)),
            len: app_config.session_len_byte,
            life_time_server: app_config.session_life_time_server,
            life_time_client: app_config.session_life_time_client,
            rng: Mutex::new(app_config.session_rng.clone()),
        }
    }

    pub fn update_last_access_time(&self, session_vec: Vec<u8>) {
        let latest_access_time = Utc::now().timestamp_millis() as u64;
        if let Some(session_data) = self.sessions.lock().unwrap().get_mut(&session_vec) {
            session_data.last_access_time = latest_access_time;
        }
    }


    pub fn user_set(&self, session_vec: Vec<u8>, ruid: u128) -> Option<SessionData> { // セッションのユーザーをセット
        if let Some(session_data) = self.sessions.lock().unwrap().get_mut(&session_vec) {
            // ruid を先頭に移動
            if let Some(pos) = session_data.users.iter().position(|&x| x == ruid) {
                let user = session_data.users.remove(pos);
                session_data.users.insert(0, user);

                Some(session_data.clone())
            } else {
                None
            }
    
        } else {
            None
        }
    }

    pub fn user(&self, session_vec: Vec<u8>) -> Option<u128> {
        if let Some(session_data) = self.sessions.lock().unwrap().get(&session_vec) {
            session_data.users.first().copied()
        } else {
            None
        }
    }

    pub fn get(&self , session_vec: Vec<u8>) -> Option<SessionData> { // セッションにあるユーザーの一覧
        if let Some(session_data) = self.sessions.lock().unwrap().get(&session_vec) {
            Some(session_data.clone())
        } else {
            None
        }
    }

    pub fn add(&self, session_vec: Vec<u8>, ruid: u128) -> Option<SessionData>{ // セッションにユーザーを追加
        if let Some(session_data) = self.sessions.lock().unwrap().get_mut(&session_vec) {
            session_data.users.push(ruid);
            Some(session_data.clone())
        } else {
            None
        }
    }

    pub fn rem(&self, session_vec: Vec<u8>, ruid: u128) -> Option<SessionData> { // セッションのユーザーを削除
        if let Some(session_data) = self.sessions.lock().unwrap().get_mut(&session_vec) {
            session_data.users.retain(|&x| x != ruid);
            Some(session_data.clone())
        } else {
            None
        }
    }

    pub fn set(&self) -> Vec<u8> { // 新しいセッション
        let session_vec = self.generate();
        if self.sessions.lock().unwrap().contains_key(&session_vec) {
            self.set()
        } else {
            let latest_access_time = Utc::now().timestamp_millis() as u64;
            self.sessions.lock().unwrap().insert(session_vec.clone(), SessionData{
                last_access_time: latest_access_time,
                users: Vec::new(),
            });
            session_vec
        }
    }

    pub fn unset(&self, session_vec: Vec<u8>) -> Option<SessionData> { // セッションを削除
        self.sessions.lock().unwrap().remove(&session_vec)
    }

    pub fn generate(&self) -> Vec<u8> { // 乱数の生成
        let mut buffer = vec![0u8; self.len];
        self.rng.lock().unwrap().fill_bytes(&mut buffer);

        buffer
    }

    pub fn vec_to_base64(&self, session_vec: Vec<u8>) -> String{
        general_purpose::STANDARD.encode(session_vec)
    }

    pub fn base64_to_vec(&self, session_base64: &str) -> Result<Vec<u8>, DecodeError> {
        general_purpose::STANDARD.decode(session_base64)
    }
}