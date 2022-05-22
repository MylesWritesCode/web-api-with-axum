use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::{
    net::{Ipv4Addr, SocketAddr},
    str::FromStr,
};

mod users;

pub async fn start_server(host: &Option<String>, port: &Option<u16>) {
    let host: Ipv4Addr = match host {
        Some(v) => Ipv4Addr::from_str(v).unwrap_or(Ipv4Addr::LOCALHOST),
        None => Ipv4Addr::LOCALHOST,
    };
    let port = port.unwrap_or(3000);

    let app = Router::new()
        .route("/", get(root))
        .route("/diagnostics", get(diagnostics))
        .merge(users::router());

    let addr = SocketAddr::from((host, port));
    println!("Listening on {}", addr);

    let _server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await;
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageResponse {
    message: String,
}

async fn root() -> Json<MessageResponse> {
    let res = MessageResponse {
        message: "Hello, world!".to_string(),
    };

    return Json(res);
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticsResponse {
    uptime: String,
    version: String,
}

async fn diagnostics() -> Json<DiagnosticsResponse> {
    let res = Json(DiagnosticsResponse {
        uptime: "2000".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    });

    return res;
}
