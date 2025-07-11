use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::medicine::{MedicineId, Medicine};

// pub type ScheduleId = String;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medicine_schedule_new() {
        let schedule = MedicineSchedule::new(
            "08:00".to_string(),
            "medicine-id-123".to_string(),
            500.0
        );
        
        assert_eq!(schedule.time, "08:00");
        assert_eq!(schedule.medicine_id, "medicine-id-123");
        assert_eq!(schedule.amount, 500.0);
        assert_eq!(schedule.description, "");
        assert!(!schedule.id.is_empty());
    }

    #[test]
    fn test_medicine_schedule_with_id() {
        let id = "schedule-id-456".to_string();
        let schedule = MedicineSchedule::with_id(
            id.clone(),
            "12:00".to_string(),
            "medicine-id-789".to_string(),
            250.0
        );
        
        assert_eq!(schedule.id, id);
        assert_eq!(schedule.time, "12:00");
        assert_eq!(schedule.medicine_id, "medicine-id-789");
        assert_eq!(schedule.amount, 250.0);
        assert_eq!(schedule.description, "");
    }

    #[test]
    fn test_medicine_schedule_ordering() {
        let schedule1 = MedicineSchedule::new("08:00".to_string(), "med1".to_string(), 100.0);
        let schedule2 = MedicineSchedule::new("12:00".to_string(), "med2".to_string(), 200.0);
        let schedule3 = MedicineSchedule::new("06:00".to_string(), "med3".to_string(), 300.0);
        
        let mut schedules = vec![schedule1.clone(), schedule2.clone(), schedule3.clone()];
        schedules.sort();
        
        // Should be sorted by time (numerically)
        assert_eq!(schedules[0].time, "06:00");
        assert_eq!(schedules[1].time, "08:00");
        assert_eq!(schedules[2].time, "12:00");
    }

    #[test]
    fn test_api_medicine_schedule_to_schedule() {
        let api_schedule = ApiMedicineSchedule {
            time: "14:30".to_string(),
            medicine_id: "test-medicine-id".to_string(),
            amount: 400.0,
        };
        
        let schedule = api_schedule.to_schedule();
        
        assert_eq!(schedule.time, "14:30");
        assert_eq!(schedule.medicine_id, "test-medicine-id");
        assert_eq!(schedule.amount, 400.0);
        assert_eq!(schedule.description, "");
        assert!(!schedule.id.is_empty());
    }

    #[test]
    fn test_api_medicine_schedule_to_schedule_with_id() {
        let api_schedule = ApiMedicineSchedule {
            time: "16:00".to_string(),
            medicine_id: "test-medicine-id".to_string(),
            amount: 300.0,
        };
        
        let id = "custom-schedule-id".to_string();
        let schedule = api_schedule.to_schedule_with_id(id.clone());
        
        assert_eq!(schedule.id, id);
        assert_eq!(schedule.time, "16:00");
        assert_eq!(schedule.medicine_id, "test-medicine-id");
        assert_eq!(schedule.amount, 300.0);
        assert_eq!(schedule.description, "");
    }

    #[test]
    fn test_daily_schedule_new() {
        let medicines = vec![
            (Some(Medicine::new("Med1".to_string(), 100.0, "mg".to_string(), 50.0)), 1.0),
            (Some(Medicine::new("Med2".to_string(), 200.0, "mg".to_string(), 25.0)), 2.0),
        ];
        
        let daily_schedule = DailySchedule::new("09:00".to_string(), medicines.clone());
        
        assert_eq!(daily_schedule.time, "09:00");
        assert_eq!(daily_schedule.medicines.len(), 2);
        assert_eq!(daily_schedule.taken, None);
    }

    #[test]
    fn test_daily_schedule_ordering() {
        let schedule1 = DailySchedule::new("08:00".to_string(), vec![]);
        let schedule2 = DailySchedule::new("12:00".to_string(), vec![]);
        let schedule3 = DailySchedule::new("06:00".to_string(), vec![]);
        
        let mut schedules = vec![schedule1.clone(), schedule2.clone(), schedule3.clone()];
        schedules.sort();
        
        assert_eq!(schedules[0].time, "06:00");
        assert_eq!(schedules[1].time, "08:00");
        assert_eq!(schedules[2].time, "12:00");
    }

    #[test]
    fn test_daily_schedule_with_date_new() {
        let daily_schedules = vec![
            DailySchedule::new("08:00".to_string(), vec![]),
            DailySchedule::new("12:00".to_string(), vec![]),
        ];
        
        let schedule_with_date = DailyScheduleWithDate::new("2024-01-15".to_string(), daily_schedules.clone());
        
        assert_eq!(schedule_with_date.date, "2024-01-15");
        assert_eq!(schedule_with_date.schedules.len(), 2);
    }

    #[test]
    fn test_daily_schedule_with_date_ordering() {
        let schedule1 = DailyScheduleWithDate::new("2024-01-15".to_string(), vec![]);
        let schedule2 = DailyScheduleWithDate::new("2024-01-20".to_string(), vec![]);
        let schedule3 = DailyScheduleWithDate::new("2024-01-10".to_string(), vec![]);
        
        let mut schedules = vec![schedule1.clone(), schedule2.clone(), schedule3.clone()];
        schedules.sort();
        
        assert_eq!(schedules[0].date, "2024-01-10");
        assert_eq!(schedules[1].date, "2024-01-15");
        assert_eq!(schedules[2].date, "2024-01-20");
    }

    #[test]
    fn test_medicine_schedule_serialization() {
        let schedule = MedicineSchedule::new(
            "10:30".to_string(),
            "test-medicine-id".to_string(),
            500.0
        );
        
        let json = serde_json::to_string(&schedule).unwrap();
        let deserialized: MedicineSchedule = serde_json::from_str(&json).unwrap();
        
        assert_eq!(schedule.id, deserialized.id);
        assert_eq!(schedule.time, deserialized.time);
        assert_eq!(schedule.medicine_id, deserialized.medicine_id);
        assert_eq!(schedule.amount, deserialized.amount);
        assert_eq!(schedule.description, deserialized.description);
    }

    #[test]
    fn test_api_medicine_schedule_serialization() {
        let api_schedule = ApiMedicineSchedule {
            time: "15:45".to_string(),
            medicine_id: "test-medicine-id".to_string(),
            amount: 300.0,
        };
        
        let json = serde_json::to_string(&api_schedule).unwrap();
        let deserialized: ApiMedicineSchedule = serde_json::from_str(&json).unwrap();
        
        assert_eq!(api_schedule.time, deserialized.time);
        assert_eq!(api_schedule.medicine_id, deserialized.medicine_id);
        assert_eq!(api_schedule.amount, deserialized.amount);
    }

    #[test]
    fn test_daily_schedule_serialization() {
        let daily_schedule = DailySchedule::new("09:00".to_string(), vec![]);
        
        let json = serde_json::to_string(&daily_schedule).unwrap();
        let deserialized: DailySchedule = serde_json::from_str(&json).unwrap();
        
        assert_eq!(daily_schedule.time, deserialized.time);
        assert_eq!(daily_schedule.medicines.len(), deserialized.medicines.len());
        assert_eq!(daily_schedule.taken, deserialized.taken);
    }

    #[test]
    fn test_daily_schedule_with_date_serialization() {
        let schedule_with_date = DailyScheduleWithDate::new("2024-01-15".to_string(), vec![]);
        
        let json = serde_json::to_string(&schedule_with_date).unwrap();
        let deserialized: DailyScheduleWithDate = serde_json::from_str(&json).unwrap();
        
        assert_eq!(schedule_with_date.date, deserialized.date);
        assert_eq!(schedule_with_date.schedules.len(), deserialized.schedules.len());
    }
} 