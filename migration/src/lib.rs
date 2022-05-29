pub use sea_orm_migration::prelude::*;

mod m20220523_220125_create_users_table;
mod m20220524_234921_create_organizations_table;
mod m20220528_185620_create_organization_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220523_220125_create_users_table::Migration),
            Box::new(m20220524_234921_create_organizations_table::Migration),
            Box::new(m20220528_185620_create_organization_users_table::Migration),
        ]
    }
}
