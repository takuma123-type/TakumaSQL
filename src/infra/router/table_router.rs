use crate::interface::controller::TableController;
use crate::interface::presenter::TablePresenterImpl;
use crate::usecase::tableusecase::create_table_usecase::CreateTableUsecase;
use crate::interface::repository::InMemoryTableRepository;

pub fn new_table_router() {
    // Presenter、Repository、Usecase、Controllerを初期化
    let presenter = TablePresenterImpl;
    let mut repository = InMemoryTableRepository::new();
    let mut usecase = CreateTableUsecase::new(&mut repository);
    let controller = TableController::new(&usecase, &presenter);

    // テーブルを作成
    controller.create_table("users".to_string());
}
