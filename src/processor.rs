use actix_web::{web, HttpRequest, HttpResponse};

use serde_json::Value;

use crate::AppMod;


pub struct Processor {
    pub app: web::Data<AppMod>,
    pub userRUID: u128,
    pub user: i32, // アカウントレベル
    pub perm: Vec<u128>,
    pub err: Option<Value>,
    pub response: Option<HttpResponse>,
    pub request: HttpRequest,
    pub session_id: Vec<u8>,
}


impl Processor {
    pub fn new(app: web::Data<AppMod>, req: HttpRequest) -> Self{
        Self {
            app: app,
            userRUID: 0,
            user: 0,
            perm: Vec::new(),
            err: None,
            response: None,
            request: req,
            session_id: Vec::new(),
        }
    }

    pub fn session_check(&mut self) {
        let session_id_some = self.request.cookie("session_id");
        if let Some(session_id) = session_id_some { // セッションを持ってる場合
            match self.app.session.base64_to_vec(&session_id.to_string()) {
                Ok(session_vec) => {
                    
                },
                Err(e) => println!("{}",e),
            }

        } else { // 持ってない場合
            
        }
    }
}