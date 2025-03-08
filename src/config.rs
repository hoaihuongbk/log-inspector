use dotenvy::{dotenv, from_path};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub openai_api_key: String,
    pub openai_host: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Priority 1: System environment variables
        let env_api_key = env::var("OPENAI_API_KEY");
        let env_host = env::var("OPENAI_HOST");

        if let (Ok(api_key), Ok(host)) = (env_api_key, env_host) {
            return Ok(Config {
                openai_api_key: api_key,
                openai_host: host,
            });
        }

        // Priority 2: User config file (~/.log-inspector.cnf)
        let config_path = Config::get_config_path()?;
        if config_path.exists() {
            from_path(&config_path)?;
            if let (Ok(api_key), Ok(host)) = (env::var("OPENAI_API_KEY"), env::var("OPENAI_HOST")) {
                return Ok(Config {
                    openai_api_key: api_key,
                    openai_host: host,
                });
            }
        }

        // Priority 3: Local .env file
        dotenv().ok();
        let dotenv_api_key = env::var("OPENAI_API_KEY");
        let dotenv_host =
            env::var("OPENAI_HOST").unwrap_or_else(|_| "https://api.openai.com".to_string());

        if let Ok(api_key) = dotenv_api_key {
            return Ok(Config {
                openai_api_key: api_key,
                openai_host: dotenv_host,
            });
        }

        Err("No configuration found. Please set environment variables, create ~/.log-inspector.cnf, or provide .env file".into())
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let home = dirs::home_dir().ok_or("Could not find home directory")?;
        Ok(home.join(".log-inspector.cnf"))
    }
}
