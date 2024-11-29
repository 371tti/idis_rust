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
    pub parent: Option<Box<ErrState>>, // 親エラーを保持するフィールドを追加
    pub is_root: bool,
}

impl ErrState {
    pub fn new(process_num: u64, message: String, parent: Option<ErrState>) -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        let parent_new = if let Some(mut parent_val) = parent {
            parent_val.is_root = false;
            Some(parent_val)
        } else {
            None
        };

        Self {
            process_num,
            message,
            timestamp: utc_timestamp,
            parent: parent_new.map(Box::new), // 親エラーをBoxでラップして保持
            is_root: true,
        }
    }
}