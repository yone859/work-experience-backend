use crate::database::entities::{self, prelude::*, *};
use crate::common::format_date::{self};

use std::collections::HashMap;
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::database::db_connection::DbInfo;


pub async fn  get_basic_info() -> axum::Json<serde_json::Value> {
    let db = DbInfo::new().await;

    //自己紹介テーブルSELECT
    let select_res_self_intro:Option<self_introduction::Model> = select_self_introduction(&db.get_db_connection()).await.expect("database select error!");
    //資格テーブルSELECT
    let select_res_quali:Vec<qualification::Model> = select_qualification(&db.get_db_connection()).await.expect("database select error!");

    //資格情報を文字列に加工する
    let self_intro_return_data :HashMap<String, String> = make_self_intro_str(select_res_self_intro).await;
    //資格情報を文字列に加工する
    let quali_return_data :HashMap<String, Vec<HashMap<String, String>>> = make_quali_str(select_res_quali).await;


    let return_data = (self_intro_return_data, quali_return_data);

    let data = serde_json::json!(return_data);
    axum::Json(data)
}

pub async fn  show_top(axum::extract::Query(params):
axum::extract::Query<HashMap<String, String>>) -> axum::Json<serde_json::Value> {
    let db = DbInfo::new().await;

    //案件情報加工
    let work_experience_return_data :HashMap<String, Vec<HashMap<String, String>>> = make_work_experience_str(&db.get_db_connection(), &params).await;

    let return_data = (work_experience_return_data);

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

//案件情報加工
pub async fn make_work_experience_str(db: &DbConn, params: &HashMap<String, String>)->HashMap<String, Vec<HashMap<String, String>>>{
    let mut work_experience_return_data: HashMap<String, Vec<HashMap<String, String>>>  = HashMap::new();

    //全案件情報が格納されるベクタ
    let mut work_experience_records: Vec<HashMap<String, String>> = Vec::new();

    let current: i32 = params["current_record"].parse::<i32>().unwrap();
    let next_fetch_record: i32 = params["next_fetch_record"].parse::<i32>().unwrap();

    // INNER JOINで職歴、devtool取得
    let work_experience_with_dev_tool:Vec<(dev_tool::Model, Option<work_experience::Model>)> = dev_tool::Entity::find()
        .find_also_related(work_experience::Entity) 
        .having(work_experience::Column::ProjectNo.between(current, next_fetch_record))
        .order_by_desc(work_experience::Column::LeaveDate)
        .all(db)
        .await.expect("database select error!");

    // INNER JOINでdevsupporttool取得
    let pjt_support_tool:Vec<(pjt_support_tool::Model, Option<work_experience::Model>)> = pjt_support_tool::Entity::find()
        .find_also_related(work_experience::Entity) 
        .having(work_experience::Column::ProjectNo.between(current, next_fetch_record))
        .order_by_desc(work_experience::Column::LeaveDate)
        .all(db)
        .await.expect("database select error!");


    
    let mut counter:i32 = 0;
    for (dev_tool, work_experience) in work_experience_with_dev_tool {
        //１案件ごとの職歴
        let mut record: HashMap<String, String>  = HashMap::new();

        if let Some(work_experience) = work_experience {
            record.insert(String::from("project_no"), work_experience.project_no.to_string());
            record.insert(String::from("project_title"), work_experience.project_title.clone().unwrap());
            record.insert(String::from("member_headcount"), work_experience.member_headcount.unwrap().to_string());
            record.insert(String::from("participate_date"), format_date::format_yyyy_mm_dd(&work_experience.participate_date.unwrap()));
            record.insert(String::from("leave_date"), format_date::format_yyyy_mm_dd(&work_experience.leave_date.unwrap()));
            record.insert(String::from("program_language"), work_experience.program_language.clone().unwrap());
            record.insert(String::from("pjt_content"), work_experience.pjt_content.clone().unwrap());
            record.insert(String::from("work_kind"), work_experience.work_kind.clone().unwrap());
        }

        //devtool成形
        let tool_types = vec![
            &dev_tool.dev_tool_type1,
            &dev_tool.dev_tool_type2,
            &dev_tool.dev_tool_type3,
            &dev_tool.dev_tool_type4,
            &dev_tool.dev_tool_type5,
            &dev_tool.dev_tool_type6,
            &dev_tool.dev_tool_type7,
            &dev_tool.dev_tool_type8,
        ];
        
        let tool_names = vec![
            &dev_tool.dev_tool_name1,
            &dev_tool.dev_tool_name2,
            &dev_tool.dev_tool_name3,
            &dev_tool.dev_tool_name4,
            &dev_tool.dev_tool_name5,
            &dev_tool.dev_tool_name6,
            &dev_tool.dev_tool_name7,
            &dev_tool.dev_tool_name8,
        ];
    
        for (tool_type, tool_name) in tool_types.iter().zip(tool_names.iter()) {
            if let Some(t_type) = tool_type {
                if !t_type.is_empty() {
                    if let Some(t_name) = tool_name {
                        if !t_name.is_empty() {
                            record.insert(t_type.clone(), t_name.clone());
                        }
                    }
                }
            }
        }

        //devsupporttool成形
        if let Some((pjt_support_tool, _)) = pjt_support_tool.get(counter as usize) {
            let tool_types = vec![
                &pjt_support_tool.pjt_support_tool_type1,
                &pjt_support_tool.pjt_support_tool_type2,
                &pjt_support_tool.pjt_support_tool_type3,
                &pjt_support_tool.pjt_support_tool_type4,
                &pjt_support_tool.pjt_support_tool_type5,
                &pjt_support_tool.pjt_support_tool_type6,
                &pjt_support_tool.pjt_support_tool_type7,
                &pjt_support_tool.pjt_support_tool_type8,
            ];
        
            let tool_names = vec![
                &pjt_support_tool.pjt_support_tool_name1,
                &pjt_support_tool.pjt_support_tool_name2,
                &pjt_support_tool.pjt_support_tool_name3,
                &pjt_support_tool.pjt_support_tool_name4,
                &pjt_support_tool.pjt_support_tool_name5,
                &pjt_support_tool.pjt_support_tool_name6,
                &pjt_support_tool.pjt_support_tool_name7,
                &pjt_support_tool.pjt_support_tool_name8,
            ];
        
            for (tool_type, tool_name) in tool_types.iter().zip(tool_names.iter()) {
                if let Some(t_type) = tool_type {
                    if !t_type.is_empty() {
                        if let Some(t_name) = tool_name {
                            if !t_name.is_empty() {
                                record.insert(t_type.clone(), t_name.clone());
                            }
                        }
                    }
                }
            }
        }
        counter+= 1;
        
        //1案件の情報を詰める
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