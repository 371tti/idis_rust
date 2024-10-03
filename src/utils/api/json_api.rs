// src/api/json_api.rs

use actix_web::{HttpResponse, HttpRequest, http::StatusCode};
use bytes::Bytes;
use serde_json::Value;

use actix_web::http::header::{CONTENT_TYPE, HeaderMap};

use crate::sys::init::AppConfig;

#[derive(Clone)]
pub struct JsonApi {
    chunk_size: usize,
}

impl JsonApi {
    pub fn new(app_config: &AppConfig) -> JsonApi {
        JsonApi {
            chunk_size: app_config.streaming_chunk_size,
        }
    }

    pub fn stream(&self, json_data: Value) -> JsonStream {
        JsonStream {
            json_data,
            chunk_size: self.chunk_size,
            headers: HeaderMap::new(),
            status_code: StatusCode::OK,
            file_name: None,
            inline: false,
        }
    }
}

pub struct JsonStream {
    json_data: Value,
    chunk_size: usize,
    headers: HeaderMap,
    status_code: StatusCode,
    file_name: Option<String>,
    inline: bool,
}

impl JsonStream {
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.parse().unwrap(), value.parse().unwrap());
        self
    }

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.header(actix_web::http::header::CONTENT_TYPE.as_str(), content_type)
    }

    pub fn cache_control(mut self, value: &str) -> Self {
        self.header(actix_web::http::header::CACHE_CONTROL.as_str(), value)
    }

    pub fn expires(mut self, value: &str) -> Self {
        self.header(actix_web::http::header::EXPIRES.as_str(), value)
    }

    pub fn cors(mut self, origin: &str) -> Self {
        self.header(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(), origin)
    }

    pub fn etag(mut self, tag: &str) -> Self {
        self.headers.insert(actix_web::http::header::ETAG.as_str().parse().unwrap(), tag.parse().unwrap());
        self
    }

    pub fn file_name(mut self, file_name: &str) -> Self {
        self.file_name = Some(file_name.to_string());
        self
    }

    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = inline;
        self
    }

    pub async fn send(self) -> HttpResponse {
        // JSON データを文字列に変換
        let json_string = serde_json::to_string(&self.json_data).unwrap_or_default();
        let total_length = json_string.len() as u64;

        // ヘッダー設定
        let mut response_builder = HttpResponse::build(self.status_code);
        response_builder.insert_header((CONTENT_TYPE, "application/json"));

        // Content-Dispositionの設定（ファイル名とインライン/添付の設定）
        if let Some(file_name) = &self.file_name {
            let disposition = if self.inline {
                "inline"
            } else {
                "attachment"
            };
            response_builder.insert_header((
                actix_web::http::header::CONTENT_DISPOSITION,
                format!("{}; filename=\"{}\"", disposition, file_name),
            ));
        }

        for (key, value) in self.headers.iter() {
            response_builder.insert_header((key.clone(), value.clone()));
        }

        let stream = futures::stream::unfold((json_string, 0, total_length, self.chunk_size), |(json_string, pos, total_length, chunk_size)| async move {
            if pos as u64 >= total_length {
                return None;
            }

            let end_pos = (pos + chunk_size).min(total_length as usize);
            let chunk = json_string[pos..end_pos].to_string();
            let next_pos = end_pos;

            Some((Ok::<_, std::io::Error>(Bytes::from(chunk)), (json_string, next_pos, total_length, chunk_size)))
        });

        response_builder.streaming(Box::pin(stream))
    }
}
