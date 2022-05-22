use axum::{routing::get, Json, Router};

use super::MessageResponse;

pub fn router() -> Router {
    return Router::new().route("/webhooks", get(root));
}

async fn root() -> Json<MessageResponse> {
  return Json(MessageResponse {
    message: "Hello from webhooks route".to_string(),
  });
}
