use anyhow::Result;
use redis::{AsyncCommands, Client, aio::Connection};
use serde_json;
use crate::models::{
    MedicineSchedule, ApiMedicineSchedule, DailySchedule, DailyScheduleWithDate
};

pub struct MedicineScheduleRepository {
    redis_client: Client,
    prefix: String,
}

impl MedicineScheduleRepository {
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

    pub async fn create(&self, api_schedule: ApiMedicineSchedule) -> Result<String> {
        let schedule = api_schedule.to_schedule();
        let key = format!("{}{}", self.prefix, schedule.id);
        let value = serde_json::to_string(&schedule)?;
        
        let mut conn = self.get_connection().await?;
        let _: () = conn.set(&key, value).await?;
        
        Ok(schedule.id)
    }

    pub async fn get_all(&self) -> Result<Vec<MedicineSchedule>> {
        let mut conn = self.get_connection().await?;
        let pattern = format!("{}*", self.prefix);
        let keys: Vec<String> = conn.keys(&pattern).await?;
        
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let values: Vec<Option<String>> = conn.mget(&keys).await?;
        let mut schedules = Vec::new();
        
        for value in values {
            if let Some(json_str) = value {
                if let Ok(schedule) = serde_json::from_str::<MedicineSchedule>(&json_str) {
                    schedules.push(schedule);
                }
            }
        }
        
        schedules.sort();
        Ok(schedules)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<MedicineSchedule>> {
        let key = format!("{}{}", self.prefix, id);
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn.get(&key).await?;
        
        match value {
            Some(json_str) => {
                let schedule = serde_json::from_str::<MedicineSchedule>(&json_str)?;
                Ok(Some(schedule))
            }
            None => Ok(None),
        }
    }

    pub async fn update(&self, id: &str, api_schedule: ApiMedicineSchedule) -> Result<bool> {
        let schedule = api_schedule.to_schedule_with_id(id.to_string());
        let key = format!("{}{}", self.prefix, id);
        let value = serde_json::to_string(&schedule)?;
        
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

    pub async fn get_daily_schedule(&self, _date: &str, medicine_repo: &crate::repositories::MedicineRepository) -> Result<Vec<DailySchedule>> {
        let schedules = self.get_all().await?;
        let mut daily_schedules = Vec::new();
        
        // Group schedules by time
        let mut time_groups: std::collections::HashMap<String, Vec<MedicineSchedule>> = std::collections::HashMap::new();
        
        for schedule in schedules {
            time_groups.entry(schedule.time.clone()).or_insert_with(Vec::new).push(schedule);
        }
        
        for (time, schedules) in time_groups {
            let mut medicines_with_amounts = Vec::new();
            
            for schedule in schedules {
                let medicine = medicine_repo.get_by_id(&schedule.medicine_id).await?;
                medicines_with_amounts.push((medicine, schedule.amount));
            }
            
            let daily_schedule = DailySchedule::new(time, medicines_with_amounts);
            daily_schedules.push(daily_schedule);
        }
        
        daily_schedules.sort();
        Ok(daily_schedules)
    }

    pub async fn get_daily_schedule_with_date(&self, date: &str, medicine_repo: &crate::repositories::MedicineRepository) -> Result<DailyScheduleWithDate> {
        let schedules = self.get_daily_schedule(date, medicine_repo).await?;
        Ok(DailyScheduleWithDate::new(date.to_string(), schedules))
    }
} 