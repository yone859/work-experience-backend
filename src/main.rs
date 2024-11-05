mod database;
mod router;
mod career_contents;
mod common;
mod auth;


use crate::database::entities::{prelude::*, *};

use crate::router::routing::running_router;


#[tokio::main]
async fn main() {
    
    //サーバー起動、ルーター登録
    let _ =  running_router().await;
}

// pub async fn select_employees(db: &DbConn) -> Result<Option<employees::Model>, DbErr> {
//     let selected: Option<employees::Model> = Employees::find_by_id(1).one(db).await?;
    
// // `as_ref()`を使って`Option`内の値を借用する
// if let Some(employee) = selected.as_ref() {
//     println!("{}", employee.id);
// } else {
//     println!("Name is None");
// }

//     Ok(selected)
// }
