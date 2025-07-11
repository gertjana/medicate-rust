use anyhow::Result;
use redis::{AsyncCommands, Client, aio::Connection};
use serde_json;
use crate::models::{DosageHistory, ApiDosageHistory};

pub struct DosageHistoryRepository {
    redis_client: Client,
    prefix: String,
}

impl DosageHistoryRepository {
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

    pub async fn create(&self, api_history: ApiDosageHistory) -> Result<String> {
        let history = api_history.to_dosage_history(
            uuid::Uuid::new_v4().to_string(),
            String::new()
        );
        let key = format!("{}{}", self.prefix, history.id);
        let value = serde_json::to_string(&history)?;
        
        let mut conn = self.get_connection().await?;
        conn.set(&key, value).await?;
        
        Ok(history.id)
    }

    pub async fn get_all(&self) -> Result<Vec<DosageHistory>> {
        let mut conn = self.get_connection().await?;
        let pattern = format!("{}*", self.prefix);
        let keys: Vec<String> = conn.keys(&pattern).await?;
        
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let values: Vec<Option<String>> = conn.mget(&keys).await?;
        let mut histories = Vec::new();
        
        for value in values {
            if let Some(json_str) = value {
                if let Ok(history) = serde_json::from_str::<DosageHistory>(&json_str) {
                    histories.push(history);
                }
            }
        }
        
        histories.sort();
        Ok(histories)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<DosageHistory>> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn.get(&key).await?;
        
        match value {
            Some(json_str) => {
                let history = serde_json::from_str::<DosageHistory>(&json_str)?;
                Ok(Some(history))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        conn.del(&key).await?;
        
        Ok(())
    }
} 