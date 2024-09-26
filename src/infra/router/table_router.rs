use crate::interface::controller::TableController;
use crate::interface::presenter::TablePresenterImpl;
use crate::usecase::tableusecase::create_table_usecase::CreateTableUsecase;

pub fn new_table_router() {
    let presenter = TablePresenterImpl;
    let usecase = CreateTableUsecase::new();
    let controller = TableController::new(&usecase, &presenter);

    controller.create_table("users".to_string());
}
