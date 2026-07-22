use axum::response::Json;
use serde_json::{Value, json};

pub async fn root() -> Json<Value> {
    Json(json!({"hello": "world"}))
}
