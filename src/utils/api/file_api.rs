// src/api/file_app.rs

use actix_web::{HttpResponse, http::header::HeaderMap, http::StatusCode, HttpRequest};
use std::path::{PathBuf, Path};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, SeekFrom, AsyncSeekExt};
use mime_guess::from_path;
use bytes::Bytes;
use actix_web::body::SizedStream;

use crate::utils::json_f;
use crate::utils::api::json_api::JsonApi;

use crate::sys::init::AppConfig;

#[derive(Clone)]
pub struct FileApi {
    default_path: PathBuf,
    streaming_chunk_size: usize,
    chunked_threshold_size: u64,
    json_api: JsonApi,
}

impl FileApi {
    pub fn new(app_config: &AppConfig, json_api: &JsonApi) -> FileApi {
        Self {
            default_path: app_config.default_path.clone().into(),
            streaming_chunk_size: app_config.streaming_chunk_size,
            chunked_threshold_size: app_config.chunked_threshold_size,
            json_api: json_api.clone(),
        }
    }

    pub fn stream(&self, relative_path: impl Into<PathBuf>) -> FileStream {
        let mut file_path = relative_path.into();

        if file_path.is_absolute() {
            file_path = self.default_path.join(file_path.strip_prefix("/").unwrap_or_else(|_| Path::new("")));
        } else {
            file_path = self.default_path.join(file_path);
        }

        FileStream {
            file_path,
            status_code: StatusCode::OK,
            headers: HeaderMap::new(),
            chunk_size: self.streaming_chunk_size,
            file_name: None,
            inline: false,
            chunked_threshold_size: self.chunked_threshold_size,
            json_api: self.json_api.clone(),
        }
    }
}

pub struct FileStream {
    file_path: PathBuf,
    status_code: StatusCode,
    headers: HeaderMap,
    chunk_size: usize,
    file_name: Option<String>,
    inline: bool,
    chunked_threshold_size: u64,
    json_api: JsonApi,
}

impl FileStream {
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

    pub async fn send(self, req: HttpRequest) -> HttpResponse {
        let file_path = self.file_path;

        let mut file = match File::open(&file_path).await {
            Ok(file) => file,
            Err(_) => {
                let mut response = self.json_api.stream(json_f::err(0, 404, "file is not found"));
                
                // self.headersのすべてのヘッダーを追加
                for (key, value) in &self.headers {
                    response = response.header(key.as_str(), value.to_str().unwrap_or("invalid header value"));
                }
    
                return response.inline(self.inline).send().await;
            }
        };
    
        // メタデータを取得
        let metadata = match file.metadata().await {
            Ok(metadata) => metadata,
            Err(_) => {
                let mut response = self.json_api.stream(json_f::err(0, 500, "could not read file metadata"));
    
                // self.headersのすべてのヘッダーを追加
                for (key, value) in &self.headers {
                    response = response.header(key.as_str(), value.to_str().unwrap_or("invalid header value"));
                }
    
                return response.inline(self.inline).send().await;
            }
        };

        let file_size = metadata.len();

        // Rangeヘッダーの解析
        let range_header = req.headers().get("Range").and_then(|header| header.to_str().ok());

        let (start, end, status) = if let Some(range) = range_header {
            if range.starts_with("bytes=") {
                let ranges: Vec<&str> = range[6..].split('-').collect();
                let start = ranges[0].parse::<u64>().unwrap_or(0);
                let end = ranges.get(1).and_then(|&r| r.parse::<u64>().ok()).unwrap_or(file_size - 1);
                (start, end, StatusCode::PARTIAL_CONTENT)
            } else {
                (0, file_size - 1, StatusCode::OK)
            }
        } else {
            (0, file_size - 1, StatusCode::OK)
        };

        let content_length = end - start + 1;
        let mime_type = from_path(&file_path).first_or_octet_stream();

        // ヘッダー設定用に変数にコピーを保持
        let mime_type_str = mime_type.as_ref().to_string();

        // 先にヘッダーを設定してからresponseに追加
        let mut response_builder = HttpResponse::build(status);
        response_builder.insert_header((actix_web::http::header::CONTENT_TYPE, mime_type_str));
        response_builder.insert_header(("Content-Length", content_length.to_string()));

        // Content-Disposition設定
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

        // Content-Rangeヘッダーの設定
        if status == StatusCode::PARTIAL_CONTENT {
            response_builder.insert_header((
                actix_web::http::header::CONTENT_RANGE,
                format!("bytes {}-{}/{}", start, end, file_size),
            ));
        }

        for (key, value) in self.headers.iter() {
            response_builder.insert_header((key.clone(), value.clone()));
        }

        let stream = futures::stream::unfold((file, start, end, self.chunk_size), |(mut file, pos, end, chunk_size)| async move {
            if pos > end {
                return None;
            }
        
            // ファイルを指定された位置にシーク
            if let Err(_) = file.seek(SeekFrom::Start(pos)).await {
                return None;
            }
        
            let mut buffer = vec![0; chunk_size];
            let bytes_read = match file.read(&mut buffer).await {
                Ok(0) => return None, // 読み込み終了
                Ok(n) => n,
                Err(_) => return None,
            };
        
            Some((Ok::<_, std::io::Error>(Bytes::from(buffer[..bytes_read].to_vec())), (file, pos + bytes_read as u64, end, chunk_size)))
        });

        if content_length < self.chunked_threshold_size {
            // Use Content-Length and send as a single response
            response_builder.body(SizedStream::new(content_length, stream))
        } else {
            // Use chunked transfer encoding
            response_builder.streaming(Box::pin(stream))
        }
    }
}
