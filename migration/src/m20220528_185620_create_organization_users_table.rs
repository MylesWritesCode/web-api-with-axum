use sea_orm_migration::prelude::*;

use entity::organization_users::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220528_185620_create_organization_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        return manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::OrganizationId).uuid().not_null())
                    .col(ColumnDef::new(Column::UserId).uuid().not_null())
                    .to_owned(),
            )
            .await;
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        return manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await;
    }
}
