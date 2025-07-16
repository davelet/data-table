use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::ErrorKind;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub jdbc_url: Option<String>,
    pub schema: Option<String>,
    pub create_time: Option<i64>,
    pub touch_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowState {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: f64,
    pub height: f64,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            x: None,
            y: None,
            width: 1200.0,
            height: 800.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: String,
    pub font_size: u8,
    pub show_line_numbers: bool,
    pub max_rows_display: usize,
    pub saved_connections: Vec<ConnectionConfig>,
    pub window_state: Option<WindowState>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            font_size: 14,
            show_line_numbers: true,
            max_rows_display: 100,
            saved_connections: Vec::new(),
            window_state: None,
        }
    }
}

pub fn get_config_file_path() -> Result<PathBuf, String> {
    // Get the home directory using standard Rust APIs
    let home_dir = dirs::home_dir()
        .ok_or_else(|| "Could not find home directory".to_string())?;
    
    // Create a .config/you-my-sql directory in the home directory
    let app_dir = home_dir.join(".config").join("you-my-sql");
    
    // Create the directory if it doesn't exist
    if !app_dir.exists() {
        println!("Config directory does not exist, creating it");
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    let config_path = app_dir.join("config.toml");
    
    Ok(config_path)
}

pub fn read_config() -> Result<AppConfig, String> {
    println!("[DEBUG] read_config() called");
    let config_path = get_config_file_path()?;
    println!("[DEBUG] Config path: {}", config_path.display());
    
    // If the config file doesn't exist, return the default config
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            println!("[DEBUG] Config file read successfully");
            let result = toml::from_str(&content)
                .map_err(|e| format!("Failed to parse config file: {}", e));
            
            if result.is_ok() {
                println!("[DEBUG] Config parsed successfully");
            } else {
                println!("[DEBUG] Failed to parse config");
            }
            
            result
        },
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("[DEBUG] Config file not found, creating default config");
            // Create default config file if it doesn't exist
            let default_config = AppConfig::default();
            write_config(&default_config)?;
            Ok(default_config)
        },
        Err(e) => {
            println!("[DEBUG] Error reading config file: {}", e);
            Err(format!("Failed to read config file: {}", e))
        },
    }
}

pub fn write_config(config: &AppConfig) -> Result<(), String> {
    println!("[DEBUG] write_config() called");
    let config_path = get_config_file_path()?;
    println!("[DEBUG] Writing config to: {}", config_path.display());
    
    let toml_string = toml::to_string(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    
    println!("[DEBUG] Config serialized successfully");
    
    match fs::write(&config_path, toml_string) {
        Ok(_) => {
            println!("[DEBUG] Config written successfully");
            Ok(())
        },
        Err(e) => {
            println!("[DEBUG] Error writing config: {}", e);
            Err(format!("Failed to write config file: {}", e))
        }
    }
}
