use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use crate::models::{MedicineSchedule, ApiMedicineSchedule, DailyScheduleWithDate};
use crate::repositories::{MedicineRepository, MedicineScheduleRepository};

pub fn schedule_routes() -> Router<Arc<(MedicineRepository, MedicineScheduleRepository)>> {
    Router::new()
        .route("/schedules", post(create_schedule))
        .route("/schedules", get(get_all_schedules))
        .route("/schedules/:id", get(get_schedule_by_id))
        .route("/schedules/:id", put(update_schedule))
        .route("/schedules/:id", delete(delete_schedule))
        .route("/schedules/daily/:date", get(get_daily_schedule))
}

async fn create_schedule(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
    Json(api_schedule): Json<ApiMedicineSchedule>,
) -> Result<Json<MedicineSchedule>, StatusCode> {
    tracing::info!("POST /schedules called");
    
    let (_, schedule_repo) = &*repos;
    let id = schedule_repo.create(api_schedule).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let schedule = schedule_repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(schedule))
}

async fn get_all_schedules(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
) -> Result<Json<Vec<MedicineSchedule>>, StatusCode> {
    tracing::info!("GET /schedules called");
    
    let (_, schedule_repo) = &*repos;
    let schedules = schedule_repo.get_all().await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(schedules))
}

async fn get_schedule_by_id(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
    Path(id): Path<String>,
) -> Result<Json<MedicineSchedule>, StatusCode> {
    tracing::info!("GET /schedules/{}", id);
    
    let (_, schedule_repo) = &*repos;
    let schedule = schedule_repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(schedule))
}

async fn update_schedule(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
    Path(id): Path<String>,
    Json(api_schedule): Json<ApiMedicineSchedule>,
) -> Result<Json<MedicineSchedule>, StatusCode> {
    tracing::info!("PUT /schedules/{}", id);
    
    let (_, schedule_repo) = &*repos;
    // Check if schedule exists
    schedule_repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    schedule_repo.update(&id, api_schedule).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let schedule = schedule_repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(schedule))
}

async fn delete_schedule(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("DELETE /schedules/{}", id);
    
    let (_, schedule_repo) = &*repos;
    // Check if schedule exists
    schedule_repo.get_by_id(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    
    schedule_repo.delete(&id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::NO_CONTENT)
}

async fn get_daily_schedule(
    State(repos): State<Arc<(MedicineRepository, MedicineScheduleRepository)>>,
    Path(date): Path<String>,
) -> Result<Json<DailyScheduleWithDate>, StatusCode> {
    tracing::info!("GET /schedules/daily/{}", date);
    
    let (medicine_repo, schedule_repo) = &*repos;
    let daily_schedule = schedule_repo.get_daily_schedule_with_date(&date, medicine_repo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(daily_schedule))
} 