
// sec/utils/err_set.rs

use serde::ser::{Serialize, Serializer, SerializeStruct};
use chrono::Utc;

#[derive(Debug)]
pub struct ErrState {
    pub process_num: u64,
    pub code: u16,
    pub timestamp: i64,
}

impl ErrState {
    pub fn new(process_num: u64, code: u16) -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        Self {
            process_num,
            code,
            timestamp: utc_timestamp,
        }
    }
}

// カスタムシリアライズの実装
impl Serialize for ErrState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // シリアライズするフィールドをカスタマイズ
        let mut state = serializer.serialize_struct("ErrState", 5)?;
        state.serialize_field("process_num", &self.process_num)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        state.serialize_field("version", "1.0.0")?;
        state.serialize_field("type", &0)?;
        state.end()
    }
}