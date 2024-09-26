use crate::usecase::recordusecase::recordoutput::RecordOutput;

pub trait RecordPresenter {
    fn present(&self, output: &RecordOutput);
}

pub struct RecordPresenterImpl;

impl RecordPresenter for RecordPresenterImpl {
    fn present(&self, output: &RecordOutput) {
        println!("Record operation result: success={}, message={}", output.success, output.message);
    }
}
