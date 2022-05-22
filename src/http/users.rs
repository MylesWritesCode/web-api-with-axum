use axum::{routing::get, Router};

pub fn router() -> Router {
    return Router::new().route("/users", get(get_users));
}

async fn get_users() -> &'static str {
    return "from get_users";
}
