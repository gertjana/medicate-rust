use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::medicine::MedicineId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DosageHistory {
    pub id: String,
    pub datetime: DateTime<Utc>,
    pub medicine_id: MedicineId,
    pub description: String,
    pub amount: f64,
}

impl DosageHistory {
    pub fn with_id(id: String, datetime: DateTime<Utc>, medicine_id: MedicineId, amount: f64) -> Self {
        Self {
            id,
            datetime,
            medicine_id,
            description: String::new(),
            amount,
        }
    }

    pub fn with_id_and_description(id: String, datetime: DateTime<Utc>, medicine_id: MedicineId, description: String, amount: f64) -> Self {
        Self {
            id,
            datetime,
            medicine_id,
            description,
            amount,
        }
    }
}

impl std::cmp::PartialOrd for DosageHistory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.datetime.cmp(&other.datetime))
    }
}

impl std::cmp::Ord for DosageHistory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl std::cmp::Eq for DosageHistory {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDosageHistory {
    pub date: String,
    pub time: String,
    pub medicine_id: MedicineId,
    pub amount: f64,
}

impl ApiDosageHistory {
    pub fn to_dosage_history(&self, id: String, description: String) -> Result<DosageHistory, chrono::ParseError> {
        let date_time_str = format!("{}T{}:00Z", self.date, self.time);
        let datetime = DateTime::parse_from_rfc3339(&date_time_str)?.with_timezone(&Utc);
        
        Ok(DosageHistory {
            id,
            datetime,
            medicine_id: self.medicine_id.clone(),
            description,
            amount: self.amount,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_dosage_history_with_id() {
        let id = "history-id-123".to_string();
        let datetime = Utc::now();
        let history = DosageHistory::with_id(
            id.clone(),
            datetime,
            "medicine-id-456".to_string(),
            500.0
        );
        
        assert_eq!(history.id, id);
        assert_eq!(history.datetime, datetime);
        assert_eq!(history.medicine_id, "medicine-id-456");
        assert_eq!(history.amount, 500.0);
        assert_eq!(history.description, "");
    }

    #[test]
    fn test_dosage_history_ordering() {
        let datetime1 = Utc.with_ymd_and_hms(2024, 1, 15, 8, 0, 0).unwrap();
        let datetime2 = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        let datetime3 = Utc.with_ymd_and_hms(2024, 1, 10, 8, 0, 0).unwrap();

        let history1 = DosageHistory::with_id(
            "id1".to_string(),
            datetime1,
            "med1".to_string(),
            100.0
        );
        let history2 = DosageHistory::with_id(
            "id2".to_string(),
            datetime2,
            "med2".to_string(),
            200.0
        );
        let history3 = DosageHistory::with_id(
            "id3".to_string(),
            datetime3,
            "med3".to_string(),
            300.0
        );
        
        let mut histories = vec![history1.clone(), history2.clone(), history3.clone()];
        histories.sort_by(|a, b| b.cmp(a));
        
        // Should be sorted by datetime (descending)
        assert_eq!(histories[0].datetime, datetime2);
        assert_eq!(histories[1].datetime, datetime1);
        assert_eq!(histories[2].datetime, datetime3);
    }

    #[test]
    fn test_api_dosage_history_to_dosage_history() {
        let api_history = ApiDosageHistory {
            date: "2024-01-20".to_string(),
            time: "14:30".to_string(),
            medicine_id: "test-medicine-id".to_string(),
            amount: 400.0,
        };
        
        let id = "custom-history-id".to_string();
        let description = "Test description".to_string();
        let datetime = Utc.with_ymd_and_hms(2024, 1, 20, 14, 30, 0).unwrap();
        let history = api_history.to_dosage_history(id.clone(), description.clone()).unwrap();
        
        assert_eq!(history.id, id);
        assert_eq!(history.datetime, datetime);
        assert_eq!(history.medicine_id, "test-medicine-id");
        assert_eq!(history.amount, 400.0);
        assert_eq!(history.description, description);
    }

    #[test]
    fn test_dosage_history_serialization() {
        let datetime = Utc.with_ymd_and_hms(2024, 1, 15, 9, 0, 0).unwrap();
        let history = DosageHistory::with_id(
            "test-id".to_string(),
            datetime,
            "medicine-id".to_string(),
            250.0
        );
        
        let json = serde_json::to_string(&history).unwrap();
        let deserialized: DosageHistory = serde_json::from_str(&json).unwrap();
        
        assert_eq!(history.id, deserialized.id);
        assert_eq!(history.datetime, deserialized.datetime);
        assert_eq!(history.medicine_id, deserialized.medicine_id);
        assert_eq!(history.amount, deserialized.amount);
        assert_eq!(history.description, deserialized.description);
    }

    #[test]
    fn test_api_dosage_history_serialization() {
        let api_history = ApiDosageHistory {
            date: "2024-01-20".to_string(),
            time: "16:45".to_string(),
            medicine_id: "test-medicine-id".to_string(),
            amount: 300.0,
        };
        
        let json = serde_json::to_string(&api_history).unwrap();
        let deserialized: ApiDosageHistory = serde_json::from_str(&json).unwrap();
        
        assert_eq!(api_history.date, deserialized.date);
        assert_eq!(api_history.time, deserialized.time);
        assert_eq!(api_history.medicine_id, deserialized.medicine_id);
        assert_eq!(api_history.amount, deserialized.amount);
    }

    #[test]
    fn test_dosage_history_equality() {
        let datetime1 = Utc.with_ymd_and_hms(2024, 1, 15, 8, 0, 0).unwrap();
        let datetime2 = Utc.with_ymd_and_hms(2024, 1, 15, 8, 0, 0).unwrap();
        let datetime3 = Utc.with_ymd_and_hms(2024, 1, 16, 8, 0, 0).unwrap(); // different date

        let history1 = DosageHistory::with_id(
            "id1".to_string(),
            datetime1,
            "med1".to_string(),
            100.0
        );
        let history2 = DosageHistory::with_id(
            "id1".to_string(),
            datetime2,
            "med1".to_string(),
            100.0
        );
        let history3 = DosageHistory::with_id(
            "id2".to_string(), // different id
            datetime3,
            "med1".to_string(),
            100.0
        );
        
        assert_eq!(history1, history2);
        assert_ne!(history1, history3);
    }

    #[test]
    fn test_dosage_history_partial_ord() {
        let datetime1 = Utc.with_ymd_and_hms(2024, 1, 15, 8, 0, 0).unwrap();
        let datetime2 = Utc.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
        let h1 = DosageHistory { id: "".to_string(), datetime: datetime1, medicine_id: "".to_string(), description: "".to_string(), amount: 0.0 };
        let h2 = DosageHistory { id: "".to_string(), datetime: datetime2, medicine_id: "".to_string(), description: "".to_string(), amount: 0.0 };
        assert!(h2 > h1);
        assert!(h1 < h2);
    }
} 