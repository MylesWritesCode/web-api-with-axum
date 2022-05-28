use axum::{extract::Path, http::StatusCode, response::Json, routing::get, Router};
use sea_orm::prelude::{DateTimeUtc, Uuid};
use serde::{Deserialize, Serialize};

use entity::organization;
use organization::Entity as UserEntity;

use super::Response;

pub fn router() -> Router {
    return Router::new()
        .route("/orgs", get(get_organizations).post(create_organization))
        .route(
            "/orgs/:id",
            get(get_organization)
                .post(update_organization)
                .delete(delete_organization),
        );
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Organization {
    id: Uuid,
    name: String,
    display_name: String,
    created_at: DateTimeUtc,
    modified_at: DateTimeUtc,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DbResponse {
    organization: Organization,
}

async fn get_organizations() {
    // @todo Return an array of all users, with pagination
    todo!()
}

async fn get_organization(Path(org_id): Path<String>) -> Response<DbResponse> {
    // @todo Get user from db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        organization: Organization {
            id: Uuid::new_v4(),
            name: "".to_string(),
            display_name: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn create_organization() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        organization: Organization {
            id: Uuid::new_v4(),
            name: "".to_string(),
            display_name: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn update_organization() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        organization: Organization {
            id: Uuid::new_v4(),
            name: "".to_string(),
            display_name: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}

async fn delete_organization() -> Response<DbResponse> {
    // @todo Create user in db, then return it

    let created = DateTimeUtc::from(std::time::SystemTime::now());
    let modified = DateTimeUtc::from(std::time::SystemTime::now());

    let res = DbResponse {
        organization: Organization {
            id: Uuid::new_v4(),
            name: "".to_string(),
            display_name: "".to_string(),
            created_at: created,
            modified_at: modified,
        },
    };

    return (StatusCode::OK, Json(res));
}
