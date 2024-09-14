use actix_web::{get, web, App, HttpServer, Responder, middleware::Logger, HttpResponse, HttpRequest};
use env_logger::Env;
use utils::api::mongo_client::MongoClient;
use utils::api::user::User;

use std::f64::consts;
use std::{clone, string};
use std::sync::Mutex;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::ptr;
use std::sync::Arc;





mod utils;
mod sys;
mod processor;

use crate::utils::api::json_api::JsonApi;
use crate::utils::api::file_api::FileApi;

use crate::utils::ruid::Ruid;
use crate::utils::ruid::RuidGenerator;

use crate::sys::init::AppConfig;

use crate::utils::api::session::Session;

use crate::processor::Processor;

#[get("/")]
async fn index(app: web::Data<AppMod>, req: HttpRequest) -> impl Responder {
    let mut state = Processor::new(app, req);

    return "asas";
}

// #[get("/s")]
// async fn indexs(app: web::Data<AppMod>, req: HttpRequest) -> impl Responder {
//     // ここで送信するファイルのパスを指定します。
//     let file_path: PathBuf = PathBuf::from("nogera0.mp4");

//     app.file_api.stream(file_path)
//         .cors("*")
//         .inline(false)
//         .file_name("nogera.mp4")
//         .send(req)
//         .await
// }


pub struct AppMod {
    file_api: FileApi,
    json_api: JsonApi,
    session: Session,
    ruid: RuidGenerator,
    db: MongoClient,
}

impl AppMod {
    pub async fn new(app_config: AppConfig) -> Self{
        let json_api = JsonApi::new(&app_config);
        let ruid = RuidGenerator::new(&app_config);
        let session = Session::new(&app_config);
        let db = MongoClient::new(&app_config).await.expect("dbへの接続失敗　パニックなう"); // あとでエラーハンドリングする
        
        let file_api = FileApi::new(&app_config, &json_api);
        let user = User::new(&app_config, &db);

        let app_mod = AppMod {
            file_api: file_api,
            json_api: json_api,
            session: session,
            ruid: ruid,
            db: db,
        };

        return app_mod;
    }    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーの初期化
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_config = AppConfig::new();



    // `AppMod::new` を `await` して `AppMod` のインスタンスを取得
    let app_mod_instance = AppMod::new(app_config.clone()).await;

    // `AppMod` のインスタンスを `Arc` でラップし、`web::Data` に渡す
    let app_mod = web::Data::new(Arc::new(app_mod_instance));

    // サーバーの設定
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  // リクエストのログを記録するミドルウェアを追加
            .app_data(app_mod.clone()) // アプリケーション全体で共有
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
