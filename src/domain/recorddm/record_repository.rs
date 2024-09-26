use crate::domain::recorddm::record_entity::Record;
use std::collections::HashMap;

pub trait RecordRepository {
    fn save(&self, record: Record);
    fn find_by_field(&self, field_name: &str, value: &str) -> Option<&Record>;
}
