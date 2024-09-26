use crate::domain::recorddm::record_entity::Record;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub records: VecDeque<Record>,
}

impl Table {
    pub fn new(name: &str) -> Self {
        Table {
            name: name.to_string(),
            records: VecDeque::new(),
        }
    }

    pub fn insert_record(&mut self, record: Record) {
        self.records.push_back(record);
    }

    pub fn select_all(&self) -> &VecDeque<Record> {
        &self.records
    }
}
