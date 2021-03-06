use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};

use entity::user;
use user::Entity as UserEntity;

use super::Response;

pub fn router() -> Router {
    return Router::new()
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).post(update_user).delete(delete_user),
        );
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    id: Uuid,
    org_id: Option<Uuid>,
    name: String,
    display_name: String,
    email: String,
    created_at: DateTimeUtc,
    modified_at: DateTimeUtc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DbResponse {
    user: User,
}

async fn get_users() {
    // @todo Return an array of all users, with pagination
    todo!()
}

async fn get_user(Path(user_id): Path<String>) -> Response<DbResponse> {
    // @todo Get user from db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        user: User {
            id: Uuid::new_v4(),
            org_id: Some(Uuid::new_v4()),
            name: "".to_string(),
            display_name: "".to_string(),
            email: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn create_user() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        user: User {
            id: Uuid::new_v4(),
            org_id: Some(Uuid::new_v4()),
            name: "".to_string(),
            display_name: "".to_string(),
            email: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn update_user() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        user: User {
            id: Uuid::new_v4(),
            org_id: Some(Uuid::new_v4()),
            name: "".to_string(),
            display_name: "".to_string(),
            email: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn delete_user() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        user: User {
            id: Uuid::new_v4(),
            org_id: Some(Uuid::new_v4()),
            name: "".to_string(),
            display_name: "".to_string(),
            email: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}
