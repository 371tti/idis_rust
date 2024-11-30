use std::collections::HashMap;

use super::messages::Messages;

pub struct Lang {
    pub jp: Option<Messages>,
    pub en: Option<Messages>,
}

impl Lang {
    pub fn pack(&self, lang_type: &str) -> &Messages {
        let result = match lang_type {
            "jp" => self.jp.as_ref().or(self.en.as_ref()).unwrap(),
            "en" => self.en.as_ref().unwrap(),
            _ => self.en.as_ref().unwrap(),
        };
        result
    }
}