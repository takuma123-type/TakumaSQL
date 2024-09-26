use crate::domain::tabledm::table_entity::Table;
use crate::domain::recorddm::record_entity::Record;
use std::error::Error;

pub struct FetchRecordsUsecase<'a> {
    table: &'a Table,
}

impl<'a> FetchRecordsUsecase<'a> {
    pub fn new(table: &'a Table) -> Self {
        FetchRecordsUsecase { table }
    }

    pub fn execute(&self) -> Result<&Vec<Record>, Box<dyn Error>> {
        Ok(self.table.select_all())
    }
}
