use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
    env
};

use axum::{routing::get, Router};
use clap::{Args, Subcommand};

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

async fn start_server(host: &Option<String>, port: &Option<u16>) {

    let host: Ipv4Addr = match host {
        Some(v) => Ipv4Addr::from_str(v).unwrap_or(Ipv4Addr::LOCALHOST),
        None => Ipv4Addr::LOCALHOST,
    };
    let port = port.unwrap_or(3000);

    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from((host, port));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}
