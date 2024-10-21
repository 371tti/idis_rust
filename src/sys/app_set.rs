use crate::{build_handlers::{file_api::FileApi, json_api::JsonApi}, db_handlers::mongo_client::MongoClient, state_services::{session::Session, user::User, ws_set::WsConnectionSet}, utils::ruid::RuidGenerator};

use super::init::AppConfig;

pub struct AppSet {
    pub file_api: FileApi,
    pub json_api: JsonApi,
    pub session: Session,
    pub ruid: RuidGenerator,
    pub db: MongoClient,
    pub config: AppConfig,
    pub user: User,
    pub ws: WsConnectionSet,
}

impl AppSet {
    pub async fn new(app_config: AppConfig) -> Self{
        let json_api = JsonApi::new(&app_config);
        let ruid = RuidGenerator::new(&app_config);
        let session = Session::new(&app_config);
        let db = MongoClient::new(&app_config).await.expect("dbへの接続失敗　パニックなう"); // あとでエラーハンドリングする
        let ws = WsConnectionSet::new(&app_config);
        
        let file_api = FileApi::new(&app_config, &json_api);
        let user = User::new(&app_config, &db);

        let app_mod = AppSet {
            file_api: file_api,
            json_api: json_api,
            session: session,
            ruid: ruid,
            db: db,
            config: app_config,
            user: user,
            ws: ws,
        };

        return app_mod;
    }    
}