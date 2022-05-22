use axum::{routing::get, Json, Router, extract::Path};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    return Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user));
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserResponse {
    user: User,
}

async fn get_users() -> &'static str {
    return "from get_users";
}

async fn get_user(Path(user_id): Path<String>) -> Json<UserResponse> {
    todo!()
}
