use std::collections::HashMap;

use crate::database::db_connection::DbInfo;
use crate::database::entities::{self, prelude::*, *};
use sea_orm::*;


pub async  fn select_null_check()  {
    let db = DbInfo::new().await;

    let result_select:Vec<staffs::Model>
        = select_staff(&db.get_db_con()).await.expect("database select error!");

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


pub async fn select_staff(db: &DbConn) -> Result<Vec<staffs::Model>, DbErr> {
    let selected: Vec<staffs::Model> = 
        staffs::Entity::find()
        .all(db)
        .await.unwrap();

    Ok(selected)
}


