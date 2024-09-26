use std::collections::HashMap;

pub struct RecordInput {
    pub fields: HashMap<String, String>,
}

impl RecordInput {
    pub fn new(fields: HashMap<String, String>) -> Self {
        RecordInput { fields }
    }
}
