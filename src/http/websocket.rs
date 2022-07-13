use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use futures::{SinkExt, StreamExt};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

// #[derive(Clone)]
// struct Message {
//     username: String,
//     addr: SocketAddr,
//     message: String,
// }

struct ChatroomState {
    peer_map: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

pub fn router() -> Router {
    let peer_map = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(128);

    let state = Arc::new(ChatroomState { peer_map, tx });

    return Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(state));
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<ChatroomState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ChatroomState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If a username is not taken, fill username string
            check_username(&state, &mut username, &name);

            if !username.is_empty() {
                break;
            } else {
                sender
                    .send(Message::Text("Username is already taken".to_string()))
                    .await;

                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();

    let message = format!("{} has joined the chat", username);
    state.tx.send(message);

    // Receive broadcast messages and send text message to our client
    let mut send_task = tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass to receiving
    let tx = state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            tx.send(format!("{}: {}", name, text));
        }
    });

    // If any task exits, abort the other
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Connection closed message
    let message = format!("{} has left the chat.", username);
    state.tx.send(message);
    state.peer_map.lock().unwrap().remove(&username);
}

fn check_username(state: &ChatroomState, string: &mut String, name: &str) {
    let mut peer_map = state.peer_map.lock().unwrap();

    if !peer_map.contains(name) {
        peer_map.insert(name.to_owned());

        string.push_str(name);
    }
}
