use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

use crate::models::{ApiMedicine, ApiMedicineSchedule, ApiDosageHistory};
use crate::repositories::{MedicineRepository, MedicineScheduleRepository, DosageHistoryRepository};

pub async fn create_test_medicine_repo() -> Arc<MedicineRepository> {
    Arc::new(
        MedicineRepository::new("redis://localhost:6379", "test:medicine:".to_string()).unwrap()
    )
}

pub async fn create_test_schedule_repos() -> Arc<(MedicineRepository, MedicineScheduleRepository)> {
    Arc::new((
        MedicineRepository::new("redis://localhost:6379", "test:medicine:".to_string()).unwrap(),
        MedicineScheduleRepository::new("redis://localhost:6379", "test:schedule:".to_string()).unwrap()
    ))
}

pub async fn create_test_dosage_history_repo() -> Arc<DosageHistoryRepository> {
    Arc::new(
        DosageHistoryRepository::new("redis://localhost:6379", "test:dosage:".to_string()).unwrap()
    )
}

pub async fn make_request<B>(app: axum::Router, method: &str, uri: &str, body: Option<B>) -> Response
where
    B: serde::Serialize,
{
    let mut request_builder = Request::builder()
        .method(method)
        .uri(uri);

    let body = if let Some(body) = body {
        Body::from(serde_json::to_string(&body).unwrap())
    } else {
        Body::empty()
    };

    let request = request_builder.body(body).unwrap();
    app.oneshot(request).await.unwrap()
}

pub fn create_test_api_medicine() -> ApiMedicine {
    ApiMedicine {
        name: "Test Medicine".to_string(),
        dose: 500.0,
        unit: "mg".to_string(),
        stock: 100.0,
    }
}

pub fn create_test_api_schedule() -> ApiMedicineSchedule {
    ApiMedicineSchedule {
        time: "08:00".to_string(),
        medicine_id: "test-medicine-id".to_string(),
        amount: 500.0,
    }
}

pub fn create_test_api_dosage_history() -> ApiDosageHistory {
    ApiDosageHistory {
        date: "2024-01-15".to_string(),
        time: "08:30".to_string(),
        medicine_id: "test-medicine-id".to_string(),
        amount: 500.0,
    }
} 