use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::{state_services::{err_set::ErrState, state_set::State}, sys::app_set::AppSet};
use super::{analyse::Analyze, perm_load::PermLoad};

pub struct Processor {
    pub app_set: web::Data<AppSet>,
    pub state: State,
    pub body_stream: web::Payload,
    pub req: HttpRequest,
    pub lock_this_server: bool,
}

impl Processor {
    pub fn new(app_set: web::Data<AppSet>, body_stream: web::Payload, req: HttpRequest) -> Self {

        Self {
            app_set: app_set,
            state: State::new(),
            body_stream: body_stream,
            req: req,
            lock_this_server: false,
        }
    }


    pub async fn run(mut self) -> HttpResponse {

        if self.req.headers().contains_key("upgrade") {
            match self.handle_ws_request().await {
                Ok(resp) => resp,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        } else {
            match self.handle_http_request().await {
                Ok(resp) => resp,
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
    }

    async fn handle_http_request(mut self) -> Result<HttpResponse , ErrState> {
        if let Err(e) = self.analyze_http() {
            return match serde_json::to_string(&e) {
                Ok(json) => Ok(HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(json)),
                Err(e) => {
                    eprintln!("Error serializing ErrState to JSON: {:?}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            };
        }

        if let Err(e) = self.session_check_http() {
            return match serde_json::to_string(&e) {
                Ok(json) => Ok(HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(json)),
                Err(e) => {
                    eprintln!("Error serializing ErrState to JSON: {:?}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            };
        }

        match serde_json::to_string(&self.state) {
            Ok(json) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(json)),
            Err(e) => {
                eprintln!("Error serializing state to JSON: {:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
    }

    async fn handle_ws_request(mut self) -> Result<HttpResponse, ErrState> {
        self.app_set.ws:
    }
}
