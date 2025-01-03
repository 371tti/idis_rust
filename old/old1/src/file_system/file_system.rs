use std::sync::Arc;

use crate::utils::mime::MimeDetector;
use crate::{db_handlers::mongo_client::MongoClient, sys::init::AppConfig};

use super::meta_set::MetaData;
use super::perm_set::Perm;
use actix_web::{web, Error, HttpResponse};
use futures::StreamExt;
use std::fs::{File, Metadata};
use std::io::Write;
use std::path::Path;

pub struct FileSystem {
    pub db: Arc<MongoClient>,
    pub mime: MimeDetector,
}

impl FileSystem {
    pub async fn new(app_config: &AppConfig, db: Arc<MongoClient>) -> Self {
        Self {
            db: db,
            mime: MimeDetector::new(),
        }
    }

    pub async fn create() {


    }


    pub async  fn meta_create(name: String, path: String, size: u64, perm: Perm, icon: Option<u128>) {

      }



}
