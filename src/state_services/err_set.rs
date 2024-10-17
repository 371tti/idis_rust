// sec/utils/err_set.rs

use serde::ser::{Serialize, Serializer, SerializeStruct};
use chrono::Utc;

#[derive(Debug)]
pub struct ErrState {
    pub process_num: u64,
    pub code: u16,
    pub timestamp: i64,
    pub parent: Option<Box<ErrState>>, // 親エラーを保持するフィールドを追加
    pub is_root: bool,
}

impl ErrState {
    pub fn new(process_num: u64, code: u16, parent: Option<ErrState>) -> Self {
        let utc_timestamp = Utc::now().timestamp_millis();
        let parent_new = if let Some(mut parent_val) = parent {
            parent_val.is_root = false;
            Some(parent_val)
        } else {
            None
        };

        Self {
            process_num,
            code,
            timestamp: utc_timestamp,
            parent: parent_new.map(Box::new), // 親エラーをBoxでラップして保持
            is_root: true,
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
        let mut state = serializer.serialize_struct("ErrState", 6)?;
        state.serialize_field("process_num", &self.process_num)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("timestamp", &self.timestamp)?;
        
        if let Some(ref parent) = self.parent {
            state.serialize_field("parent", parent)?;
        } else {
            state.serialize_field("parent", &None::<ErrState>)?;
        }

        if self.is_root {
            state.serialize_field("version", "1.0.0")?;
            state.serialize_field("type", &0)?;
        }
        state.end()
    }
}