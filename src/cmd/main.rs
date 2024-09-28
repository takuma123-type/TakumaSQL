use crate::infra::router::{new_table_router, new_record_router};

fn main() {
    new_table_router();  // テーブルの作成
    new_record_router("John Doe").expect("Failed to create record");  // レコードの作成
}
