mod database;
use sea_orm::{DatabaseConnection, DbConn, DbErr, EntityTrait};

use crate::database::db_connection::establish_connection;
use crate::database::entities::*;
use crate::database::entities::prelude::Employees;

#[tokio::main]
async fn main() {
    let db:DatabaseConnection  = establish_connection().await.expect("connection error!");
    let select_res:Option<employees::Model> = select_employees(&db).await.expect("database select error!");
    println!("{}", select_res.unwrap().id);
}

pub async fn select_employees(db: &DbConn) -> Result<Option<employees::Model>, DbErr> {
    let selected: Option<employees::Model> = Employees::find_by_id(1).one(db).await?;
    Ok(selected)
}
