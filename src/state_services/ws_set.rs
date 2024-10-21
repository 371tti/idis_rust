use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::sys::init::AppConfig;

use super::state_set::State;

#[derive(Clone)]
pub struct WsSettings {
    pub heartbeat_interval: Duration,  // ハートビートの間隔
    pub client_timeout: Duration,      // クライアントのタイムアウト時間
}

/// WebSocket接続を表すアクター
pub struct WsConnection {
    pub state: State,
    pub hb: Instant,            // 最後のハートビート受信時間
    pub settings: WsSettings,   // ハートビート設定
    pub session_vec: Vec<u8>,   // セッションID
    pub connection_num: u32,    // コネクション番号
    pub ws_connection_set: Arc<WsConnectionSet>, // WsConnectionSetへの参照
    pub is_disconnected: bool,  // 切断処理が行われたかどうかのフラグ
}

impl WsConnection {
    // ハートビートの開始
    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = self.settings.heartbeat_interval;
        let timeout = self.settings.client_timeout;

        ctx.run_interval(interval, move |act, ctx| {
            // 最後のハートビートからの経過時間を確認
            if Instant::now().duration_since(act.hb) > timeout {
                println!("Heartbeat failed, disconnecting!");
                // タイムアウトの場合、接続を閉じる
                act.cleanup();  // クリーンアップ処理を呼び出す
                ctx.stop();
                return;
            }
            // Pingメッセージを送信
            ctx.ping(b"PING");
        });
    }

    // クリーンアップ処理：WsConnectionSetから自身を削除
    fn cleanup(&mut self) {
        if !self.is_disconnected {
            let mut ws_connections = self.ws_connection_set.ws_connections.lock().unwrap();
            if let Some(connection_map) = ws_connections.get_mut(&self.session_vec) {
                connection_map.remove(&self.connection_num);
                if connection_map.is_empty() {
                    ws_connections.remove(&self.session_vec);
                }
            }
            self.is_disconnected = true;  // フラグを立てる
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);  // アクターが開始したときにハートビートを開始
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("WebSocket connection stopped");
        self.cleanup();  // クリーンアップ処理を呼び出す
    }
}

// WebSocketメッセージの受信と処理
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received Text: {}", text);
                ctx.text(format!("Echo: {}", text));  // テキストメッセージをエコー
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Received Binary: {:?}", bin);
                ctx.binary(bin);  // バイナリメッセージをエコー
            }
            Ok(ws::Message::Ping(ping)) => {
                self.hb = Instant::now();  // Ping受信時にハートビートタイムを更新
                ctx.pong(&ping);  // Pongで応答
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();  // Pongを受信したらタイムスタンプを更新
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);  // クローズメッセージを受信したら接続を閉じる
                self.cleanup();      // クリーンアップ処理を呼び出す
                ctx.stop();
            }
            Err(e) => {
                println!("WebSocket error: {:?}", e);
                self.cleanup();      // エラー時にもクリーンアップ処理を呼び出す
                ctx.stop();
            }
            _ => (),
        }
    }
}

// メッセージ型を定義
#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendBinary(pub Vec<u8>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendPing(pub Vec<u8>);

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseConnection;

// Stateを取得するためのメッセージ
#[derive(Message)]
#[rtype(result = "State")]
pub struct GetState;

// メッセージハンドラの実装
impl Handler<SendMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);  // テキストメッセージを送信
    }
}

impl Handler<SendBinary> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendBinary, ctx: &mut Self::Context) {
        ctx.binary(msg.0);  // バイナリメッセージを送信
    }
}

impl Handler<SendPing> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendPing, ctx: &mut Self::Context) {
        ctx.ping(&msg.0);  // Pingメッセージを送信
    }
}

impl Handler<CloseConnection> for WsConnection {
    type Result = ();

    fn handle(&mut self, _: CloseConnection, ctx: &mut Self::Context) {
        ctx.close(None);  // 接続を閉じる
        self.cleanup();   // クリーンアップ処理を呼び出す
        ctx.stop();
    }
}

// GetStateメッセージに対するハンドラ
impl Handler<GetState> for WsConnection {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, _msg: GetState, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.state.clone())  // `State`を複製して返す
    }
}

/// WebSocket接続を管理するセット
pub struct WsConnectionSet {
    pub ws_connections: Mutex<HashMap<Vec<u8>, HashMap<u32, Addr<WsConnection>>>>,  // セッションID -> コネクション番号 -> アクターアドレス
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

    // 特定のコネクションにテキストメッセージを送信
    pub fn send_message(&self, session_vec: Vec<u8>, connection_num: u32, message: String) {
        let ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendMessage(message));  // メッセージを送信
            }
        }
    }

    // 特定のコネクションにバイナリメッセージを送信
    pub fn send_binary(&self, session_vec: Vec<u8>, connection_num: u32, data: Vec<u8>) {
        let ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendBinary(data));  // バイナリメッセージを送信
            }
        }
    }

    // 特定のコネクションにPingを送信
    pub fn send_ping(&self, session_vec: Vec<u8>, connection_num: u32, ping_data: Vec<u8>) {
        let ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                ws_connection.do_send(SendPing(ping_data));  // Pingメッセージを送信
            }
        }
    }

    // 特定のWebSocketコネクションの作成
    pub fn create(
        self: &Arc<Self>,
        session_vec: Vec<u8>,
        connection_num: u32,
        req: &actix_web::HttpRequest,   // WebSocket初期化に必要なHttpRequest
        stream: actix_web::web::Payload, // WebSocket初期化に必要なストリーム
        state: State,
    ) -> Addr<WsConnection> {
        // 新しいWsConnectionを作成
        let ws_connection = WsConnection {
            state,
            hb: Instant::now(),
            settings: self.ws_settings.clone(),  // WsSettingsをコピーして渡す
            session_vec: session_vec.clone(),
            connection_num,
            ws_connection_set: Arc::clone(self),
            is_disconnected: false,  // フラグを初期化
        };

        // WebSocketアクターを起動
        let (addr, _response) = ws::WsResponseBuilder::new(ws_connection, req, stream)
            .start_with_addr()
            .unwrap();

        // アクターアドレスを管理に追加
        {
            let mut ws_connections = self.ws_connections.lock().unwrap();
            let connection_map = ws_connections.entry(session_vec.clone()).or_insert_with(HashMap::new);
            connection_map.insert(connection_num, addr.clone());
        }

        addr
    }

    // 特定のWebSocketコネクションを閉じる
    pub fn close(&self, session_vec: Vec<u8>, connection_num: u32) {
        let mut ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get_mut(&session_vec) {
            if let Some(ws_connection) = connection_map.remove(&connection_num) {
                ws_connection.do_send(CloseConnection);  // 接続を閉じる
            }
            if connection_map.is_empty() {
                ws_connections.remove(&session_vec);
            }
        }
    }

    // セッション内の全コネクションの数を取得する
    pub fn get_connection_count(&self, session_vec: Vec<u8>) -> usize {
        let ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get(&session_vec) {
            connection_map.len()  // コネクション数を返す
        } else {
            0  // 存在しないセッションの場合は0を返す
        }
    }

    // セッション内の特定のコネクションのStateを非同期で取得する
    pub async fn get_connection_state(
        &self,
        session_vec: Vec<u8>,
        connection_num: u32,
    ) -> Option<State> {
        let ws_connections = self.ws_connections.lock().unwrap();
        if let Some(connection_map) = ws_connections.get(&session_vec) {
            if let Some(ws_connection) = connection_map.get(&connection_num) {
                // `GetState`メッセージをアクターに送信し、その応答を取得
                match ws_connection.send(GetState).await {
                    Ok(state) => Some(state),
                    Err(_) => None,  // 送信が失敗した場合
                }
            } else {
                None  // コネクションが存在しない場合
            }
        } else {
            None  // セッションが存在しない場合
        }
    }
}
