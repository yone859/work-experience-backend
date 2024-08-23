use crate::database::entities::{*,prelude::*};
use crate::common::format_date::{self};

use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder};

use crate::database::db_connection::DbInfo;

pub async fn  show_top() -> axum::Json<serde_json::Value> {
    let db = DbInfo::new().await;

    //自己紹介テーブルSELECT
    let select_res_self_intro:Option<self_introduction::Model> = select_self_introduction(&db.get_db_connection()).await.expect("database select error!");
    //資格テーブルSELECT
    let select_res_quali:Vec<qualification::Model> = select_qualification(&db.get_db_connection()).await.expect("database select error!");

    //自己紹介情報と資格情報ベクタ
    let mut self_intro_return_data: Vec<String> = Vec::new();
    let mut quali_return_data: Vec<Vec<String>> = Vec::new();

    //返却用ベクタ
    let mut return_data: Vec<Vec<String>> = Vec::new();

    //自己紹介情報を文字列に加工する
    match &select_res_self_intro {
        Some(self_introduction) => self_intro_return_data.push(self_introduction.text.clone()),
        None => println!("No name available"),
    }
    //////if letで書く場合
    // if let Some(self_introduction) = select_res_self_intro.as_ref() {
    //     println!("{}", self_introduction.text);
    // } else {
    //     println!("Name is None");
    // }

    //資格情報を文字列に加工する
    for quali_record in &select_res_quali{
        let mut record: Vec<String>  = Vec::new();
        
        match &quali_record.name {
            Some(name) => record.push(name.clone()),
            None => println!("No name available"),
        }

        match &quali_record.obtainment_date {
            Some(obtainment_date) => {
                let formatted_date = format_date::format_yyyy_mm_dd(obtainment_date);
                record.push(formatted_date)
            },
            None => println!("No obtainment_date available"),
        }
        quali_return_data.push(record);
    }

    //自己紹介情報と資格情報を返却用ベクタに格納
    return_data.push(self_intro_return_data.clone());
    return_data.extend(quali_return_data);

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

