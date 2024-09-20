use std::{collections::HashMap, sync::Mutex};
use chrono::{Duration, Utc};
use rand_chacha::ChaCha20Rng;
use rand::RngCore;

use base64::{engine::general_purpose, DecodeError, Engine as _};

use crate::sys::init::AppConfig;

#[derive(Clone)]
pub struct SessionData {
    last_access_time: u64,
    generated_time: u64,
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
    pub fn new(app_config: &AppConfig) -> Self {
        Session {
            sessions: Mutex::new(HashMap::with_capacity(app_config.session_default_capacity)),
            len: app_config.session_len_byte,
            life_time_server: app_config.session_life_time_server,
            life_time_client: app_config.session_life_time_client,
            rng: Mutex::new(app_config.session_rng.clone()),
        }
    }

    pub fn update_last_access_time(&self, session_vec: Vec<u8>) -> Result<(), String> {
        let latest_access_time = Utc::now().timestamp_millis() as u64;
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get_mut(&session_vec) {
            session_data.last_access_time = latest_access_time;
        }
        Ok(())
    }

    pub fn user_set(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get_mut(&session_vec) {
            // ruid を先頭に移動
            if let Some(pos) = session_data.users.iter().position(|&x| x == ruid) {
                let user = session_data.users.remove(pos);
                session_data.users.insert(0, user);

                Ok(Some(session_data.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn user(&self, session_vec: Vec<u8>) -> Result<Option<u128>, String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get(&session_vec) {
            Ok(session_data.users.first().copied())
        } else {
            Ok(None)
        }
    }

    pub fn get(&self, session_vec: Vec<u8>) -> Result<Option<SessionData>, String> {
        let sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get(&session_vec) {
            Ok(Some(session_data.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn add(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get_mut(&session_vec) {
            session_data.users.push(ruid);
            Ok(Some(session_data.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn rem(&self, session_vec: Vec<u8>, ruid: u128) -> Result<Option<SessionData>, String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        if let Some(session_data) = sessions.get_mut(&session_vec) {
            session_data.users.retain(|&x| x != ruid);
            Ok(Some(session_data.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn set(&self) -> Result<Vec<u8>, String> {
        loop {
            let session_vec = self.generate()?;
            let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
            if !sessions.contains_key(&session_vec) {
                let time = Utc::now().timestamp_millis() as u64;
                sessions.insert(
                    session_vec.clone(),
                    SessionData {
                        last_access_time: time,
                        generated_time: time,
                        users: Vec::new(),
                    },
                );
                return Ok(session_vec);
            }
            // ロックを解放して再度ループ
            drop(sessions);
        }
    }

    pub fn unset(&self, session_vec: Vec<u8>) -> Result<Option<SessionData>, String> {
        let mut sessions = self.sessions.lock().map_err(|e| e.to_string())?;
        Ok(sessions.remove(&session_vec))
    }

    pub fn generate(&self) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; self.len];
        let mut rng = self.rng.lock().map_err(|e| e.to_string())?;
        rng.fill_bytes(&mut buffer);

        Ok(buffer)
    }

    pub fn vec_to_base64(&self, session_vec: Vec<u8>) -> String {
        general_purpose::STANDARD.encode(session_vec)
    }

    pub fn base64_to_vec(&self, session_base64: &str) -> Result<Vec<u8>, DecodeError> {
        general_purpose::STANDARD.decode(session_base64)
    }

}
