pub struct CreateTableInput {
    pub name: String,
}

impl CreateTableInput {
    pub fn new(name: String) -> Self {
        CreateTableInput { name }
    }
}
