

// Add the necessary imports
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Clone)]
pub struct UserAgent {
    pub browser_name: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub category: Option<String>,
    pub vendor: Option<String>,
    pub browser_type: Option<String>,
}

impl UserAgent {
    pub fn new(
        browser_name: Option<String>,
        browser_version: Option<String>,
        os: Option<String>,
        os_version: Option<String>,
        category: Option<String>,
        vendor: Option<String>,
        browser_type: Option<String>,
    ) -> Self {
        UserAgent {
            browser_name,
            browser_version,
            os,
            os_version,
            category,
            vendor,
            browser_type,
        }
    }
}

// カスタムシリアライズの実装
impl Serialize for UserAgent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // シリアライズするフィールドをカスタマイズ
        let mut state = serializer.serialize_struct("UserAgent", 9)?;
        state.serialize_field("browser_name", &self.browser_name)?;
        state.serialize_field("browser_version", &self.browser_version)?;
        state.serialize_field("os", &self.os)?;
        state.serialize_field("os_version", &self.os_version)?;
        state.serialize_field("category", &self.category)?;
        state.serialize_field("vendor", &self.vendor)?;
        state.serialize_field("browser_type", &self.browser_type)?;
        state.serialize_field("version", &Some("1.0.0".to_string()))?;  // 追加フィールド
        state.serialize_field("type", &Some(9))?; // 追加フィールド
        state.end()
    }
}