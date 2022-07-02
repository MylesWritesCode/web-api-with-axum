use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use axum::{routing::get, Json, Router};
use serde::Serialize;

use super::MessageResponse;

pub fn router() -> Router {
    return Router::new()
        .route("/diagnostics/simple", get(get_simple))
        .route("/diagnostics/expensive", get(get_expensive));
}

async fn get_simple() -> Json<MessageResponse> {
    return Json(MessageResponse {
        message: "This is a simple response from an endpoint that does practically nothing"
            .to_string(),
    });
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpensiveResponse {
    person: Person,
    hash: u64,
}

#[derive(Hash, Serialize)] 
struct Person {
    id: u32,
    name: String,
    city: String,
}

async fn get_expensive() -> Json<ExpensiveResponse> {
    let person: Person = Person { id: 59, name: "Myles".to_string(), city: "Honolulu".to_string() };
    
    let mut hasher = DefaultHasher::new();
    person.hash(&mut hasher);

    return Json(ExpensiveResponse {
        person,
        hash: hasher.finish(),
    });
}
