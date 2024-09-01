use std::env;
use dotenv::dotenv;
use crate::career_contents::{edit_career, show_career, create_career};

pub async  fn running_router()  {
    let app = axum::Router::new()
    
    //トップページ情報取得
    .route("/top", axum::routing::get(show_career::show_top))
    // .route("/other", axum::routing::get(handler_other));
    .route("/", axum::routing::get(handle_index))
    .route("/edit-work-experience", axum::routing::post(edit_career::updata_career))
    .route("/create-work-experience", axum::routing::post(create_career::insert_career));

    

    dotenv().ok();
    axum::Server::bind(&env::var("HOST_NAME").unwrap().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

    async fn handle_index()-> axum::response::Html<String> {
      let tera = tera::Tera::new("templates/*").unwrap();
    
      let mut context = tera::Context::new();
      context.insert("title", "Index page");
      context.insert("message", "これはサンプルです。");
    
      let output = tera.render("index.html", &context);
      axum::response::Html(output.unwrap())
    }

      // async fn handler_other() -> String {
      //   "This is other page...".to_string()
      // }
}