use serde::{Deserialize, Serialize};
use chrono::Utc;
use serde_with::{serde_as, DisplayFromStr};
use super::custom_serializers_adapters::TimeStamp;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrState {
    pub process_num: u64,
    pub message: String,
    #[serde_as(as = "TimeStamp")]
    pub timestamp: i64,
    pub from: Option<Box<ErrState>>, // 親エラーを保持するフィールドを追加
}

impl ErrState {
    pub fn new(process_num: u64, message: String, parent: Option<ErrState>) -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        let from = if let Some(parent_val) = parent {
            Some(Box::new(parent_val))
        } else { None };

        Self {
            process_num,
            message,
            timestamp: utc_timestamp,
            from: from,
        }
    }
}