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
        conn.set(&key, value).await?;
        
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
        conn.set(&key, value).await?;
        
        Ok(true)
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        conn.del(&key).await?;
        
        Ok(())
    }

    pub async fn add_stock(&self, id: &str, amount: f64) -> Result<bool> {
        if let Some(medicine) = self.get_by_id(id).await? {
            let updated_medicine = medicine.add_stock(amount);
            let key = format!("{}{}", self.prefix, id);
            let value = serde_json::to_string(&updated_medicine)?;
            
            let mut conn = self.get_connection().await?;
            conn.set(&key, value).await?;
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn reduce_stock(&self, id: &str, amount: f64) -> Result<bool> {
        if let Some(medicine) = self.get_by_id(id).await? {
            if let Some(updated_medicine) = medicine.reduce_stock(amount) {
                let key = format!("{}{}", self.prefix, id);
                let value = serde_json::to_string(&updated_medicine)?;
                
                let mut conn = self.get_connection().await?;
                conn.set(&key, value).await?;
                
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
} 