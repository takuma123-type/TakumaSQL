use crate::usecase::tableusecase::tableoutput::CreateTableOutput;

pub trait TablePresenter {
    fn present(&self, output: &CreateTableOutput);
}

pub struct TablePresenterImpl;

impl TablePresenter for TablePresenterImpl {
    fn present(&self, output: &CreateTableOutput) {
        println!("Table creation result: success={}, message={}", output.success, output.message);
    }
}
