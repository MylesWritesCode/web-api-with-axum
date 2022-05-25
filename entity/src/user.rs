use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub name: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTimeUtc,
    pub modified_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
        on_update = "Cascade",
        on_delete = "Cascade",
    )]
    Organization,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        return Relation::Organization.def();
    }
}

impl ActiveModelBehavior for ActiveModel {}
