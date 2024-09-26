use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Record {
    pub fields: HashMap<String, String>,
}

impl Record {
    pub fn new(fields: HashMap<String, String>) -> Self {
        Record { fields }
    }

    pub fn get_field(&self, field_name: &str) -> Option<&String> {
        self.fields.get(field_name)
    }

    pub fn set_field(&mut self, field_name: String, value: String) {
        self.fields.insert(field_name, value);
    }
}
