pub struct I18n {
    locale: String,
    translations: std::collections::HashMap<String, String>,
}

impl I18n {
    pub fn new(locale: &str) -> Self {
        I18n {
            locale: locale.to_string(),
            translations: std::collections::HashMap::new(),
        }
    }

    pub fn add_translation(&mut self, key: &str, value: &str) {
        self.translations.insert(key.to_string(), value.to_string());
    }

    pub fn translate(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
}
