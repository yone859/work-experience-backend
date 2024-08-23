use std::env;
use std::time::Duration;
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database, DatabaseConnection, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use crate::database::entities::{todos, users};
use crate::database::entities::prelude::Todos;
use sea_orm::ColumnTrait;

pub struct DbInfo {
    db_connection:DbConn
  }

  impl DbInfo {
    pub async fn new() -> Self {
        let db_connection:DatabaseConnection  = establish_connection().await.expect("connection error!");
        DbInfo {db_connection}
    }
    pub fn get_db_connection(&self) -> &DbConn {
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

pub async fn insert_user(db: &DbConn) -> Result<users::Model, DbErr> {
    // ユーザーアクティブモデルを生成
    let user = users::ActiveModel {
        id: ActiveValue::NotSet,
        name: Set("John Smith".to_string())
    };

    // insert
    let user: users::Model = user.insert(db).await?;

    Ok(user)
}

pub async fn insert_todos(db: &DbConn, user: &users::Model) -> Result<todos::Model, DbErr> {
    let todo = todos::ActiveModel {
        id: ActiveValue::NotSet,
        title: Set("Test".to_string()),
        description: Set("".to_string()),
        done: Default::default(),
        created_by: Set(user.id),
        updated_by:  Set(user.id),
    };
    let todo: todos::Model = todo.insert(db).await?;

    let todo2 = todos::ActiveModel {
        id: ActiveValue::NotSet,
        title: Set("Test2".to_string()),
        description: Set("This todo is a dummy.".to_string()),
        done: Default::default(),
        created_by: Set(user.id),
        updated_by:  Set(user.id),
    };
    let _ = todo2.insert(db).await?;

    let todo3 = todos::ActiveModel {
        id: ActiveValue::NotSet,
        title: Set("Test3".to_string()),
        description: Set("test data.".to_string()),
        done: Set(true),
        created_by: Set(user.id),
        updated_by:  Set(user.id),
    };
    let _ = todo3.insert(db).await?;

    Ok(todo)
}

pub async fn select_todo(db: &DbConn, todo: todos::Model) -> Result<Option<todos::Model>, DbErr> {
    // ID 指定の検索
    let selected: Option<todos::Model> = Todos::find_by_id(todo.id).one(db).await?;
    Ok(selected)
}

pub async fn select_todos_by_user(db: &DbConn, user: &users::Model) -> Result<Vec<todos::Model>, DbErr> {
    let selected: Vec<todos::Model> = Todos::find().filter(todos::Column::CreatedBy.eq(user.id)).all(db).await?;
    Ok(selected)
}

pub async fn update_todo(db: &DbConn, todo: todos::Model) -> Result<todos::Model, DbErr> {
    // アクティブモデルを into で生成
    let mut target: todos::ActiveModel = todo.into();

    // 値を書き換える
    target.done = Set(true);

    // update
    let todo: todos::Model = target.update(db).await?;
    Ok(todo)
}

pub async fn delete_todo(db: &DbConn, todo: todos::Model) -> Result<(), DbErr> {
    // アクティブモデルを into で生成
    let target: todos::ActiveModel = todo.into();

    // delete
    let _: DeleteResult = target.delete(db).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connection_works() {
        let result = check_connection().await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn it_works() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .init();

        let db = establish_connection().await.expect("connection error!");

        match insert_user(&db).await {
            Ok(user) => {
                println!("{:?}", user);

                match insert_todos(&db, &user).await {
                    Ok(todo) => {
                        println!("{:?}", todo);

                        //
                        let result = select_todos_by_user(&db, &user).await;
                        println!("{:?}", result);

                        match select_todo(&db, todo).await {
                            Ok(result) => {
                                match result {
                                    None => {}
                                    Some(todo) => {
                                        println!("{:?}", todo);

                                        match update_todo(&db, todo).await {
                                            Ok(todo) => {
                                                println!("{:?}", todo);

                                                let result = delete_todo(&db, todo).await;
                                                assert!(result.is_ok());
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}