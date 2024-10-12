use actix_web::web;

use crate::{state_services::state_set::State, sys::app_set::AppSet};

pub struct Processor {
    pub app: web::Data<AppSet>,
    pub state: State,
    pub lock_this_server: bool,
}

impl Processor {
    
}