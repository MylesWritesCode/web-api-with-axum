use sea_orm_migration::{
    prelude::*,
    sea_orm::prelude::{DateTimeUtc, Uuid},
};

use entity::organization::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220524_234921_create_organizations_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = DateTimeUtc::from(std::time::SystemTime::now());
        return manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Uuid::new_v4()),
                    )
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::DisplayName).string().not_null())
                    .col(ColumnDef::new(Column::Password).string())
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(now),
                    )
                    .col(
                        ColumnDef::new(Column::ModifiedAt)
                            .date_time()
                            .not_null()
                            .default(now),
                    )
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
