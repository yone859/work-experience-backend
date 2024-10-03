use std::env;
use axum::{response::{IntoResponse, Redirect},routing};
use http::{header::HeaderValue};
use dotenv::dotenv;
use crate::career_contents::{edit_career, show_career, create_career};
use crate::auth::{sign_up, login};
use tower_http::cors::{CorsLayer,Any};

pub async  fn running_router()  {
    let cors = CorsLayer::new()
    .allow_origin(Any)  // 全てのオリジンを許可
    .allow_methods(Any)  // 全てのHTTPメソッドを許可
    .allow_headers(Any); // 全てのヘッダーを許可

    let app = axum::Router::new()
    
    //トップページ情報取得
    // .route("/sign-up", routing::post(sign_up::create_login_info))
    // .route("/login", routing::post(login::check_password))
    .route("/top", routing::get(show_career::show_top))
    // .route("/", routing::get(handle_index))
    // .route("/edit-work-experience", routing::post(edit_career::updata_career))
    // .route("/create-work-experience", routing::post(create_career::insert_career))
    .layer(cors);

    dotenv().ok();
    axum::Server::bind(&env::var("HOST_NAME").unwrap().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();

    // async fn handle_index()-> axum::response::Html<String> {
    //   let tera = tera::Tera::new("templates/*").unwrap();
    
    //   let mut context = tera::Context::new();
    //   context.insert("title", "Index page");
    //   context.insert("message", "これはサンプルです。");
    
    //   let output = tera.render("index.html", &context);
    //   axum::response::Html(output.unwrap())
    // }
}