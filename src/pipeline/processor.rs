use actix_web::{web, HttpRequest, HttpResponse};

use crate::{state_services::state_set::State, sys::app_set::{self, AppSet}};

use super::analyse::{self, Analyze};

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

    pub fn run(mut self) -> HttpResponse {
        if let Err(e) = self.analyze_http() {
            // Handle the error, e.g., log it
            eprintln!("Error analyzing HTTP: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }

        match serde_json::to_string(&self.state) {
            Ok(json) => HttpResponse::Ok()
                .content_type("application/json")
                .body(json),
            Err(e) => {
                eprintln!("Error serializing state to JSON: {:?}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

