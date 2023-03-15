//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: i32,
    pub user_name: String,
    pub user_authority: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_phone: Option<String>,
    pub user_email: String,
    #[sea_orm(column_type = "Text")]
    pub user_address: String,
    pub create_time: DateTime,
    pub article_num: i32,
    pub album_num: i32,
    pub pwd_hash: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
