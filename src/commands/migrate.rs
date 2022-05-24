use clap::{Args, Subcommand};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

#[derive(Args)]
pub struct MigrateArguments {
    #[clap(subcommand)]
    command: Option<MigrateCommands>,
}

#[derive(Subcommand)]
pub enum MigrateCommands {
    Up,
    Down,
}

pub async fn migrate_command(args: &MigrateArguments, connection: &DatabaseConnection) {
    match &args.command {
        Some(command) => match command {
            MigrateCommands::Up => up(&connection, None).await,
            MigrateCommands::Down => down(&connection, None).await,
        },
        None => up(&connection, None).await,
    }
}

async fn up(connection: &DatabaseConnection, steps: Option<u32>) {
    match Migrator::up(&connection, steps).await {
        Ok(_) => println!("Migrations up ran successfully"),
        Err(e) => println!("Migrations failed: {}", e),
    }
}

async fn down(connection: &DatabaseConnection, steps: Option<u32>) {
    match Migrator::down(&connection, steps).await {
        Ok(_) => println!("Migrations down ran successfully"),
        Err(e) => println!("Migrations failed: {}", e),
    }
}
