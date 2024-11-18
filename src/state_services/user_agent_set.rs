

// Add the necessary imports
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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