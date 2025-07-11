use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_port: u16,
    pub redis_host: String,
    pub redis_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let redis_host = env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
        let redis_port = env::var("REDIS_PORT")
            .unwrap_or_else(|_| "6379".to_string())
            .parse()
            .unwrap_or(6379);

        Self {
            server_port,
            redis_host,
            redis_port,
        }
    }

    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }
} 