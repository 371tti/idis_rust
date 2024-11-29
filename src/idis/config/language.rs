use std::{collections::HashMap, path::Path};

use serde_json::Value;

use super::messages::Messages;

#[derive(Eq, Hash, PartialEq)]
enum LangPack {
    EN,
    JP,
}
pub struct Lang {
    pub pack: HashMap<LangPack, Messages>
}

impl Lang {
    pub fn new(lang_json: &Value) -> Self {
        let mut pack = HashMap::new();

        let en = lang_json["en"].clone();

        Self {
            pack
        }
    }

    pub fn pack(&self, lang: LangPack) -> &Messages {
        match self.pack.get(&lang) {
            Some(messages) => messages,
            None => self.pack.get(&LangPack::EN).unwrap()
        }
    }
}