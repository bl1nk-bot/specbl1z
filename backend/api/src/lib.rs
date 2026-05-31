use axum::{
    routing::{get, post},
    Json, Router, extract::State,
};
use std::sync::{Arc, Mutex};
use specgen_core::{db::Database, memory::{MemoryStore, MemoryQuery}, models::MemoryEntry};
use serde_json::Value;

/// AppState wraps Database in a Mutex to ensure thread-safety (Sync) for Axum.
pub struct AppState {
    pub db: Mutex<Database>,
}

pub async fn run_server() -> anyhow::Result<()> {
    // Database path relative to workspace root
    let db_path = "backend/craft.db";
    let db = Database::new(db_path)?;
    let state = Arc::new(AppState { db: Mutex::new(db) });

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/memory", get(query_memory_handler).post(insert_memory_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Specgen Rust Backend (Tiered Memory) listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn query_memory_handler(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<MemoryEntry>> {
    let db_lock = state.db.lock().expect("Failed to lock database");
    let store = MemoryStore::new(&*db_lock);
    
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
    let store = MemoryStore::new(&*db_lock);

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
