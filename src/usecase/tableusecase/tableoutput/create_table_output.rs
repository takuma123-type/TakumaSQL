pub struct CreateTableOutput {
    pub success: bool,
    pub message: String,
}

impl CreateTableOutput {
    pub fn new(success: bool, message: String) -> Self {
        CreateTableOutput { success, message }
    }
}
