use axum::{extract::Path, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    return Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user));
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: Option<i32>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserResponse {
    user: User,
}

async fn get_users() -> &'static str {
    // @todo Return an array of all users, with pagination
    return "from get_users";
}

async fn get_user(Path(user_id): Path<String>) -> Json<UserResponse> {
    // @todo Get user from db, then return it
    return Json(UserResponse {
        user: User {
            id: Some(user_id.parse::<i32>().unwrap()),
            name: Some("myles".to_string()),
        },
    });
}
