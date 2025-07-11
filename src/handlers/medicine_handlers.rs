use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use crate::models::{Medicine, ApiMedicine};
use crate::repositories::MedicineRepository;

pub fn medicine_routes() -> Router<Arc<MedicineRepository>> {
    Router::new()
        .route("/medicines", post(create_medicine))
        .route("/medicines", get(get_all_medicines))
        .route("/medicines/:id", get(get_medicine_by_id))
        .route("/medicines/:id", put(update_medicine))
        .route("/medicines/:id", delete(delete_medicine))
        .route("/medicines/:id/addStock", post(add_stock))
}

async fn create_medicine(
    State(repo): State<Arc<MedicineRepository>>,
    Json(api_medicine): Json<ApiMedicine>,
) -> Result<Json<Medicine>, StatusCode> {
    tracing::info!("POST /medicines called");
    
    let id = repo.create(api_medicine).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let medicine = repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(medicine))
}

async fn get_all_medicines(
    State(repo): State<Arc<MedicineRepository>>,
) -> Result<Json<Vec<Medicine>>, StatusCode> {
    tracing::info!("GET /medicines called");
    
    let medicines = repo.get_all().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(medicines))
}

async fn get_medicine_by_id(
    State(repo): State<Arc<MedicineRepository>>,
    Path(id): Path<String>,
) -> Result<Json<Medicine>, StatusCode> {
    tracing::info!("GET /medicines/{}", id);
    
    let medicine = repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(medicine))
}

async fn update_medicine(
    State(repo): State<Arc<MedicineRepository>>,
    Path(id): Path<String>,
    Json(api_medicine): Json<ApiMedicine>,
) -> Result<Json<Medicine>, StatusCode> {
    tracing::info!("PUT /medicines/{}", id);
    
    // Check if medicine exists
    repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    repo.update(&id, api_medicine).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let medicine = repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(medicine))
}

async fn delete_medicine(
    State(repo): State<Arc<MedicineRepository>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("DELETE /medicines/{}", id);
    
    // Check if medicine exists
    repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    repo.delete(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn add_stock(
    State(repo): State<Arc<MedicineRepository>>,
    Path(id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Medicine>, StatusCode> {
    tracing::info!("POST /medicines/{}/addStock", id);
    
    let amount = params.get("amount")
        .ok_or(StatusCode::BAD_REQUEST)?
        .parse::<f64>()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let success = repo.add_stock(&id, amount).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !success {
        return Err(StatusCode::NOT_FOUND);
    }
    
    let medicine = repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(medicine))
} 