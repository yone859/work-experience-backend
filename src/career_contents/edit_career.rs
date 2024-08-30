use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use crate::{database::db_connection::DbInfo, dev_tool, pjt_support_tool, work_experience, DevTool, PjtSupportTool, WorkExperience};

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
    work_kind: String,

    dev_tool_type1: String,
    dev_tool_name1: String,
    dev_tool_type2: String,
    dev_tool_name2: String,
    dev_tool_type3: String,
    dev_tool_name3: String,
    dev_tool_type4: String,
    dev_tool_name4: String,
    dev_tool_type5: String,
    dev_tool_name5: String,
    dev_tool_type6: String,
    dev_tool_name6: String,
    dev_tool_type7: String,
    dev_tool_name7: String,
    dev_tool_type8: String,
    dev_tool_name8: String,

    pjt_support_tool_type1: String,
    pjt_support_tool_name1: String,
    pjt_support_tool_type2: String,
    pjt_support_tool_name2: String,
    pjt_support_tool_type3: String,
    pjt_support_tool_name3: String,
    pjt_support_tool_type4: String,
    pjt_support_tool_name4: String,
    pjt_support_tool_type5: String,
    pjt_support_tool_name5: String,
    pjt_support_tool_type6: String,
    pjt_support_tool_name6: String,
    pjt_support_tool_type7: String,
    pjt_support_tool_name7: String,
    pjt_support_tool_type8: String,
    pjt_support_tool_name8: String,
}

  impl RequestForm {
    //フォームリクエスト初期化
    pub async fn new(req:axum::Form<RequestForm>) -> RequestForm {
        RequestForm {
            project_no:req.project_no, project_title:req.project_title.to_string(), pjt_content:req.pjt_content.to_string(), participate_date:req.participate_date,
            leave_date:req.leave_date, member_headcount:req.member_headcount, program_language:req.program_language.to_string(), work_kind:req.work_kind.to_string(),

            dev_tool_type1:req.dev_tool_type1.to_string(),dev_tool_name1:req.dev_tool_name1.to_string(),dev_tool_type2:req.dev_tool_type2.to_string(),dev_tool_name2:req.dev_tool_name2.to_string(),
            dev_tool_type3:req.dev_tool_type3.to_string(),dev_tool_name3:req.dev_tool_name3.to_string(),dev_tool_type4:req.dev_tool_type4.to_string(),dev_tool_name4:req.dev_tool_name4.to_string(),
            dev_tool_type5:req.dev_tool_type5.to_string(),dev_tool_name5:req.dev_tool_name5.to_string(),dev_tool_type6:req.dev_tool_type6.to_string(),dev_tool_name6:req.dev_tool_name6.to_string(),
            dev_tool_type7:req.dev_tool_type7.to_string(),dev_tool_name7:req.dev_tool_name7.to_string(),dev_tool_type8:req.dev_tool_type8.to_string(),dev_tool_name8:req.dev_tool_name8.to_string(),
        
            pjt_support_tool_type1:req.pjt_support_tool_type1.to_string(),pjt_support_tool_name1:req.pjt_support_tool_name1.to_string(),pjt_support_tool_type2:req.pjt_support_tool_type2.to_string(),pjt_support_tool_name2:req.pjt_support_tool_name2.to_string(),
            pjt_support_tool_type3:req.pjt_support_tool_type3.to_string(),pjt_support_tool_name3:req.pjt_support_tool_name3.to_string(),pjt_support_tool_type4:req.pjt_support_tool_type4.to_string(),pjt_support_tool_name4:req.pjt_support_tool_name4.to_string(),
            pjt_support_tool_type5:req.pjt_support_tool_type5.to_string(),pjt_support_tool_name5:req.pjt_support_tool_name5.to_string(),pjt_support_tool_type6:req.pjt_support_tool_type6.to_string(),pjt_support_tool_name6:req.pjt_support_tool_name6.to_string(),
            pjt_support_tool_type7:req.pjt_support_tool_type7.to_string(),pjt_support_tool_name7:req.pjt_support_tool_name7.to_string(),pjt_support_tool_type8:req.pjt_support_tool_type8.to_string(),pjt_support_tool_name8:req.pjt_support_tool_name8.to_string()
        }
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
    update_work_experience(&db.get_db_connection(), &req.get_request_form()).await;
}

//対象データ更新
pub async fn update_work_experience(db: &DbConn, request_data: &RequestForm){
    //対象データをSELECT
    let selected_work_experience: Option<work_experience::Model> = WorkExperience::find_by_id(request_data.project_no).one(db).await.expect("database update error!");
    let selected_dev_tool: Option<dev_tool::Model> = DevTool::find_by_id(request_data.project_no).one(db).await.expect("database update error!");
    let selected_pjt_support_tool: Option<pjt_support_tool::Model> = PjtSupportTool::find_by_id(request_data.project_no).one(db).await.expect("database update error!");

    let mut active_work_experience: work_experience::ActiveModel = selected_work_experience.clone().unwrap().into(); 
    let mut active_dev_tool: dev_tool::ActiveModel = selected_dev_tool.clone().unwrap().into(); 
    let mut active_pjt_support_tool: pjt_support_tool::ActiveModel = selected_pjt_support_tool.clone().unwrap().into(); 
    
    //更新データをセット
    active_work_experience.project_title = Set(Some(request_data.project_title.to_string()));
    active_work_experience.pjt_content = Set(Some(request_data.pjt_content.to_string()));
    active_work_experience.participate_date = Set(Some(request_data.participate_date));
    active_work_experience.leave_date = Set(Some(request_data.leave_date));
    active_work_experience.member_headcount = Set(Some(request_data.member_headcount));
    active_work_experience.program_language = Set(Some(request_data.program_language.to_string()));
    active_work_experience.work_kind = Set(Some(request_data.work_kind.to_string()));

    active_dev_tool.dev_tool_type1 = Set(Some(request_data.dev_tool_type1.to_string()));
    active_dev_tool.dev_tool_name1 = Set(Some(request_data.dev_tool_name1.to_string()));
    active_dev_tool.dev_tool_type2 = Set(Some(request_data.dev_tool_type2.to_string()));
    active_dev_tool.dev_tool_name2 = Set(Some(request_data.dev_tool_name2.to_string()));
    active_dev_tool.dev_tool_type3 = Set(Some(request_data.dev_tool_type3.to_string()));
    active_dev_tool.dev_tool_name3 = Set(Some(request_data.dev_tool_name3.to_string()));
    active_dev_tool.dev_tool_type4 = Set(Some(request_data.dev_tool_type4.to_string()));
    active_dev_tool.dev_tool_name4 = Set(Some(request_data.dev_tool_name4.to_string()));
    active_dev_tool.dev_tool_type5 = Set(Some(request_data.dev_tool_type5.to_string()));
    active_dev_tool.dev_tool_name5 = Set(Some(request_data.dev_tool_name5.to_string()));
    active_dev_tool.dev_tool_type6 = Set(Some(request_data.dev_tool_type6.to_string()));
    active_dev_tool.dev_tool_name6 = Set(Some(request_data.dev_tool_name6.to_string()));
    active_dev_tool.dev_tool_type7 = Set(Some(request_data.dev_tool_type7.to_string()));
    active_dev_tool.dev_tool_name7 = Set(Some(request_data.dev_tool_name7.to_string()));
    active_dev_tool.dev_tool_type8 = Set(Some(request_data.dev_tool_type8.to_string()));
    active_dev_tool.dev_tool_name8 = Set(Some(request_data.dev_tool_name8.to_string()));
    
    active_pjt_support_tool.pjt_support_tool_type1 = Set(Some(request_data.pjt_support_tool_type1.to_string()));
    active_pjt_support_tool.pjt_support_tool_name1 = Set(Some(request_data.pjt_support_tool_name1.to_string()));
    active_pjt_support_tool.pjt_support_tool_type2 = Set(Some(request_data.pjt_support_tool_type2.to_string()));
    active_pjt_support_tool.pjt_support_tool_name2 = Set(Some(request_data.pjt_support_tool_name2.to_string()));
    active_pjt_support_tool.pjt_support_tool_type3 = Set(Some(request_data.pjt_support_tool_type3.to_string()));
    active_pjt_support_tool.pjt_support_tool_name3 = Set(Some(request_data.pjt_support_tool_name3.to_string()));
    active_pjt_support_tool.pjt_support_tool_type4 = Set(Some(request_data.pjt_support_tool_type4.to_string()));
    active_pjt_support_tool.pjt_support_tool_name4 = Set(Some(request_data.pjt_support_tool_name4.to_string()));
    active_pjt_support_tool.pjt_support_tool_type5 = Set(Some(request_data.pjt_support_tool_type5.to_string()));
    active_pjt_support_tool.pjt_support_tool_name5 = Set(Some(request_data.pjt_support_tool_name5.to_string()));
    active_pjt_support_tool.pjt_support_tool_type6 = Set(Some(request_data.pjt_support_tool_type6.to_string()));
    active_pjt_support_tool.pjt_support_tool_name6 = Set(Some(request_data.pjt_support_tool_name6.to_string()));
    active_pjt_support_tool.pjt_support_tool_type7 = Set(Some(request_data.pjt_support_tool_type7.to_string()));
    active_pjt_support_tool.pjt_support_tool_name7 = Set(Some(request_data.pjt_support_tool_name7.to_string()));
    active_pjt_support_tool.pjt_support_tool_type8 = Set(Some(request_data.pjt_support_tool_type8.to_string()));
    active_pjt_support_tool.pjt_support_tool_name8 = Set(Some(request_data.pjt_support_tool_name8.to_string()));

    //DB更新
    active_work_experience.update(db).await.expect("database update error!");
    active_dev_tool.update(db).await.expect("database update error!");
    active_pjt_support_tool.update(db).await.expect("database update error!");

}