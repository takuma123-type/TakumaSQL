use crate::domain::tabledm::table_entity::Table;

pub trait TableRepository {
    fn create_table(&self, name: &str) -> Table;
    fn find_table(&self, name: &str) -> Option<&Table>;
    fn delete_table(&self, name: &str);
}
