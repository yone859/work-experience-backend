use std::env;
use dotenv::dotenv;

pub async  fn routing()  {
    let app = axum::Router::new()

    .route("/", axum::routing::get(handler_top))
    .route("/other", axum::routing::get(handler_other));

    dotenv().ok();
    axum::Server::bind(&env::var("HOST_NAME").unwrap().parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();



    async fn handler_top() -> String {
        "Hello, World!".to_string()
      }
      async fn handler_other() -> String {
        "This is other page...".to_string()
      }

}