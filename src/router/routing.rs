use std::env;
use dotenv::dotenv;
use crate::career_contents::show_career;

pub async  fn running_router()  {
    let app = axum::Router::new()
    
    //トップページ情報取得
    .route("/top", axum::routing::get(show_career::show_top));
    // .route("/other", axum::routing::get(handler_other));

    dotenv().ok();
    axum::Server::bind(&env::var("HOST_NAME").unwrap().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

      // async fn handler_other() -> String {
      //   "This is other page...".to_string()
      // }
}