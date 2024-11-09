// mime_detector.rs

use std::path::Path;

/// MIMEタイプを判別するための構造体
pub struct MimeDetector;

impl MimeDetector {
    /// ファイルパスから拡張子を使ってMIMEタイプを判別します。
    pub fn get_mime_type_from_extension(file_path: &str) -> &'static str {
        let path = Path::new(file_path);

        match path.extension().and_then(|ext| ext.to_str()) {
            // 画像ファイル
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("bmp") => "image/bmp",
            Some("tiff") | Some("tif") => "image/tiff",
            Some("ico") => "image/vnd.microsoft.icon",
            Some("svg") => "image/svg+xml",
            Some("webp") => "image/webp",

            // 動画ファイル
            Some("mp4") => "video/mp4",
            Some("avi") => "video/x-msvideo",
            Some("mov") => "video/quicktime",
            Some("wmv") => "video/x-ms-wmv",
            Some("flv") => "video/x-flv",
            Some("mkv") => "video/x-matroska",
            Some("webm") => "video/webm",
            Some("mpeg") | Some("mpg") => "video/mpeg",
            Some("3gp") => "video/3gpp",

            // 音声ファイル
            Some("mp3") => "audio/mpeg",
            Some("wav") => "audio/wav",
            Some("ogg") => "audio/ogg",
            Some("m4a") => "audio/mp4",
            Some("flac") => "audio/flac",
            Some("aac") => "audio/aac",
            Some("wma") => "audio/x-ms-wma",

            // 文書ファイル
            Some("pdf") => "application/pdf",
            Some("doc") | Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            Some("xls") | Some("xlsx") => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Some("ppt") | Some("pptx") => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            Some("txt") => "text/plain",
            Some("csv") => "text/csv",
            Some("rtf") => "application/rtf",
            Some("md") => "text/markdown",
            Some("json") => "application/json",
            Some("xml") => "application/xml",
            Some("html") | Some("htm") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("yaml") | Some("yml") => "application/x-yaml",
            Some("toml") => "application/toml",

            // アーカイブファイル
            Some("zip") => "application/zip",
            Some("rar") => "application/vnd.rar",
            Some("7z") => "application/x-7z-compressed",
            Some("tar") => "application/x-tar",
            Some("gz") => "application/gzip",
            Some("bz2") => "application/x-bzip2",
            Some("xz") => "application/x-xz",

            // フォントファイル
            Some("ttf") => "font/ttf",
            Some("otf") => "font/otf",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",

            // その他
            Some("exe") => "application/vnd.microsoft.portable-executable",
            Some("iso") => "application/x-iso9660-image",
            Some("epub") => "application/epub+zip",
            Some("apk") => "application/vnd.android.package-archive",
            Some("deb") => "application/vnd.debian.binary-package",
            Some("rpm") => "application/x-rpm",
            Some("sh") => "application/x-sh",
            Some("c") => "text/x-c",
            Some("cpp") => "text/x-c++",
            Some("h") => "text/x-c",
            Some("hpp") => "text/x-c++",
            Some("rs") => "text/rust",
            Some("py") => "text/x-python",
            Some("java") => "text/x-java-source",
            Some("class") => "application/java-vm",
            Some("jar") => "application/java-archive",

            // デフォルト
            _ => "application/octet-stream", // 不明なファイルはバイナリデータ扱い
        }
    }
}
