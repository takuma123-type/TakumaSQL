use src::usecase::recordusecase::{CreateRecordUsecase, FetchRecordsUsecase};
use src::domain::tabledm::table_entity::Table;
use std::collections::HashMap;

#[test]
fn test_create_and_fetch_record() {
    let mut table = Table::new("users");

    let mut create_record_usecase = CreateRecordUsecase::new(&mut table);
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), "John Doe".to_string());
    let result = create_record_usecase.execute(fields);
    assert!(result.is_ok(), "Record insertion failed");

    let fetch_records_usecase = FetchRecordsUsecase::new(&table);
    let records = fetch_records_usecase.execute().unwrap();
    assert_eq!(records.len(), 1, "Expected one record in the table");
    assert_eq!(records[0].get_field("name").unwrap(), "John Doe");
}
