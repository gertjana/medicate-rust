use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::medicine::MedicineId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DosageHistory {
    pub id: String,
    pub date: String,
    pub time: String,
    pub medicine_id: MedicineId,
    pub description: String,
    pub amount: f64,
}

impl DosageHistory {
    pub fn new(date: String, time: String, medicine_id: MedicineId, amount: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            date,
            time,
            medicine_id,
            description: String::new(),
            amount,
        }
    }

    pub fn with_id(id: String, date: String, time: String, medicine_id: MedicineId, amount: f64) -> Self {
        Self {
            id,
            date,
            time,
            medicine_id,
            description: String::new(),
            amount,
        }
    }
}

impl std::cmp::PartialOrd for DosageHistory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let date_cmp = self.date.cmp(&other.date);
        if date_cmp != std::cmp::Ordering::Equal {
            Some(date_cmp.reverse())
        } else {
            let time_cmp = self.time.replace(":", "").parse::<i32>()
                .unwrap_or(0)
                .cmp(&other.time.replace(":", "").parse::<i32>().unwrap_or(0));
            Some(time_cmp.reverse())
        }
    }
}

impl std::cmp::Ord for DosageHistory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let date_cmp = self.date.cmp(&other.date);
        if date_cmp != std::cmp::Ordering::Equal {
            date_cmp.reverse()
        } else {
            self.time.replace(":", "").parse::<i32>()
                .unwrap_or(0)
                .cmp(&other.time.replace(":", "").parse::<i32>().unwrap_or(0))
                .reverse()
        }
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
    pub fn to_dosage_history(&self, id: String, _description: String) -> DosageHistory {
        DosageHistory::with_id(
            id,
            self.date.clone(),
            self.time.clone(),
            self.medicine_id.clone(),
            self.amount,
        )
    }
} 