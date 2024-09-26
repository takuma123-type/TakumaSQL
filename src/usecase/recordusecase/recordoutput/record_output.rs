pub struct RecordOutput {
    pub success: bool,
    pub message: String,
}

impl RecordOutput {
    pub fn new(success: bool, message: String) -> Self {
        RecordOutput { success, message }
    }
}
