use anyhow::Result;
use redis::{AsyncCommands, Client, aio::Connection};
use serde_json;
use crate::models::{Medicine, ApiMedicine, MedicineId};

pub struct MedicineRepository {
    redis_client: Client,
    prefix: String,
}

impl MedicineRepository {
    pub fn new(redis_url: &str, prefix: String) -> Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            redis_client: client,
            prefix,
        })
    }

    async fn get_connection(&self) -> Result<Connection> {
        self.redis_client.get_async_connection().await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn create(&self, api_medicine: ApiMedicine) -> Result<MedicineId> {
        let medicine = api_medicine.to_medicine();
        let key = format!("{}{}", self.prefix, medicine.id);
        let value = serde_json::to_string(&medicine)?;
        
        let mut conn = self.get_connection().await?;
        let _: () = conn.set(&key, value).await?;
        
        Ok(medicine.id)
    }

    pub async fn get_all(&self) -> Result<Vec<Medicine>> {
        let mut conn = self.get_connection().await?;
        let pattern = format!("{}*", self.prefix);
        let keys: Vec<String> = conn.keys(&pattern).await?;
        
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let values: Vec<Option<String>> = conn.mget(&keys).await?;
        let mut medicines = Vec::new();
        
        for value in values {
            if let Some(json_str) = value {
                if let Ok(medicine) = serde_json::from_str::<Medicine>(&json_str) {
                    medicines.push(medicine);
                }
            }
        }
        
        medicines.sort();
        Ok(medicines)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Medicine>> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn.get(&key).await?;
        
        match value {
            Some(json_str) => {
                let medicine = serde_json::from_str::<Medicine>(&json_str)?;
                Ok(Some(medicine))
            }
            None => Ok(None),
        }
    }

    pub async fn update(&self, id: &str, api_medicine: ApiMedicine) -> Result<bool> {
        let medicine = api_medicine.to_medicine_with_id(id.to_string());
        let key = format!("{}{}", self.prefix, id);
        let value = serde_json::to_string(&medicine)?;
        
        let mut conn = self.get_connection().await?;
        let _: () = conn.set(&key, value).await?;
        
        Ok(true)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        let _: () = conn.del(&key).await?;
        
        Ok(())
    }

    pub async fn add_stock(&self, id: &str, amount: f64) -> Result<bool> {
        if let Some(medicine) = self.get_by_id(id).await? {
            let updated_medicine = medicine.add_stock(amount);
            let key = format!("{}{}", self.prefix, id);
            let value = serde_json::to_string(&updated_medicine)?;
            
            let mut conn = self.get_connection().await?;
            let _: () = conn.set(&key, value).await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // pub async fn reduce_stock(&self, id: &str, amount: f64) -> Result<bool> {
    //     if let Some(medicine) = self.get_by_id(id).await? {
    //         if let Some(updated_medicine) = medicine.reduce_stock(amount) {
    //             let key = format!("{}{}", self.prefix, id);
    //             let value = serde_json::to_string(&updated_medicine)?;
    //             
    //             let mut conn = self.get_connection().await?;
    //             let _: () = conn.set(&key, value).await?;
    //             
    //             Ok(true)
    //         } else {
    //             Ok(false)
    //         }
    //     } else {
    //         Ok(false)
    //     }
    // }
} 

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_repository() -> MedicineRepository {
        // Use a test Redis instance or mock
        MedicineRepository::new("redis://localhost:6379", "test:medicine:".to_string()).unwrap()
    }

    async fn create_empty_test_repository() -> MedicineRepository {
        // Use a unique prefix to ensure empty database
        let unique_prefix = format!("test:empty:{}:", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
        MedicineRepository::new("redis://localhost:6379", unique_prefix).unwrap()
    }

    #[tokio::test]
    async fn test_create_medicine() {
        let repo = create_test_repository().await;
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 500.0,
            unit: "mg".to_string(),
            stock: 100.0,
        };

        let result = repo.create(api_medicine).await;
        assert!(result.is_ok());
        
        let id = result.unwrap();
        assert!(!id.is_empty());
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let repo = create_test_repository().await;
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 500.0,
            unit: "mg".to_string(),
            stock: 100.0,
        };

        let id = repo.create(api_medicine).await.unwrap();
        let medicine = repo.get_by_id(&id).await.unwrap();
        
        assert!(medicine.is_some());
        let medicine = medicine.unwrap();
        assert_eq!(medicine.name, "Test Medicine");
        assert_eq!(medicine.dose, 500.0);
        assert_eq!(medicine.unit, "mg");
        assert_eq!(medicine.stock, 100.0);
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let repo = create_test_repository().await;
        let result = repo.get_by_id("non-existent-id").await;
        
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_medicine() {
        let repo = create_test_repository().await;
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 500.0,
            unit: "mg".to_string(),
            stock: 100.0,
        };

        let id = repo.create(api_medicine).await.unwrap();
        
        let updated_api_medicine = ApiMedicine {
            name: "Updated Medicine".to_string(),
            dose: 750.0,
            unit: "mg".to_string(),
            stock: 150.0,
        };

        let result = repo.update(&id, updated_api_medicine).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        let medicine = repo.get_by_id(&id).await.unwrap().unwrap();
        assert_eq!(medicine.name, "Updated Medicine");
        assert_eq!(medicine.dose, 750.0);
        assert_eq!(medicine.stock, 150.0);
    }

    #[tokio::test]
    async fn test_delete_medicine() {
        let repo = create_test_repository().await;
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 500.0,
            unit: "mg".to_string(),
            stock: 100.0,
        };

        let id = repo.create(api_medicine).await.unwrap();
        
        // Verify it exists
        let medicine = repo.get_by_id(&id).await.unwrap();
        assert!(medicine.is_some());

        // Delete it
        let result = repo.delete(&id).await;
        assert!(result.is_ok());

        // Verify it's gone
        let medicine = repo.get_by_id(&id).await.unwrap();
        assert!(medicine.is_none());
    }

    #[tokio::test]
    async fn test_add_stock() {
        let repo = create_test_repository().await;
        let api_medicine = ApiMedicine {
            name: "Test Medicine".to_string(),
            dose: 500.0,
            unit: "mg".to_string(),
            stock: 100.0,
        };

        let id = repo.create(api_medicine).await.unwrap();
        
        let result = repo.add_stock(&id, 50.0).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        let medicine = repo.get_by_id(&id).await.unwrap().unwrap();
        assert_eq!(medicine.stock, 150.0);
    }

    #[tokio::test]
    async fn test_add_stock_medicine_not_found() {
        let repo = create_test_repository().await;
        let result = repo.add_stock("non-existent-id", 50.0).await;
        
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_get_all_medicines() {
        let repo = create_test_repository().await;
        
        // Create multiple medicines
        let medicines = vec![
            ApiMedicine {
                name: "Medicine A".to_string(),
                dose: 100.0,
                unit: "mg".to_string(),
                stock: 50.0,
            },
            ApiMedicine {
                name: "Medicine B".to_string(),
                dose: 200.0,
                unit: "mg".to_string(),
                stock: 75.0,
            },
        ];

        for medicine in medicines {
            repo.create(medicine).await.unwrap();
        }

        let all_medicines = repo.get_all().await.unwrap();
        assert!(all_medicines.len() >= 2);
        
        // Check that they're sorted by name
        let names: Vec<&str> = all_medicines.iter().map(|m| m.name.as_str()).collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        assert_eq!(names, sorted_names);
    }

    #[tokio::test]
    async fn test_get_all_empty() {
        let repo = create_empty_test_repository().await;
        let medicines = repo.get_all().await.unwrap();
        assert_eq!(medicines.len(), 0);
    }

    #[test]
    fn test_medicine_repository_new() {
        let result = MedicineRepository::new("redis://localhost:6379", "test:".to_string());
        assert!(result.is_ok());
        
        let repo = result.unwrap();
        assert_eq!(repo.prefix, "test:");
    }

    #[test]
    fn test_medicine_repository_new_invalid_redis_url() {
        let result = MedicineRepository::new("invalid-url", "test:".to_string());
        assert!(result.is_err());
    }
} 