mod database;
mod router;


use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbConn, DbErr, EntityTrait, Set, ActiveValue};

use crate::database::db_connection::establish_connection;

use crate::router::routing::routing;
use crate::database::entities::*;
use crate::database::entities::prelude::Employees;


#[tokio::main]
async fn main() {

    //サーバー起動、ルーター登録
    let _ =  routing();

    let db:DatabaseConnection  = establish_connection().await.expect("connection error!");

    let select_res:Option<employees::Model> = select_employees(&db).await.expect("database select error!");
    let update_res:Option<employees::Model> = update_employees(&db, &select_res).await.expect("database update error!");
    let insert_res:Option<employees::Model> = insert_employees(&db).await.expect("database insert error!");

    println!("{}", select_res.unwrap().id);

    if let Some(name ) = &update_res.unwrap().name {
        println!("{}", name);
    } else {
        println!("Name is None");
    }

    if let Some(name ) = &insert_res.unwrap().name {
        println!("{}", name);
    } else {
        println!("Name is None");
    }
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

pub async fn update_employees(db: &DbConn, employees: &Option<employees::Model>) -> Result<Option<employees::Model>, DbErr> {
    
    let mut active_employees: employees::ActiveModel = employees.clone().unwrap().into(); 
    
    active_employees.name = Set(Some("tanaka".to_string()));

    let active_employees: employees::Model = active_employees.update(db).await?;

    if let Some(name) = &Some(&active_employees.clone()).unwrap().name {
        println!("{}", name);
    } else {
        println!("Name is None");
    }

    Ok(Some(active_employees))
}

pub async fn insert_employees(db: &DbConn) -> Result<Option<employees::Model>, DbErr> {
    // ユーザーアクティブモデルを生成
    let user = employees::ActiveModel {
        id: ActiveValue::NotSet, // auto_increment() なのでセットしない
        name: Set(Some("鈴木".to_string())),
        ..Default::default()
        // department_id: Set(Some(1)),
    };

    // insert
    let user: employees::Model = user.insert(db).await?;

    if let Some(name) = &Some(&user.clone()).unwrap().name {
        println!("{}", name);
    } else {
        println!("Name is None");
    }

    Ok(Some(user))
}
