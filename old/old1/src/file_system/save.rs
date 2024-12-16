use std::{fs::File, io::Write, path::PathBuf};

use actix_web::web;
use futures::StreamExt;

use crate::state_services::err_set::ErrState;

pub struct Save {

}

impl Save {
    pub fn new() -> Save {
        Save {}
    }

    pub async fn save_binary(&self, mut payload: web::Payload, collection: String, name: String) -> Result<usize, ErrState> {
        let mut file_path = PathBuf::from(collection);
        file_path.push(name);
        let mut file = File::create(file_path).map_err(|e| ErrState::new(1, "".to_string(), None))?;

        let mut total_bytes: usize = 0;

        while let Some(chunk) = payload.next().await {
            let data = chunk.map_err(|e| ErrState::new(1, "".to_string(), None))?;
            total_bytes += data.len();
            file.write_all(&data).map_err(|e| ErrState::new(1, "".to_string(), None))?;      
        }

        Ok(total_bytes)
    }

    pub async fn save_meta() {

    }

    pub async fn save_inline() {
        
    }
}