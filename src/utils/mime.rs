use std::collections::HashMap;
use std::path::Path;

/// MIMEタイプを判別するための構造体
pub struct MimeDetector {
    mime_map: HashMap<&'static str, &'static str>,
}

impl MimeDetector {
    /// `MimeDetector`の新しいインスタンスを生成し、MIMEタイプを登録する
    pub fn new() -> Self {
        let mut mime_map = HashMap::new();

        // 画像ファイル
        mime_map.insert("jpg", "image/jpeg");
        mime_map.insert("jpeg", "image/jpeg");
        mime_map.insert("png", "image/png");
        mime_map.insert("gif", "image/gif");
        mime_map.insert("bmp", "image/bmp");
        mime_map.insert("tiff", "image/tiff");
        mime_map.insert("tif", "image/tiff");
        mime_map.insert("ico", "image/vnd.microsoft.icon");
        mime_map.insert("svg", "image/svg+xml");
        mime_map.insert("webp", "image/webp");

        // 動画ファイル
        mime_map.insert("mp4", "video/mp4");
        mime_map.insert("avi", "video/x-msvideo");
        mime_map.insert("mov", "video/quicktime");
        mime_map.insert("wmv", "video/x-ms-wmv");
        mime_map.insert("flv", "video/x-flv");
        mime_map.insert("mkv", "video/x-matroska");
        mime_map.insert("webm", "video/webm");
        mime_map.insert("mpeg", "video/mpeg");
        mime_map.insert("mpg", "video/mpeg");
        mime_map.insert("3gp", "video/3gpp");

        // 音声ファイル
        mime_map.insert("mp3", "audio/mpeg");
        mime_map.insert("wav", "audio/wav");
        mime_map.insert("ogg", "audio/ogg");
        mime_map.insert("m4a", "audio/mp4");
        mime_map.insert("flac", "audio/flac");
        mime_map.insert("aac", "audio/aac");
        mime_map.insert("wma", "audio/x-ms-wma");

        // 文書ファイル
        mime_map.insert("pdf", "application/pdf");
        mime_map.insert("doc", "application/vnd.openxmlformats-officedocument.wordprocessingml.document");
        mime_map.insert("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document");
        mime_map.insert("xls", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
        mime_map.insert("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet");
        mime_map.insert("ppt", "application/vnd.openxmlformats-officedocument.presentationml.presentation");
        mime_map.insert("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation");
        mime_map.insert("txt", "text/plain");
        mime_map.insert("csv", "text/csv");
        mime_map.insert("rtf", "application/rtf");
        mime_map.insert("md", "text/markdown");
        mime_map.insert("json", "application/json");
        mime_map.insert("xml", "application/xml");
        mime_map.insert("html", "text/html");
        mime_map.insert("htm", "text/html");
        mime_map.insert("css", "text/css");
        mime_map.insert("js", "application/javascript");
        mime_map.insert("yaml", "application/x-yaml");
        mime_map.insert("yml", "application/x-yaml");
        mime_map.insert("toml", "application/toml");

        // アーカイブファイル
        mime_map.insert("zip", "application/zip");
        mime_map.insert("rar", "application/vnd.rar");
        mime_map.insert("7z", "application/x-7z-compressed");
        mime_map.insert("tar", "application/x-tar");
        mime_map.insert("gz", "application/gzip");
        mime_map.insert("bz2", "application/x-bzip2");
        mime_map.insert("xz", "application/x-xz");

        // フォントファイル
        mime_map.insert("ttf", "font/ttf");
        mime_map.insert("otf", "font/otf");
        mime_map.insert("woff", "font/woff");
        mime_map.insert("woff2", "font/woff2");

        // その他
        mime_map.insert("exe", "application/vnd.microsoft.portable-executable");
        mime_map.insert("iso", "application/x-iso9660-image");
        mime_map.insert("epub", "application/epub+zip");
        mime_map.insert("apk", "application/vnd.android.package-archive");
        mime_map.insert("deb", "application/vnd.debian.binary-package");
        mime_map.insert("rpm", "application/x-rpm");
        mime_map.insert("sh", "application/x-sh");
        mime_map.insert("c", "text/x-c");
        mime_map.insert("cpp", "text/x-c++");
        mime_map.insert("h", "text/x-c");
        mime_map.insert("hpp", "text/x-c++");
        mime_map.insert("rs", "text/rust");
        mime_map.insert("py", "text/x-python");
        mime_map.insert("java", "text/x-java-source");
        mime_map.insert("class", "application/java-vm");
        mime_map.insert("jar", "application/java-archive");

        MimeDetector { mime_map }
    }

    /// ファイルパスから拡張子を使ってMIMEタイプを判別します。
    pub fn get_mime_type_from_extension(&self, file_path: &str) -> &'static str {
        let path = Path::new(file_path);

        // 拡張子からMIMEタイプを取得、見つからない場合はデフォルト
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| self.mime_map.get(ext))
            .copied()
            .unwrap_or("application/octet-stream")
    }
}
