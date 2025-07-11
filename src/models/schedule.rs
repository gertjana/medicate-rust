use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::medicine::{MedicineId, Medicine};

pub type ScheduleId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MedicineSchedule {
    pub id: String,
    pub time: String,
    pub medicine_id: MedicineId,
    pub description: String,
    pub amount: f64,
}

impl MedicineSchedule {
    pub fn new(time: String, medicine_id: MedicineId, amount: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            time,
            medicine_id,
            description: String::new(),
            amount,
        }
    }

    pub fn with_id(id: String, time: String, medicine_id: MedicineId, amount: f64) -> Self {
        Self {
            id,
            time,
            medicine_id,
            description: String::new(),
            amount,
        }
    }
}

impl std::cmp::PartialOrd for MedicineSchedule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let time_cmp = self.time.replace(":", "").parse::<i32>()
            .unwrap_or(0)
            .cmp(&other.time.replace(":", "").parse::<i32>().unwrap_or(0));
        Some(time_cmp)
    }
}

impl std::cmp::Ord for MedicineSchedule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.replace(":", "").parse::<i32>()
            .unwrap_or(0)
            .cmp(&other.time.replace(":", "").parse::<i32>().unwrap_or(0))
    }
}

impl std::cmp::Eq for MedicineSchedule {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMedicineSchedule {
    pub time: String,
    pub medicine_id: MedicineId,
    pub amount: f64,
}

impl ApiMedicineSchedule {
    pub fn to_schedule(&self) -> MedicineSchedule {
        MedicineSchedule::new(
            self.time.clone(),
            self.medicine_id.clone(),
            self.amount,
        )
    }

    pub fn to_schedule_with_id(&self, id: String) -> MedicineSchedule {
        MedicineSchedule::with_id(id, self.time.clone(), self.medicine_id.clone(), self.amount)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailySchedule {
    pub time: String,
    pub medicines: Vec<(Option<Medicine>, f64)>,
    pub taken: Option<bool>,
}

impl DailySchedule {
    pub fn new(time: String, medicines: Vec<(Option<Medicine>, f64)>) -> Self {
        Self {
            time,
            medicines,
            taken: None,
        }
    }
}

impl std::cmp::PartialOrd for DailySchedule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.time.cmp(&other.time))
    }
}

impl std::cmp::Ord for DailySchedule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

impl std::cmp::Eq for DailySchedule {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyScheduleWithDate {
    pub date: String,
    pub schedules: Vec<DailySchedule>,
}

impl DailyScheduleWithDate {
    pub fn new(date: String, schedules: Vec<DailySchedule>) -> Self {
        Self { date, schedules }
    }
}

impl std::cmp::PartialOrd for DailyScheduleWithDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.date.cmp(&other.date))
    }
}

impl std::cmp::Ord for DailyScheduleWithDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl std::cmp::Eq for DailyScheduleWithDate {} 