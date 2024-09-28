use crate::domain::recorddm::{record_entity::Record, record_repository::RecordRepository};
use std::collections::HashMap;

pub struct InMemoryRecordRepository {
    records: HashMap<String, Record>,
}

impl InMemoryRecordRepository {
    pub fn new() -> Self {
        InMemoryRecordRepository {
            records: HashMap::new(),
        }
    }
}

impl RecordRepository for InMemoryRecordRepository {
    fn save(&mut self, record: Record) {
        self.records.insert(record.get_field("name").unwrap().clone(), record);
    }

    fn find_by_field(&self, field_name: &str, value: &str) -> Option<&Record> {
        for record in self.records.values() {
            if let Some(field_value) = record.get_field(field_name) {
                if field_value == value {
                    return Some(record);
                }
            }
        }
        None
    }
}
