use clap::{Args, Subcommand};

use crate::http::start_server;

#[derive(Args)]
pub struct ServerArguments {
    #[clap(subcommand)]
    command: Option<ServerCommands>,
    args: Option<String>,
}

#[derive(Subcommand)]
pub enum ServerCommands {
    Start {
        host: Option<String>,
        port: Option<u16>,
    },
}

pub async fn server_command(args: &ServerArguments) {
    match &args.command {
        Some(commands) => match commands {
            ServerCommands::Start { host, port } => start_server(host, port).await,
        },
        None => default(),
    }
}

fn default() {
    println!("Running the default command from the example module");
}
