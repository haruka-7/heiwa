use axum::response::Json;
use serde_json::{json, Value};

pub async fn create() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

pub async fn author() -> Json<Value> {
    Json(json!({ "data": 42 }))
}
