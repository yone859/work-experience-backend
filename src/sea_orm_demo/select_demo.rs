use std::collections::HashMap;

use crate::database::db_connection::DbInfo;
use crate::database::entities::{self, prelude::*, *};
use sea_orm::*;


pub async  fn select_null_check()  {
    let db = DbInfo::new().await;

    let start_id:i32 = 1;
    let end_id:i32 = 2;

    let result_select:Vec<staffs::Model>
        = select_staff(start_id, end_id, &db.get_db_con()).await.expect("database select error!");

    let mut staff_list:Vec<String> = Vec::new();
    for staff in &result_select{        
        if let Some(str) = staff.remark.clone() {
            if !str.is_empty() {
                staff_list.push(staff.staff_name.clone().unwrap());
            }
        }
    }

    println!("備考欄に記載があるスタッフ：{:?}", staff_list);
}


pub async fn select_staff(from: i32, by: i32, db: &DbConn) -> Result<Vec<staffs::Model>, DbErr> {
    let selected: Vec<staffs::Model> = 
        staffs::Entity::find()
        .having(staffs::Column::Id.between(from, by))
        .all(db)
        .await.unwrap();

    Ok(selected)
}


