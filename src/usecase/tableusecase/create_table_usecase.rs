use crate::domain::tabledm::table_entity::Table;
use crate::domain::tabledm::table_repository::TableRepository;
use crate::usecase::tableusecase::tableinput::CreateTableInput;
use crate::usecase::tableusecase::tableoutput::CreateTableOutput;
use std::error::Error;

pub struct CreateTableUsecase<'a> {
    table_repository: &'a mut dyn TableRepository,
}

impl<'a> CreateTableUsecase<'a> {
    pub fn new(table_repository: &'a mut dyn TableRepository) -> Self {
        CreateTableUsecase { table_repository }
    }

    pub fn execute(&mut self, input: CreateTableInput) -> Result<CreateTableOutput, Box<dyn Error>> {
        let table = Table::new(&input.name);
        self.table_repository.create_table(&input.name);

        Ok(CreateTableOutput {
            success: true,
            message: format!("Table '{}' created successfully", input.name),
        })
    }
}
