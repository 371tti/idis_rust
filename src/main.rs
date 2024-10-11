/*
IDIS RUST version
author 371tti


*/

use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger, HttpResponse, HttpRequest};
use env_logger::Env;
use sys::app_set::AppSet;
use std::f64::consts;
use std::{clone, string};
use std::sync::Mutex;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::ptr;
use std::sync::Arc;

//  load user module
mod utils;
mod sys;
mod build_handlers;
mod db_handlers;
mod state_services;

use crate::build_handlers::json_api::JsonApi;
use crate::build_handlers::file_api::FileApi;
use crate::utils::ruid::Ruid;
use crate::utils::ruid::RuidGenerator;
use crate::sys::init::AppConfig;
use crate::state_services::session::Session;

#[get("/")]
async fn index(app_set: web::Data<AppSet>, req: HttpRequest) -> impl Responder {
""
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーの初期化
    env_logger::init_from_env(Env::default().default_filter_or("error"));

    let app_config = AppConfig::new();



    // `AppMod::new` を `await` して `AppMod` のインスタンスを取得
    let app_set_instance = AppSet::new(app_config.clone()).await;

    // `AppMod` のインスタンスを `Arc` でラップし、`web::Data` に渡す
    let app_set = web::Data::new(app_set_instance);


    // サーバーの設定
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // リクエストのログを記録するミドルウェアを追加
            .app_data(app_set.clone()) // アプリケーション全体で共有
            .service(index)           // ルーティングのサービスを追加
    })
    .bind(app_config.server_bind.clone())?
    .workers(app_config.server_workers.clone())
    .backlog(app_config.server_backlog.clone())
    .server_hostname(app_config.server_name_host.clone())
    .run();

    // サーバーをバックグラウンドで実行し、その終了を待機
    server.await?;

    Ok(())
}
