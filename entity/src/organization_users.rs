use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "organization_users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub organization_id: Uuid,
    #[sea_orm(primary_key)]
    pub user_id: Uuid,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrganizationId",
        to = "super::organization::Column::Id"
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
        on_update = "Cascade",
        on_delete = "Cascade",

    )]
    User,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Organization.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
