use crate::domain::tabledm::table_entity::Table;
use crate::domain::recorddm::record_entity::Record;
use std::error::Error;

pub struct GetRecordUsecase<'a> {
    table: &'a Table,
}

impl<'a> GetRecordUsecase<'a> {
    pub fn new(table: &'a Table) -> Self {
        GetRecordUsecase { table }
    }

    pub fn execute(&self, field_name: &str, value: &str) -> Result<Option<&Record>, Box<dyn Error>> {
        for record in self.table.select_all() {
            if let Some(field_value) = record.get_field(field_name) {
                if field_value == value {
                    return Ok(Some(record));
                }
            }
        }
        Ok(None)
    }
}
