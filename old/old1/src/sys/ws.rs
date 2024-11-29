// ws_connection_set.rs

use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::state_services::err_set::ErrState;
use crate::state_services::state_set::State;
use crate::sys::init::AppConfig;

// ws_connectionモジュールをインポート
use crate::actors::ws::{
    WsConnection, WsSettings, SendMessage, SendBinary, SendPing, CloseConnection, GetState,
};

/// WebSocket接続を管理するセット
pub struct WsConnectionSet {
    pub ws_connections: Mutex<HashMap<Vec<u8>, HashMap<u32, Addr<WsConnection>>>>,
    pub ws_settings: WsSettings,
}

impl WsConnectionSet {
    // 新しいWsConnectionSetを生成
    pub fn new(app_config: &AppConfig) -> Self {
        Self {
            ws_connections: Mutex::new(HashMap::new()),
            ws_settings: WsSettings {
                heartbeat_interval: Duration::from_secs(app_config.ws_heartbeat_interval),
                client_timeout: Duration::from_secs(app_config.ws_client_timeout),
            },
        }
    }

    // コネクションにテキストメッセージを送信
    pub fn send_message(&self, session_vec: Vec<u8>, connection_num: u32, message: String) -> Result<(), ErrState> {
        let ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1004, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendMessage(message));
                Ok(())
            } else {
                Err(ErrState::new(1013, "指定されたコネクション番号が見つかりません".to_string(), None))
            }
        } else {
            Err(ErrState::new(1014, "指定されたセッションIDが見つかりません".to_string(), None))
        }
    }

    // コネクションにバイナリメッセージを送信
    pub fn send_binary(&self, session_vec: Vec<u8>, connection_num: u32, data: Vec<u8>) -> Result<(), ErrState> {
        let ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1005, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendBinary(data));
                Ok(())
            } else {
                Err(ErrState::new(1013, "指定されたコネクション番号が見つかりません".to_string(), None))
            }
        } else {
            Err(ErrState::new(1014, "指定されたセッションIDが見つかりません".to_string(), None))
        }
    }

    // コネクションにPingを送信
    pub fn send_ping(&self, session_vec: Vec<u8>, connection_num: u32, ping_data: Vec<u8>) -> Result<(), ErrState> {
        let ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1006, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendPing(ping_data));
                Ok(())
            } else {
                Err(ErrState::new(1013, "指定されたコネクション番号が見つかりません".to_string(), None))
            }
        } else {
            Err(ErrState::new(1014, "指定されたセッションIDが見つかりません".to_string(), None))
        }
    }

    // コネクションの作成
    pub fn create(
        self: &Arc<Self>,
        session_vec: Vec<u8>,
        connection_num: u32,
        req: &actix_web::HttpRequest,
        stream: actix_web::web::Payload,
        state: State,
    ) -> Result<Addr<WsConnection>, ErrState> {
        let ws_connection = WsConnection {
            state,
            hb: Instant::now(),
            settings: self.ws_settings.clone(),
            session_vec: session_vec.clone(),
            connection_num,
            ws_connection_set: Arc::clone(self),
            is_disconnected: false,
        };

        let (addr, _response) = ws::WsResponseBuilder::new(ws_connection, req, stream)
            .start_with_addr()
            .map_err(|_e| {
                ErrState::new(1008, "WebSocketアクターの起動に失敗しました".to_string(), None)
            })?;

        let mut ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1007, "内部エラーが発生しました".to_string(), None)
        })?;

        let connection_map = ws_connections.entry(session_vec.clone()).or_insert_with(HashMap::new);
        connection_map.insert(connection_num, addr.clone());

        Ok(addr)
    }

    // コネクションを閉じる
    pub fn close(&self, session_vec: Vec<u8>, connection_num: u32) -> Result<(), ErrState> {
        let mut ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1009, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get_mut(&session_vec) {
            if let Some(ws_connection) = connection_map.remove(&connection_num) {
                ws_connection.do_send(CloseConnection);
            } else {
                return Err(ErrState::new(1015, "指定されたコネクション番号が見つかりません".to_string(), None));
            }
            if connection_map.is_empty() {
                ws_connections.remove(&session_vec);
            }
            Ok(())
        } else {
            Err(ErrState::new(1016, "指定されたセッションIDが見つかりません".to_string(), None))
        }
    }

    // コネクションの数を取得
    pub fn get_connection_count(&self, session_vec: Vec<u8>) -> Result<usize, ErrState> {
        let ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1010, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get(&session_vec) {
            Ok(connection_map.len())
        } else {
            Ok(0)
        }
    }

    // コネクションのStateを取得
    pub async fn get_connection_state(
        &self,
        session_vec: Vec<u8>,
        connection_num: u32,
    ) -> Result<Option<State>, ErrState> {
        let ws_connections = self.ws_connections.lock().map_err(|_e| {
            ErrState::new(1012, "内部エラーが発生しました".to_string(), None)
        })?;

        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.send(GetState).await.map(Some).map_err(|_e| {
                    ErrState::new(1011, "Stateの取得に失敗しました".to_string(), None)
                })
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
