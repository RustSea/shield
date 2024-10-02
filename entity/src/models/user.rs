//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    #[sea_orm(unique)]
    pub email: String,
    pub email_verified_at: Option<DateTimeWithTimeZone>,
    pub phone: Option<String>,
    pub image: Option<String>,
    pub two_factor_enabled_at: Option<DateTimeWithTimeZone>,
    pub password_hash: Option<String>,
    pub is_temp_password: bool,
    pub locked_at: Option<DateTimeWithTimeZone>,
    pub realm_id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::realm::Entity",
        from = "Column::RealmId",
        to = "super::realm::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Realm,
    #[sea_orm(has_many = "super::resource_group::Entity")]
    ResourceGroup,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::realm::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Realm.def()
    }
}

impl Related<super::resource_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceGroup.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}
