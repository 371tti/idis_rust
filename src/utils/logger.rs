use actix_web::middleware::Logger;

pub fn custom_actix_logger(server_name: &str) -> Logger {
    Logger::new(
        format!(
            "\n[{}] \n\
            Client IP: %a\n\
            CF IP: \"%{{CF-Connecting-IP}}i\"\n\
            Request Line: \"%r\"\n\
            Status Code: %s\n\
            Response Size: %b bytes\n\
            Referer: \"%{{Referer}}i\"\n\
            User-Agent: \"%{{User-Agent}}i\"\n\
            Processing Time: ps:%Tms\n\
            Send Time: send:%Dms",
            server_name // サーバー名（例: "IndexServer"）
        )
        .as_str(),
    )
}