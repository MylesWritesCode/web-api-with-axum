use sea_orm_migration::prelude::*;

use entity::user::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220523_220125_create_users_table"
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
                    .col(
                        ColumnDef::new(Column::Id)
                            .uuid()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Column::Name).text().not_null())
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