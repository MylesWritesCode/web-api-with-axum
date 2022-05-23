/**
 * This is the main driver code for the starter.
 * Run with `cargo run` or `<project_name>` to see the auto-generated help text.
 */
use clap::{Parser, Subcommand};

mod commands;
use commands::server::*;

mod settings;
use settings::Settings;

mod http;

#[derive(clap::Parser)]
#[clap(name = "Starter kit")]
#[clap(author = "Myles <myles@themapletree.io>")]
#[clap(version = "0.1.0")]
#[clap(about = "This is a starter kit for creating a CLI application with Rust.")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Server(ServerArguments),
}

#[tokio::main]
async fn main() {
    zenv::Zenv::new(".env", false).configure().ok();

    let cli = Cli::parse();
    let settings = Settings::new();

    match settings {
        Ok(_) => println!("Sucessfully loaded settings!"),
        Err(e) => println!("Error loading settings: {:?}", e),
    }

    match &cli.command {
        Some(command) => match command {
            Commands::Server(args) => server_command(args).await,
        },
        None => default_command(),
    };
}

fn default_command() {
    println!("Running the default command from the top level");
}