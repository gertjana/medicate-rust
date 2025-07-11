use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use std::sync::Arc;
use crate::models::{DosageHistory, ApiDosageHistory};
use crate::repositories::DosageHistoryRepository;

pub fn dosage_history_routes() -> Router<Arc<DosageHistoryRepository>> {
    Router::new()
        .route("/dosage-history", post(create_dosage_history))
        .route("/dosage-history", get(get_all_dosage_history))
        .route("/dosage-history/:id", delete(delete_dosage_history))
}

async fn create_dosage_history(
    State(repo): State<Arc<DosageHistoryRepository>>,
    Json(api_history): Json<ApiDosageHistory>,
) -> Result<Json<DosageHistory>, StatusCode> {
    tracing::info!("POST /dosage-history called");
    
    let id = repo.create(api_history).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let history = repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(history))
}

async fn get_all_dosage_history(
    State(repo): State<Arc<DosageHistoryRepository>>,
) -> Result<Json<Vec<DosageHistory>>, StatusCode> {
    tracing::info!("GET /dosage-history called");
    
    let histories = repo.get_all().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(histories))
}

async fn delete_dosage_history(
    State(repo): State<Arc<DosageHistoryRepository>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("DELETE /dosage-history/{}", id);
    
    // Check if history exists
    repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    repo.delete(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
} 