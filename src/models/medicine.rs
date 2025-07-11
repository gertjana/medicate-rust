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

    pub fn reduce_stock(&self, amount: f64) -> Option<Self> {
        let new_stock = self.stock - amount;
        if new_stock < 0.0 {
            None
        } else {
            Some(Self {
                stock: new_stock,
                ..self.clone()
            })
        }
    }

    pub fn to_api_medicine(&self) -> ApiMedicine {
        ApiMedicine {
            name: self.name.clone(),
            dose: self.dose,
            unit: self.unit.clone(),
            stock: self.stock,
        }
    }
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