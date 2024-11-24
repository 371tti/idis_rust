use std::sync::Arc;
use ruid_set::ruid::*;
use ruid_set::prefix::Prefix;

use crate::{
    db_handlers::mongo_client::MongoClient, 
    state_services::{session::Session, user::User, user_session::UserSession}, 
};
use super::{init::AppConfig, ws::WsConnectionSet};

pub struct AppSet {
    pub db: Arc<MongoClient>,  // Arcでラップ
    pub config: AppConfig,
    pub ws: WsConnectionSet,
    pub user_session: UserSession,
}

impl AppSet {
    pub async fn new(app_config: AppConfig) -> Self {
        let db = Arc::new(
            MongoClient::new(&app_config)
                .await
                .expect("dbへの接続失敗 パニックなう"), // 後でエラーハンドリングを追加
        );
        let ws = WsConnectionSet::new(&app_config);
        let user_session = UserSession::new(&app_config, Arc::clone(&db));


        Self {
            db: Arc::clone(&db),  // Arcをクローン
            config: app_config.clone(),
            ws,
            user_session,
        }
    }
}
