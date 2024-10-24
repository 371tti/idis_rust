// ws_connection.rs

use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use std::sync::Arc;

// 必要なモジュールのインポート
use crate::state_services::err_set::ErrState;
use crate::state_services::state_set::State;
use crate::sys::ws::WsConnectionSet;

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

#[derive(Clone)]
pub struct WsSettings {
    pub heartbeat_interval: Duration,
    pub client_timeout: Duration,
}

/// WebSocket接続を表すアクター
pub struct WsConnection {
    pub state: State,
    pub hb: Instant,
    pub settings: WsSettings,
    pub session_vec: Vec<u8>,
    pub connection_num: u32,
    pub ws_connection_set: Arc<WsConnectionSet>,
    pub is_disconnected: bool,
}

impl WsConnection {
    // ハートビートの開始
    pub fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        let interval = self.settings.heartbeat_interval;
        let timeout = self.settings.client_timeout;

        ctx.run_interval(interval, move |act, ctx| {
            if Instant::now().duration_since(act.hb) > timeout {
                // ハートビートタイムアウト時のエラーハンドリング
                match act.cleanup() {
                    Ok(_) => {
                        let error = ErrState::new(1001, "ハートビートがタイムアウトしました".to_string(), None);
                        println!("エラー: {:?}", error);
                    }
                    Err(e) => {
                        let error = ErrState::new(1001, "ハートビートがタイムアウトし、クリーンアップに失敗しました".to_string(), Some(e));
                        println!("エラー: {:?}", error);
                    }
                }
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }

    // クリーンアップ処理
    pub fn cleanup(&mut self) -> Result<(), ErrState> {
        if !self.is_disconnected {
            let mut ws_connections = self.ws_connection_set.ws_connections.lock().map_err(|_e| {
                ErrState::new(1002, "内部エラーが発生しました".to_string(), None)
            })?;

            if let Some(connection_map) = ws_connections.get_mut(&self.session_vec) {
                connection_map.remove(&self.connection_num);
                if connection_map.is_empty() {
                    ws_connections.remove(&self.session_vec);
                }
            }
            self.is_disconnected = true;
        }
        Ok(())
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        if let Err(e) = self.cleanup() {
            let error = ErrState::new(1017, "停止時のクリーンアップに失敗しました".to_string(), Some(e));
            println!("エラー: {:?}", error);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Ping(ping)) => {
                self.hb = Instant::now();
                ctx.pong(&ping);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                if let Err(e) = self.cleanup() {
                    let error = ErrState::new(1018, "クローズ時のクリーンアップに失敗しました".to_string(), Some(e));
                    println!("エラー: {:?}", error);
                }
                ctx.stop();
            }
            Err(_e) => {
                let error = ErrState::new(1003, "WebSocketプロトコルエラーが発生しました".to_string(), None);
                println!("エラー: {:?}", error);
                if let Err(e) = self.cleanup() {
                    let error = ErrState::new(1019, "エラー発生時のクリーンアップに失敗しました".to_string(), Some(e));
                    println!("エラー: {:?}", error);
                }
                ctx.stop();
            }
            _ => (),
        }
    }
}

// メッセージハンドラの実装
impl Handler<SendMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl Handler<SendBinary> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendBinary, ctx: &mut Self::Context) {
        ctx.binary(msg.0);
    }
}

impl Handler<SendPing> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: SendPing, ctx: &mut Self::Context) {
        ctx.ping(&msg.0);
    }
}

impl Handler<CloseConnection> for WsConnection {
    type Result = ();

    fn handle(&mut self, _: CloseConnection, ctx: &mut Self::Context) {
        ctx.close(None);
        if let Err(e) = self.cleanup() {
            let error = ErrState::new(1020, "CloseConnectionハンドラでのクリーンアップに失敗しました".to_string(), Some(e));
            println!("エラー: {:?}", error);
        }
        ctx.stop();
    }
}

// GetStateメッセージに対するハンドラ
impl Handler<GetState> for WsConnection {
    type Result = MessageResult<GetState>;

    fn handle(&mut self, _msg: GetState, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.state.clone())
    }
}
