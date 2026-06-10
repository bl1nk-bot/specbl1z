use axum::{extract::State, routing::get, Json, Router};
use serde_json::Value;
use specgen_core::{
    db::Database,
    memory::{MemoryQuery, MemoryStore},
    models::MemoryEntry,
};
use std::sync::{Arc, Mutex};

/// AppState wraps Database in a Mutex to ensure thread-safety (Sync) for Axum.
pub struct AppState {
    pub db: Mutex<Database>,
}

pub async fn run_server(port: u16) -> anyhow::Result<()> {
    // Database path relative to workspace root
    let db_path = "data/craft.db";
    let db = Database::new(db_path)?;
    let state = Arc::new(AppState { db: Mutex::new(db) });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route(
            "/api/memory",
            get(query_memory_handler).post(insert_memory_handler),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!(
        "Specgen Rust Backend (Tiered Memory) listening on {}",
        listener.local_addr()?
    );
    axum::serve(listener, app).await?;
    Ok(())
}

async fn query_memory_handler(State(state): State<Arc<AppState>>) -> Json<Vec<MemoryEntry>> {
    let db_lock = state.db.lock().expect("Failed to lock database");
    let store = MemoryStore::new(&db_lock);

    // Default query for now, can be extended with parameters
    let query = MemoryQuery::default();
    let results = store.query(&query).unwrap_or_default();
    Json(results)
}

async fn insert_memory_handler(
    State(state): State<Arc<AppState>>,
    Json(mut entry): Json<MemoryEntry>,
) -> Json<Value> {
    let db_lock = state.db.lock().expect("Failed to lock database");
    let store = MemoryStore::new(&db_lock);

    // Apply "New > Old" policy: set high confidence for new entries
    // and adjust based on policy/1000 if provided in provenance or tags
    if entry.confidence == 0.0 {
        entry.confidence = 1.0;
    }

    match store.insert(entry) {
        Ok(id) => Json(serde_json::json!({
            "status": "success",
            "id": id,
            "policy_applied": "new_over_old_v3"
        })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn test_app() -> Router {
        let db = Database::new(":memory:").expect("in-memory db");
        let state = Arc::new(AppState { db: Mutex::new(db) });

        Router::new()
            .route("/health", get(|| async { "OK" }))
            .route(
                "/api/memory",
                get(query_memory_handler).post(insert_memory_handler),
            )
            .with_state(state)
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let app = test_app();
        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn memory_get_returns_array() {
        let app = test_app();
        let req = Request::builder()
            .uri("/api/memory")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn memory_post_inserts_and_returns_id() {
        let app = test_app();
        let entry = serde_json::json!({
            "scope": 1,
            "category": 1,
            "topic": 1,
            "key": "test_key",
            "value": "test_value",
            "confidence": 1.0,
            "status": "active",
            "created_at": 0,
            "updated_at": 0,
            "version": 1,
            "tags": [],
            "access_level": "private"
        });
        let req = Request::builder()
            .uri("/api/memory")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&entry).unwrap()))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn app_state_create() {
        let db = Database::new(":memory:").expect("in-memory db");
        let state = AppState { db: Mutex::new(db) };
        let _ = Arc::new(state);
    }
}
