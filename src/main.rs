mod database;
mod sea_orm_demo;
mod common;


use crate::database::entities::{self, prelude::*, *};
use crate::common::format_date::{self};

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbBackend, DbConn, DbErr, EntityTrait, JoinType, QuerySelect, QueryTrait, Set};

use crate::database::db_connection::establish_connection;
use crate::sea_orm_demo::select_demo::{*};
use crate::database::db_connection::DbInfo;
use sha2::{Sha256, Digest};
use std::fmt::Write;


#[tokio::main]
async fn main() {
        
    let _ =  select_null_check().await;
    let _ =  select_inner_jouin().await;

}
