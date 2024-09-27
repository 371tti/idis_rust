use std::{collections::HashMap, error::Error, ptr::metadata};

use actix_web::{http::Method, web::{self, Json}, HttpRequest, HttpResponse};

use serde_json::{json, Value};

use crate::{
    utils::{api::user::UserData, json_f, state::State},
    AppMod,
};

pub struct Processor {
    pub app: web::Data<AppMod>,
    pub state: State,
    pub lock_this_server: bool,
}

impl Processor {
    pub fn new(app: web::Data<AppMod>) -> Self {
        Self {
            app: app,
            state: State::new(),
            lock_this_server: false,
        }
    }





    pub fn analyze(&mut self, req: HttpRequest) -> &mut Self {
        self.state.stage = 1;
        let method = req.method().as_str();
        if method == "POST" || method == "PUT" || method == "PATCH" {
            if let Some(content_length) = req.headers().get("Content-Length")
                .and_then(|val| val.to_str().ok())
                .and_then(|val| val.parse::<u64>().ok())
            {
                self.lock_this_server = content_length > self.app.config.server_cluster_lock_content_len;
            }
        } else if method == "GET" {
            
        }

        if let Some(session_id) = req.cookie("session_id") {
            if let Ok(session_vec) = self.app.session.base64_to_vec(session_id) {
                self.state.session_id = Some(session_vec);
            } else {
                self.state.session_id = None;
            }
        } else {
            self.state.session_id = None;
        }

        let url = req.uri().to_string();
        let referer: Option<&str> = req.headers().get("Referer")
            .and_then(|val| val.to_str().ok());
        let qery = json!(web::Query::<HashMap<String, String>>::from_query(req.query_string())
            .unwrap_or_else(|_| web::Query(HashMap::new()))
            .into_inner());


        self.state.result = Some(json_f::state_request(&url, qery, method, &Vec::new(), &referer));

        self
        // jsonクエリにリクエストを解析変換
    }







    pub fn session_check(&mut self) -> &mut Self {
        let result: Result<(), Box<dyn Error>> = (|| {
            self.state.stage = 2;
        // クッキーから session_id を取得
        if let Some(session_id) = self.request.cookie("session_id") {
            // session_id を base64 から Vec<u8> に変換
            if let Ok(session_vec) = self.app.session.base64_to_vec(&session_id.to_string()) {
                // session_vec の長さを確認
                if self.app.config.session_len_byte == session_vec.len() {
                    // ユーザーセッションが存在するか確認
                    if let Some(user_ruid) = self.app.session.user(session_vec.clone())? {
                        // 最終アクセス時間を更新
                        self.app
                            .session
                            .update_last_access_time(session_vec.clone())?;
                        self.app.user.update_last_access_time(&user_ruid)?;
                        // ユーザー情報を取得
                        let user_data = self.app.user.get(&user_ruid)?;
                        // 情報をステートにコピー
                        self.state.user_ruid = user_ruid;
                        self.state.session_id = session_vec;
                        return Ok(());
                    } else {
                        // ユーザーセッションがない場合
                    }
                } else {
                    // session_vec の長さが無効な場合の処理
                }
            } else {
                // base64 のフォーマットが無効な場合の処理
            }
        } else {
            // セッションを持ってない
        }

        // セッションを生成
        let new_session_vec = self.app.session.set()?;
        // ユーザーセッションを生成
        let guest_user_ruid = self.app.ruid.generate(0x0000, None);
        self.app.user.set(
            &guest_user_ruid.to_u128(),
            &format!("@guest{}", guest_user_ruid.to_string()),
            &0,
            &vec![0u128],
        )?;
        // 情報をステートにコピー
        self.state.user_ruid = guest_user_ruid.to_u128();
        self.state.session_id = new_session_vec;

        return Ok(());
    })();
    if let Err(e) = result {
        // ロールバック処理
        if self.session_id.is_empty() == false {
            self.app.session.unset(self.session_id.clone()).unwrap_or_default();
        }
        if self.user_ruid != 0 {
            self.app.user.remove(&self.user_ruid).unwrap_or_default();
        }

        // Err返す
        self.result = Some(
            json_f::err(1, 500, "Exception in session handling.")
        );
        self.status = 500;

    } else {
    }
    self
    }



    pub fn auth() {
        // 権限確認
    }

    pub fn endpoint() {
        // 内部エンドポイント郡
    }

    pub fn build() {
        // リクエストビルダー
    }


}
