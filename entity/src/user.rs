use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTimeUtc,
    pub modified_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        return super::organization_users::Relation::Organization.def();
    }

    fn via() -> Option<RelationDef> {
        return Some(super::organization_users::Relation::User.def().rev());
    }
}

impl ActiveModelBehavior for ActiveModel {}
