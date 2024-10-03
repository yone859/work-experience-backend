use std::{io,collections::HashMap, fmt::Write};

use axum::{Json, Form, response::{IntoResponse, Response}, http::StatusCode};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sha2::{Digest, Sha256};
use crate::{common::format_date, database::{db_connection::DbInfo, entities::auth}, dev_tool, pjt_support_tool, work_experience, DevTool, PjtSupportTool, WorkExperience};

#[derive(Serialize, Deserialize)]

//フォームリクエスト
pub struct RequestForm {
    login_id: String,
    password: String,
}

  impl RequestForm {
    //フォームリクエスト初期化
    pub async fn new(req:Form<RequestForm>) -> RequestForm {
        RequestForm {
            login_id:req.login_id.to_string(), password:req.password.to_string()
    }
    }
    //フォームリクエスト取得
    pub fn get_request_form(&self) -> &RequestForm {
        &self
    }
  }

  #[derive(Serialize)]
  pub struct LoginResponse {
    // status: String,
    message: String,
}

pub async fn check_password(Json(payload): Json<RequestForm>) ->Response {

    //リクエスト情報を格納
    let req:RequestForm = RequestForm::new(axum::Form(payload)).await;
    let request_data:&RequestForm=&req.get_request_form();

    //DB接続情報
    let db_info:DbInfo = DbInfo::new().await;
    let db:&DbConn = db_info.get_db_connection();
    
    let mut auth_info:Option<auth::Model> =  get_auth_info(&db, &request_data).await;

    let mut salt:String = "".to_string(); 
    let mut hash:String = "".to_string(); 
    let mut expire_date:String = "".to_string(); 

    match auth_info {
        Some(record) => {
            salt = record.salt.unwrap();
            hash = record.hash.unwrap();
            let formatted_date = format_date::format_yyyy_mm_dd(&record.expire_date.unwrap());
            expire_date = formatted_date;
        },
        None => {
            let response = LoginResponse {
                message: "Operation was successful".to_string(),
            };
            // return (StatusCode::UNAUTHORIZED, Json(response))
           return (StatusCode::UNAUTHORIZED, Json(response)).into_response();
        }
    }
    
    if salt + &generate_hash_password(&request_data.password) == hash{

        let response = LoginResponse {
            message: "Operation was successful".to_string(),
        };
        (StatusCode::OK, Json(response)).into_response()


    } else {
        
        let response = LoginResponse {
            message: "Operation was successful".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(response)).into_response()

    }


}

//ログインIDを条件にauthテーブルを検索
async fn get_auth_info(db:&DbConn, request_data: &RequestForm) -> Option<auth::Model> {
    let result = auth::Entity::find()
        .filter(auth::Column::LoginId.eq(request_data.login_id.to_string()))
        .one(db)  // `one`メソッドを使って1つのレコードを取得
        .await
        .ok()?;

    result
}

// ハッシュ化関数
fn generate_hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    let mut hash_string = String::new();
    for byte in result {
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    hash_string
}
