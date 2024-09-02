use std::fmt::Write;

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, EntityTrait, QueryOrder, QuerySelect, Set};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sha2::{Digest, Sha256};
use crate::{database::{db_connection::DbInfo, entities::auth}, dev_tool, pjt_support_tool, work_experience, DevTool, PjtSupportTool, WorkExperience};

#[derive(Serialize, Deserialize)]

//フォームリクエスト
pub struct RequestForm {
    login_id: String,
    password: String,
    expire_date: NaiveDate,
}

  impl RequestForm {
    //フォームリクエスト初期化
    pub async fn new(req:axum::Form<RequestForm>) -> RequestForm {
        RequestForm {
            // project_no:req.project_no,
            login_id:req.login_id.to_string(), password:req.password.to_string(), expire_date:req.expire_date
        }
    }
    //フォームリクエスト取得
    pub fn get_request_form(&self) -> &RequestForm {
        &self
    }
  }

pub async fn create_login_info(axum::Form(request_form): axum::Form<RequestForm>){
    //リクエスト情報を格納
    let req:RequestForm = RequestForm::new(axum::Form(request_form)).await;
    let request_data:&RequestForm=&req.get_request_form();

    //DB接続情報
    let db_info:DbInfo = DbInfo::new().await;
    let db:&DbConn = db_info.get_db_connection();

    let hash_password:String = generate_hash_password(&request_data.password.to_string());
    let random_str:String = generate_random_string(10);
    let register_str:String = random_str.clone() + &hash_password;

    // ユーザーアクティブモデルを生成
    let auth = auth::ActiveModel {
        id: ActiveValue::NotSet, // auto_increment() なのでセットしない
        login_id: Set(request_data.login_id.to_string()),
        salt: Set(Some(random_str)),
        hash: Set(Some(register_str)),
        expire_date: Set(Some(request_data.expire_date)),
        ..Default::default()
    };

    auth.insert(db).await.expect("database update error!");
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

fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}