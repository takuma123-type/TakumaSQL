use crate::usecase::tableusecase::{create_table_usecase::CreateTableUsecase, tableinput::CreateTableInput};
use crate::interface::presenter::TablePresenter;

pub struct TableController<'a> {
    create_table_usecase: &'a CreateTableUsecase<'a>,
    presenter: &'a dyn TablePresenter,
}

impl<'a> TableController<'a> {
    pub fn new(create_table_usecase: &'a CreateTableUsecase<'a>, presenter: &'a dyn TablePresenter) -> Self {
        TableController {
            create_table_usecase,
            presenter,
        }
    }

    pub fn create_table(&self, name: String) {
        let input = CreateTableInput::new(name);
        let result = self.create_table_usecase.execute(input);
        match result {
            Ok(output) => self.presenter.present(&output),
            Err(err) => println!("Error: {}", err),
        }
    }
}
