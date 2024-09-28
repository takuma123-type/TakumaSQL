use crate::domain::tabledm::{table_entity::Table, table_repository::TableRepository};
use std::collections::HashMap;

pub struct InMemoryTableRepository {
    tables: HashMap<String, Table>,
}

impl InMemoryTableRepository {
    pub fn new() -> Self {
        InMemoryTableRepository {
            tables: HashMap::new(),
        }
    }
}

impl TableRepository for InMemoryTableRepository {
    fn create_table(&self, name: &str) -> Table {
        let table = Table::new(name);
        self.tables.insert(name.to_string(), table.clone());
        table
    }

    fn find_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }

    fn delete_table(&self, name: &str) {
        self.tables.remove(name);
    }
}
