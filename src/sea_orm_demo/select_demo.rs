use std::collections::HashMap;

use crate::database::db_connection::DbInfo;
use crate::database::entities::{self, prelude::*, *};
use sea_orm::*;


pub async  fn select_null_check()  {
    let db = DbInfo::new().await;

    let from_staff_id:i32 = 1;
    let by_staff_id:i32 = 2;

    let result_select:Vec<entities::staffs::Model> = select_staff(from_staff_id, by_staff_id, &db.get_db_connection()).await.expect("database select error!");

    for dev_tool_record in &result_select{
        let mut sss:Vec<HashMap<String,String>> = Vec::new();
        
        let mut dev_tool_str:HashMap<String,String> = HashMap::new();
        let mut dev_tool_project_no_str:HashMap<String,String> = HashMap::new();

        if let Some(str) = dev_tool_record.remark.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.remark.as_deref().unwrap_or("").to_string());}
        }

    }

}


pub async fn select_staff(from: i32, by: i32, db: &DbConn) -> Result<Vec<entities::staffs::Model>, DbErr> {
    let selected: Vec<entities::staffs::Model> = 
        staffs::Entity::find()
        .having(staffs::Column::Id.between(from, by))
        .all(db)
        .await.unwrap();

    Ok(selected)
}


