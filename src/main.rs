mod database;
mod sea_orm_demo;
mod common;


use crate::database::entities::{self, prelude::*, *};
use crate::common::format_date::{self};

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbConn, DbErr, EntityTrait, JoinType, QuerySelect, QueryTrait, Set};

use crate::database::db_connection::establish_connection;
use crate::sea_orm_demo::select_demo::select_null_check;
use crate::database::db_connection::DbInfo;
use sha2::{Sha256, Digest};
use std::fmt::Write;


#[tokio::main]
async fn main() {
    
    
    let _ =  select_null_check().await;



}

pub async fn select_employees(db: &DbConn) -> Result<Option<staffs::Model>, DbErr> {
    let selected: Option<staffs::Model> = Staffs::find_by_id(1).one(db).await?;
    
// `as_ref()`を使って`Option`内の値を借用する
if let Some(employee) = selected.as_ref() {
    println!("{}", employee.id);
} else {
    println!("Name is None");
}

    Ok(selected)
}
