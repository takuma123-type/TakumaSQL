use src::usecase::tableusecase::{CreateTableUsecase, tableinput::CreateTableInput};
use src::infra::repository::InMemoryTableRepository;
use src::interface::presenter::TablePresenterImpl;

#[test]
fn test_create_table() {
    let mut repository = InMemoryTableRepository::new();
    let presenter = TablePresenterImpl;
    
    let mut create_table_usecase = CreateTableUsecase::new(&mut repository);
    let input = CreateTableInput::new("users".to_string());
    let result = create_table_usecase.execute(input);
    
    assert!(result.is_ok(), "Table creation failed");
    let output = result.unwrap();
    assert_eq!(output.success, true);
    assert_eq!(output.message, "Table 'users' created successfully");
}
