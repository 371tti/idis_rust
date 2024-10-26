use std::sync::Arc;
use crate::{
    build_handlers::{file_api::FileApi, json_api::JsonApi}, 
    db_handlers::mongo_client::MongoClient, 
    state_services::{session::Session, user::User}, 
    utils::ruid::RuidGenerator
};
use super::{init::AppConfig, ws::WsConnectionSet};

pub struct AppSet {
    pub session: Session,
    pub ruid: RuidGenerator,
    pub db: Arc<MongoClient>,  // Arcでラップ
    pub config: AppConfig,
    pub user: User,
    pub ws: WsConnectionSet,
}

impl AppSet {
    pub async fn new(app_config: AppConfig) -> Self {
        let ruid = RuidGenerator::new(&app_config);
        let session = Session::new(&app_config);
        let db = Arc::new(
            MongoClient::new(&app_config)
                .await
                .expect("dbへの接続失敗 パニックなう"), // 後でエラーハンドリングを追加
        );
        let ws = WsConnectionSet::new(&app_config);
        let user = User::new(&app_config, Arc::clone(&db));

        Self {
            session: session.await,
            ruid,
            db: Arc::clone(&db),  // Arcをクローン
            config: app_config.clone(),
            user: user.await,
            ws,
        }
    }
}
