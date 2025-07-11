use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type MedicineId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Medicine {
    pub id: MedicineId,
    pub name: String,
    pub dose: f64,
    pub unit: String,
    pub stock: f64,
}

impl Medicine {
    pub fn new(name: String, dose: f64, unit: String, stock: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            dose,
            unit,
            stock,
        }
    }

    pub fn with_id(id: MedicineId, name: String, dose: f64, unit: String, stock: f64) -> Self {
        Self {
            id,
            name,
            dose,
            unit,
            stock,
        }
    }

    pub fn add_stock(&self, amount: f64) -> Self {
        Self {
            stock: self.stock + amount,
            ..self.clone()
        }
    }

    // pub fn reduce_stock(&self, amount: f64) -> Option<Self> {
    //     let new_stock = self.stock - amount;
    //     if new_stock < 0.0 {
    //         None
    //     } else {
    //         Some(Self {
    //             stock: new_stock,
    //             ..self.clone()
    //         })
    //     }
    // }

    // pub fn to_api_medicine(&self) -> ApiMedicine {
    //     ApiMedicine {
    //         name: self.name.clone(),
    //         dose: self.dose,
    //         unit: self.unit.clone(),
    //         stock: self.stock,
    //     }
    // }
}

impl std::fmt::Display for Medicine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} {})", self.name, self.dose, self.unit)
    }
}

impl std::cmp::PartialOrd for Medicine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.name.cmp(&other.name))
    }
}

impl std::cmp::Ord for Medicine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl std::cmp::Eq for Medicine {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMedicine {
    pub name: String,
    pub dose: f64,
    pub unit: String,
    pub stock: f64,
}

impl ApiMedicine {
    pub fn to_medicine(&self) -> Medicine {
        Medicine::new(
            self.name.clone(),
            self.dose,
            self.unit.clone(),
            self.stock,
        )
    }

    pub fn to_medicine_with_id(&self, id: MedicineId) -> Medicine {
        Medicine::with_id(id, self.name.clone(), self.dose, self.unit.clone(), self.stock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medicine_new() {
        let medicine = Medicine::new(
            "Aspirin".to_string(),
            500.0,
            "mg".to_string(),
            100.0
        );
        
        assert_eq!(medicine.name, "Aspirin");
        assert_eq!(medicine.dose, 500.0);
        assert_eq!(medicine.unit, "mg");
        assert_eq!(medicine.stock, 100.0);
        assert!(!medicine.id.is_empty());
    }

    #[test]
    fn test_medicine_with_id() {
        let id = "test-id-123".to_string();
        let medicine = Medicine::with_id(
            id.clone(),
            "Ibuprofen".to_string(),
            200.0,
            "mg".to_string(),
            50.0
        );
        
        assert_eq!(medicine.id, id);
        assert_eq!(medicine.name, "Ibuprofen");
        assert_eq!(medicine.dose, 200.0);
        assert_eq!(medicine.unit, "mg");
        assert_eq!(medicine.stock, 50.0);
    }

    #[test]
    fn test_medicine_add_stock() {
        let medicine = Medicine::new(
            "Paracetamol".to_string(),
            500.0,
            "mg".to_string(),
            100.0
        );
        
        let updated = medicine.add_stock(50.0);
        assert_eq!(updated.stock, 150.0);
        assert_eq!(updated.name, medicine.name);
        assert_eq!(updated.dose, medicine.dose);
        assert_eq!(updated.unit, medicine.unit);
    }

    #[test]
    fn test_medicine_display() {
        let medicine = Medicine::new(
            "Vitamin C".to_string(),
            1000.0,
            "mg".to_string(),
            200.0
        );
        
        let display = format!("{}", medicine);
        assert_eq!(display, "Vitamin C (1000 mg)");
    }

    #[test]
    fn test_medicine_ordering() {
        let medicine1 = Medicine::new("Aspirin".to_string(), 500.0, "mg".to_string(), 100.0);
        let medicine2 = Medicine::new("Ibuprofen".to_string(), 200.0, "mg".to_string(), 50.0);
        let medicine3 = Medicine::new("Paracetamol".to_string(), 500.0, "mg".to_string(), 75.0);
        
        let mut medicines = vec![medicine1.clone(), medicine2.clone(), medicine3.clone()];
        medicines.sort();
        
        assert_eq!(medicines[0].name, "Aspirin");
        assert_eq!(medicines[1].name, "Ibuprofen");
        assert_eq!(medicines[2].name, "Paracetamol");
    }

    #[test]
    fn test_api_medicine_to_medicine() {
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 250.0,
            unit: "mg".to_string(),
            stock: 25.0,
        };
        
        let medicine = api_medicine.to_medicine();
        
        assert_eq!(medicine.name, "Test Medicine");
        assert_eq!(medicine.dose, 250.0);
        assert_eq!(medicine.unit, "mg");
        assert_eq!(medicine.stock, 25.0);
        assert!(!medicine.id.is_empty());
    }

    #[test]
    fn test_api_medicine_to_medicine_with_id() {
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 250.0,
            unit: "mg".to_string(),
            stock: 25.0,
        };
        
        let id = "custom-id-123".to_string();
        let medicine = api_medicine.to_medicine_with_id(id.clone());
        
        assert_eq!(medicine.id, id);
        assert_eq!(medicine.name, "Test Medicine");
        assert_eq!(medicine.dose, 250.0);
        assert_eq!(medicine.unit, "mg");
        assert_eq!(medicine.stock, 25.0);
    }

    #[test]
    fn test_medicine_serialization() {
        let medicine = Medicine::new(
            "Test Medicine".to_string(),
            500.0,
            "mg".to_string(),
            100.0
        );
        
        let json = serde_json::to_string(&medicine).unwrap();
        let deserialized: Medicine = serde_json::from_str(&json).unwrap();
        
        assert_eq!(medicine.name, deserialized.name);
        assert_eq!(medicine.dose, deserialized.dose);
        assert_eq!(medicine.unit, deserialized.unit);
        assert_eq!(medicine.stock, deserialized.stock);
        assert_eq!(medicine.id, deserialized.id);
    }

    #[test]
    fn test_api_medicine_serialization() {
        let api_medicine = ApiMedicine {
            name: "Test API Medicine".to_string(),
            dose: 300.0,
            unit: "mg".to_string(),
            stock: 75.0,
        };
        
        let json = serde_json::to_string(&api_medicine).unwrap();
        let deserialized: ApiMedicine = serde_json::from_str(&json).unwrap();
        
        assert_eq!(api_medicine.name, deserialized.name);
        assert_eq!(api_medicine.dose, deserialized.dose);
        assert_eq!(api_medicine.unit, deserialized.unit);
        assert_eq!(api_medicine.stock, deserialized.stock);
    }
} 