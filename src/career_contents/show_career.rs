use crate::database::entities::{self, prelude::*, *};
use crate::common::format_date::{self};

use std::collections::HashMap;
use sea_orm::{ColumnTrait, DbBackend, DbConn, DbErr, EntityTrait, IntoSimpleExpr, JoinType, QueryFilter, QueryOrder, QuerySelect, QueryTrait, RelationTrait};
use serde::de::value::SeqDeserializer;

use crate::database::db_connection::DbInfo;

pub async fn  show_top() -> axum::Json<serde_json::Value> {
    let db = DbInfo::new().await;

    //自己紹介テーブルSELECT
    let select_res_self_intro:Option<self_introduction::Model> = select_self_introduction(&db.get_db_connection()).await.expect("database select error!");
    //資格テーブルSELECT
    let select_res_quali:Vec<qualification::Model> = select_qualification(&db.get_db_connection()).await.expect("database select error!");
    //案件情報SELECT
    let select_res_work_experience:Vec<entities::work_experience::Model> = select_work_experience(&db.get_db_connection()).await.expect("database select error!");
    //devツールテーブルSELECT
    let select_res_dev_tool:Vec<entities::dev_tool::Model> = select_dev_tool(&db.get_db_connection()).await.expect("database select error!");
    //pjtサポートツールテーブルSELECT
    let select_res_pjt_support_tool:Vec<entities::pjt_support_tool::Model> = select_pjt_support_tool(&db.get_db_connection()).await.expect("database select error!");
    //devツール情報加工
    let dev_tool_str :Vec<Vec<HashMap<String,String>>> = make_dev_tool_str(select_res_dev_tool).await;
    //pjtサポートツール情報加工
    let pjt_support_tool_str :Vec<Vec<HashMap<String,String>>> = make_pjt_support_tool(select_res_pjt_support_tool).await;
    //案件情報加工
    let work_experience_return_data :HashMap<String, Vec<HashMap<String, String>>> = make_work_experience_str(select_res_work_experience, dev_tool_str, pjt_support_tool_str).await;
    //資格情報を文字列に加工する
    let self_intro_return_data :HashMap<String, String> = make_self_intro_str(select_res_self_intro).await;
    //資格情報を文字列に加工する
    let quali_return_data :HashMap<String, Vec<HashMap<String, String>>> = make_quali_str(select_res_quali).await;

    let return_data = (self_intro_return_data, quali_return_data, work_experience_return_data);

    let data = serde_json::json!(return_data);
    axum::Json(data)
}


//自己紹介テーブルSELECT
pub async fn select_self_introduction(db: &DbConn) -> Result<Option<self_introduction::Model>, DbErr> {
    SelfIntroduction::find_by_id(1).one(db).await
}

//資格テーブルSELECT
//Displayカラムが１のものを取得
pub async fn select_qualification(db: &DbConn) -> Result<Vec<qualification::Model>, DbErr> {
    let select_result: Result<Vec<qualification::Model>, DbErr> = Qualification::find()
    .filter(qualification::Column::Display.contains("1"))
    .order_by_asc(qualification::Column::ObtainmentDate)
    .all(db)
    .await;

    select_result    
}

//案件情報SELECT
pub async fn select_work_experience(db: &DbConn) -> Result<Vec<entities::work_experience::Model>, sea_orm::DbErr> {
    work_experience::Entity::find()
        .having(work_experience::Column::ProjectNo.between(1, 2))
        .order_by_asc(work_experience::Column::ParticipateDate)
        .all(db)
        .await    
}

//devツールテーブルSELECT
pub async fn select_dev_tool(db: &DbConn) -> Result<Vec<entities::dev_tool::Model>, sea_orm::DbErr> {
    dev_tool::Entity::find()
        .filter(dev_tool::Column::ProjectNo.between(1, 2))
        .all(db)
        .await                
}

//pjtサポートツールテーブルSELECT
pub async fn select_pjt_support_tool(db: &DbConn) -> Result<Vec<entities::pjt_support_tool::Model>, sea_orm::DbErr> {
    pjt_support_tool::Entity::find()
        .filter(pjt_support_tool::Column::ProjectNo.between(1, 2))
        .all(db)
        .await                
}

//devツール情報加工
pub async fn make_dev_tool_str(select_res_dev_tool:Vec<entities::dev_tool::Model>)->Vec<Vec<HashMap<String,String>>>{
    let mut dev_tool_records: Vec<Vec<HashMap<String,String>>> = Vec::new();

    for dev_tool_record in &select_res_dev_tool{
        let mut sss:Vec<HashMap<String,String>> = Vec::new();
        
        let mut dev_tool_str:HashMap<String,String> = HashMap::new();
        let mut dev_tool_project_no_str:HashMap<String,String> = HashMap::new();

        if let Some(str) = dev_tool_record.dev_tool_type1.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name1.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type2.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name2.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type3.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name3.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type4.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name4.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type5.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name5.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type6.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name6.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type7.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name7.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = dev_tool_record.dev_tool_type8.clone() {
            if !str.is_empty() {dev_tool_str.insert(str , dev_tool_record.dev_tool_name8.as_deref().unwrap_or("").to_string());}
        }
    if let Some(str) = dev_tool_record.project_no.clone() {
        dev_tool_project_no_str.insert(String::from("dev_tool_project_no_str") , str.to_string());
    }
    
    sss.push(dev_tool_project_no_str);
    sss.push(dev_tool_str);
    
    dev_tool_records.push(sss);
    // dev_tool_records.push(dev_tool_str.clone())
    }
    dev_tool_records
}

//pjtサポートツール情報加工
pub async fn make_pjt_support_tool(select_res_pjt_support_tool:Vec<entities::pjt_support_tool::Model>)->Vec<Vec<HashMap<String,String>>>{
    let mut pjt_support_tool_records: Vec<Vec<HashMap<String,String>>> = Vec::new();

    for pjt_support_tool_record in &select_res_pjt_support_tool{
        let mut sss:Vec<HashMap<String,String>> = Vec::new();
        
        let mut pjt_support_tool_str:HashMap<String,String> = HashMap::new();
        let mut pjt_support_tool_project_no_str:HashMap<String,String> = HashMap::new();

        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type1.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name1.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type2.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name2.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type3.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name3.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type4.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name4.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type5.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name5.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type6.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name6.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type7.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name7.as_deref().unwrap_or("").to_string());}
        }
        if let Some(str) = pjt_support_tool_record.pjt_support_tool_type8.clone() {
            if !str.is_empty() {pjt_support_tool_str.insert(str , pjt_support_tool_record.pjt_support_tool_name8.as_deref().unwrap_or("").to_string());}
        }
    if let Some(str) = pjt_support_tool_record.project_no.clone() {
        pjt_support_tool_project_no_str.insert(String::from("pjt_support_tool_project_no_str") , str.to_string());
    }
    
    sss.push(pjt_support_tool_project_no_str);
    sss.push(pjt_support_tool_str);
    
    pjt_support_tool_records.push(sss);
    // dev_tool_records.push(pjt_support_tool_str.clone())
    }
    pjt_support_tool_records
}

//案件情報加工
pub async fn make_work_experience_str(a:Vec<entities::work_experience::Model>,dev_tool_str:Vec<Vec<HashMap<String,String>>>,pjt_support_tool_str:Vec<Vec<HashMap<String,String>>>)->HashMap<String, Vec<HashMap<String, String>>>{
    let mut work_experience_return_data: HashMap<String, Vec<HashMap<String, String>>>  = HashMap::new();

    //案件情報を文字列に加工する
    let mut work_experience_records: Vec<HashMap<String, String>> = Vec::new();
    let mut record: HashMap<String, String>  = HashMap::new();

    for work_experience_record in &a{
        //案件No
        record.insert(String::from("project_no"), work_experience_record.project_no.clone().to_string());
        record.insert(String::from("project_title"), work_experience_record.project_title.clone().unwrap());
        record.insert(String::from("member_headcount"), work_experience_record.member_headcount.clone().unwrap().to_string());

        let formatted_date = format_date::format_yyyy_mm_dd(&work_experience_record.participate_date.unwrap());
        record.insert(String::from("participate_date"), formatted_date);

        let formatted_date = format_date::format_yyyy_mm_dd(&work_experience_record.leave_date.unwrap());
        record.insert(String::from("leave_date"), formatted_date);

        
        record.insert(String::from("program_language"), work_experience_record.program_language.clone().unwrap());
        record.insert(String::from("pjt_content"), work_experience_record.pjt_content.clone().unwrap());
        record.insert(String::from("work_kind"), work_experience_record.work_kind.clone().unwrap());


        for inner_vec in &dev_tool_str {
            let SeqDeserializer:String = work_experience_record.project_no.clone().to_string();
            let Str:String = inner_vec.get(0).unwrap().get("dev_tool_project_no_str").unwrap().clone();
            if SeqDeserializer == Str  {
                record.extend(inner_vec.get(1).unwrap().clone());
            }
        }

        for inner_vec in &pjt_support_tool_str {
            let SeqDeserializer:String = work_experience_record.project_no.clone().to_string();
            let Str:String = inner_vec.get(0).unwrap().get("pjt_support_tool_project_no_str").unwrap().clone();
            if SeqDeserializer == Str  {
                record.extend(inner_vec.get(1).unwrap().clone());
            }
        }
        work_experience_records.push(record.clone());
    }
    work_experience_return_data.insert(String::from("work_experience"), work_experience_records);
    work_experience_return_data
}

//資格情報を文字列に加工する
pub async fn make_self_intro_str(select_res_self_intro:Option<self_introduction::Model>)->HashMap<String, String>{
    let mut self_intro_return_data: HashMap<String, String>  = HashMap::new();
    self_intro_return_data.insert(String::from("self_intro"), select_res_self_intro.unwrap().text);
    self_intro_return_data
}

//資格情報を文字列に加工する
pub async fn make_quali_str(select_res_quali:Vec<qualification::Model>)->HashMap<String, Vec<HashMap<String, String>>>{
    let mut quali_return_data: HashMap<String, Vec<HashMap<String, String>>>  = HashMap::new();
    let mut quali_records: Vec<HashMap<String, String>> = Vec::new();
    for quali_record in &select_res_quali{
        let mut record: HashMap<String, String>  = HashMap::new();
        //資格名
        record.insert(String::from("name"), quali_record.name.clone().unwrap());
        //資格取得年月
        let formatted_date = format_date::format_yyyy_mm_dd(&quali_record.obtainment_date.unwrap());
        record.insert(String::from("obtainment_date"), formatted_date);

        quali_records.push(record);
    }
    quali_return_data.insert(String::from("quali"), quali_records);
    quali_return_data
}