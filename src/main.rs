mod database;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbConn, DbErr, EntityTrait, Set};

use crate::database::db_connection::establish_connection;
use crate::database::entities::*;
use crate::database::entities::prelude::Employees;

#[tokio::main]
async fn main() {
    let db:DatabaseConnection  = establish_connection().await.expect("connection error!");

    let select_res:Option<employees::Model> = select_employees(&db).await.expect("database select error!");
    let update_res:Option<employees::Model> = update_employees(&db, &select_res).await.expect("database update error!");

    println!("{}", select_res.unwrap().id);

    if let Some(name ) = &update_res.unwrap().name {
        println!("{}", name);
    } else {
        println!("Name is None");
    }
}

pub async fn select_employees(db: &DbConn) -> Result<Option<employees::Model>, DbErr> {
    let selected: Option<employees::Model> = Employees::find_by_id(1).one(db).await?;
    Ok(selected)
}

pub async fn update_employees(db: &DbConn, employees: &Option<employees::Model>) -> Result<Option<employees::Model>, DbErr> {
    
    let mut active_user: employees::ActiveModel = employees.clone().unwrap().into(); 
    
    active_user.name = Set(Some("佐藤".to_string()));

    let active_user: employees::Model = active_user.update(db).await?;
    Ok(Some(active_user))
}
