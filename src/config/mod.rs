use std::env;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub database_url: String,
}

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let database_url = match dotenvy::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => env::args()
            .nth(1)
            .expect("DATABASE_URL not found in environment variables or command line arguments"),
    };

    Ok(DotEnvyConfig { database_url })
}
