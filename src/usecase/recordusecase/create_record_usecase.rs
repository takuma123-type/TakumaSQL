use crate::domain::recorddm::record_entity::Record;
use crate::domain::recorddm::record_repository::RecordRepository;
use std::collections::HashMap;
use std::error::Error;

pub struct CreateRecordUsecase<'a> {
    repository: &'a mut dyn RecordRepository,
}

impl<'a> CreateRecordUsecase<'a> {
    pub fn new(repository: &'a mut dyn RecordRepository) -> Self {
        CreateRecordUsecase { repository }
    }

    pub fn execute(&mut self, fields: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        let record = Record::new(fields);
        self.repository.save(record);
        Ok(())
    }
}
