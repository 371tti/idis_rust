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
    pub from: Vec<Box<ErrState>>, // 親エラーを保持するフィールドを追加
}

impl ErrState {
    pub fn new(process_num: u64, message: String, parent: Option<ErrState>) -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        let mut from: Vec<Box<ErrState>> = Vec::new();
        if let Some(parent_val) = parent {
            from.push(Box::new(parent_val));
        };;

        Self {
            process_num,
            message,
            timestamp: utc_timestamp,
            from: from,
        }
    }

    pub fn push_err(&mut self, parent: ErrState) {
        self.from.push(Box::new(parent));
    }
}