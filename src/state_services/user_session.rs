// src/state_services/user_session.rs
// このモジュールは、ユーザーとセッションの状態を管理します。
// ユーザーとセッションの状態は、ユーザーのログイン状態を管理するために使用されます。

use std::sync::Arc;

use ruid_set::ruid::RuidGenerator;

use crate::db_handlers::mongo_client::MongoClient;
use crate::{db_handlers::mongo_client, sys::init::AppConfig};
use crate::state_services::session::Session;
use crate::state_services::user::User;

pub struct UserSession {
    pub user: User,
    pub session: Session,
    pub id_generator: RuidGenerator,
}

impl UserSession {
    pub fn new(app_config: &AppConfig, mongo_client: Arc<MongoClient>) -> Self {
        UserSession {
            user: User::new(app_config, mongo_client),
            session: Session::new(app_config),
            id_generator: RuidGenerator::new(),
        }
    }
    
    pub async  fn check() -> Option<> {
        
    }
}


