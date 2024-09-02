mod database;
mod router;
mod career_contents;
mod common;
mod auth;


use crate::database::entities::{self, prelude::*, *};
use crate::common::format_date::{self};

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbConn, DbErr, EntityTrait, JoinType, QuerySelect, QueryTrait, Set};

use crate::database::db_connection::establish_connection;
use crate::router::routing::running_router;
use crate::database::db_connection::DbInfo;
use sha2::{Sha256, Digest};
use std::fmt::Write;


#[tokio::main]
async fn main() {
    
    //サーバー起動、ルーター登録
    let _ =  running_router().await;

    let db:DatabaseConnection  = establish_connection().await.expect("connection error!");

    let password = "password123";
    let hashed_password = hash_password(password);
    
    println!("元のパスワード: {}", password);
    println!("ハッシュ化されたパスワード: {}", hashed_password);

}

pub async fn select_employees(db: &DbConn) -> Result<Option<employees::Model>, DbErr> {
    let selected: Option<employees::Model> = Employees::find_by_id(1).one(db).await?;
    
// `as_ref()`を使って`Option`内の値を借用する
if let Some(employee) = selected.as_ref() {
    println!("{}", employee.id);
} else {
    println!("Name is None");
}

    Ok(selected)
}

// ハッシュ化関数
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    let mut hash_string = String::new();
    for byte in result {
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    hash_string
}