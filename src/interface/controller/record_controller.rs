use crate::usecase::recordusecase::{create_record_usecase::CreateRecordUsecase, recordinput::RecordInput};
use crate::interface::presenter::RecordPresenter;

pub struct RecordController<'a> {
    create_record_usecase: &'a CreateRecordUsecase<'a>,
    presenter: &'a dyn RecordPresenter,
}

impl<'a> RecordController<'a> {
    pub fn new(create_record_usecase: &'a CreateRecordUsecase<'a>, presenter: &'a dyn RecordPresenter) -> Self {
        RecordController {
            create_record_usecase,
            presenter,
        }
    }

    pub fn create_record(&self, fields: std::collections::HashMap<String, String>) {
        let input = RecordInput::new(fields);
        let result = self.create_record_usecase.execute(input.fields);
        match result {
            Ok(output) => self.presenter.present(&output),
            Err(err) => println!("Error: {}", err),
        }
    }
}
