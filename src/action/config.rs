//! Configuration-related commands

use super::CommandResponse;
use you_my_sql_config::{AppConfig, read_config, write_config};

pub async fn get_app_config() -> CommandResponse<AppConfig> {
    println!("[DEBUG] get_app_config() command called");
    match read_config() {
        Ok(config) => {
            println!("[DEBUG] get_app_config() successful, returning config");
            CommandResponse::success(config)
        }
        Err(e) => {
            println!("[DEBUG] get_app_config() failed: {}", e);
            CommandResponse::error(e)
        }
    }
}

pub async fn save_app_config(config: AppConfig) -> CommandResponse<()> {
    println!("[DEBUG] save_app_config() command called");
    println!(
        "[DEBUG] Connections count: {}",
        config.saved_connections.len()
    );
    match write_config(&config) {
        Ok(_) => {
            println!("[DEBUG] save_app_config() successful");
            CommandResponse::success(())
        }
        Err(e) => {
            println!("[DEBUG] save_app_config() failed: {}", e);
            CommandResponse::error(e)
        }
    }
}
