use std::env;
use std::time::Duration;
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database, DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use sea_orm::ColumnTrait;

pub struct DbInfo {
    db_connection:DbConn
  }

  impl DbInfo {
    pub async fn new() -> Self {
        let db_connection:DatabaseConnection  = establish_connection().await.expect("connection error!");
        DbInfo {db_connection}
    }
    pub fn get_db_con(&self) -> &DbConn {
        &self.db_connection
    }
  }

pub async  fn check_connection() -> Result<(), DbErr> {
    // DB接続のためのコネクションを生成
    let db = establish_connection().await?;

    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("panic!");
    println!("OK");
    Ok(())
}

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not found.");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    //  DB接続のためのコネクションを生成
    Database::connect(opt).await
}
