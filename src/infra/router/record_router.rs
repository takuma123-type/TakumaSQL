use crate::interface::controller::RecordController;
use crate::interface::presenter::RecordPresenterImpl;
use crate::usecase::recordusecase::create_record_usecase::CreateRecordUsecase;
use std::collections::HashMap;

pub fn new_record_router() {
    let mut fields = HashMap::new();
    fields.insert("name".to_string(), "John Doe".to_string());

    let presenter = RecordPresenterImpl;
    let usecase = CreateRecordUsecase::new();
    let controller = RecordController::new(&usecase, &presenter);
s
    controller.create_record(fields);
}
