use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::{database::db_connection::DbInfo, work_experience, WorkExperience};

#[derive(Serialize, Deserialize)]

//フォームリクエスト
pub struct RequestForm {
    project_no: i32,
    project_title: String,
    pjt_content: String,
    participate_date: NaiveDate,
    leave_date: NaiveDate,
    member_headcount: i32,
    program_language: String,
    work_kind: String
}

  impl RequestForm {
    //フォームリクエスト初期化
    pub async fn new(req:axum::Form<RequestForm>) -> RequestForm {
        RequestForm {project_no:req.project_no, project_title:req.project_title.to_string(), pjt_content:req.pjt_content.to_string(), participate_date:req.participate_date,
            leave_date:req.leave_date, member_headcount:req.member_headcount, program_language:req.program_language.to_string(), work_kind:req.work_kind.to_string()}
    }
    //フォームリクエスト取得
    pub fn get_request_form(&self) -> &RequestForm {
        &self
    }
  }

pub async fn updata_career(axum::Form(request_form): axum::Form<RequestForm>){
    //リクエスト情報を格納
    let req = RequestForm::new(axum::Form(request_form)).await;

    //DB接続情報
    let db = DbInfo::new().await;
    let update_res:Option<work_experience::Model> = update_work_experience(&db.get_db_connection(), &req.get_request_form()).await.expect("database update error!");
}

//対象データ更新
pub async fn update_work_experience(db: &DbConn, request_data: &RequestForm) -> Result<Option<work_experience::Model>, DbErr> {
    //対象データをSELECT
    let selected_work_experience: Option<work_experience::Model> = WorkExperience::find_by_id(request_data.project_no).one(db).await?;
    let mut active_work_experience: work_experience::ActiveModel = selected_work_experience.clone().unwrap().into(); 
    
    //更新データをセット
    active_work_experience.project_title = Set(Some(request_data.project_title.to_string()));
    active_work_experience.pjt_content = Set(Some(request_data.pjt_content.to_string()));
    active_work_experience.participate_date = Set(Some(request_data.participate_date));
    active_work_experience.leave_date = Set(Some(request_data.leave_date));
    active_work_experience.member_headcount = Set(Some(request_data.member_headcount));
    active_work_experience.program_language = Set(Some(request_data.program_language.to_string()));
    active_work_experience.work_kind = Set(Some(request_data.work_kind.to_string()));

    //DB更新
    let active_work_experience: work_experience::Model = active_work_experience.update(db).await?;

    Ok(Some(active_work_experience))
}