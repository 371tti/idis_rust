use actix_web::web;

use crate::{state_services::state_set::State, sys::app_set::{self, AppSet}};

pub struct Processor {
    pub app_set: web::Data<AppSet>,
    pub state: State,
    pub body_stream: web::Payload,
    pub lock_this_server: bool,
}

impl Processor {
    pub fn new(app_set: web::Data<AppSet>, body_stream: web::Payload) -> Self {
        Self {
            app_set: app_set,
            state: State::new(),
            body_stream: body_stream,
            lock_this_server: false,
        }
    }
}