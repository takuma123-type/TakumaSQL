pub struct TableController;

impl TableController {
    pub fn new() -> Self {
        TableController
    }

    pub fn create_table(&self, table_name: String) {
        println!("Table created: {}", table_name);
    }
}

pub struct RecordController;

impl RecordController {
    pub fn new() -> Self {
        RecordController
    }

    pub fn create_record(&self, record_name: String) {
        println!("Record created: {}", record_name);
    }
}