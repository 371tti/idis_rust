use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub status_mes_json_path: String,
    pub status_page_template_path: String,
}