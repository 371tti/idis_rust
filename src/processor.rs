use std::error::Error;

use actix_web::{web::{self, Json}, HttpRequest, HttpResponse};

use serde_json::{json, Value};

use crate::{
    utils::{api::user::UserData, json_f},
    AppMod,
};

pub struct Processor {
    pub app: web::Data<AppMod>,
    pub userRUID: u128,
    pub user_data: UserData,
    pub result: Option<Value>,
    pub request: HttpRequest,
    pub session_id: Vec<u8>,
    pub status: u32,
}

impl Processor {
    pub fn new(app: web::Data<AppMod>, req: HttpRequest) -> Self {
        Self {
            app: app,
            userRUID: 0,
            user_data: UserData::default(),
            result: None,
            request: req,
            session_id: Vec::new(),
            status: 100,
        }
    }

    pub fn session_check(&mut self) -> &mut Self {
        let result: Result<(), Box<dyn Error>> = (|| {
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
                        self.userRUID = user_ruid;
                        self.user_data = user_data;
                        self.session_id = session_vec;
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
        self.user_data = self.app.user.get(&guest_user_ruid.to_u128())?;
        self.userRUID = guest_user_ruid.to_u128();
        self.session_id = new_session_vec;

        return Ok(());
    })();
    if let Err(e) = result {
        // ロールバック処理
        if self.session_id.is_empty() == false {
            self.app.session.unset(self.session_id.clone()).unwrap_or_default();
        }
        if self.userRUID != 0 {
            self.app.user.remove(&self.userRUID).unwrap_or_default();
        }
        self.user_data = UserData::default();

        // Err返す
        self.result = Some(
            json_f::err(1, 500, "Exception in session handling.")
        );
        self.status = 500;

    } else {
    }
    self
    }

    pub fn parsing() {
        // jsonクエリにリクエストを解析変換
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
