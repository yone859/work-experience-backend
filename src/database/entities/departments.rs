//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "departments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub departments_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::staffs::Entity")]
    Staffs,
}

impl Related<super::staffs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staffs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
