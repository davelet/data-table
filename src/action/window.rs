//! Window state-related commands

use super::CommandResponse;
use serde::Deserialize;
use you_my_sql_config::{WindowState, read_config, write_config};

#[derive(Debug, Deserialize)]
pub struct WindowStateParams {
    pub x: i32,
    pub y: i32,
    pub width: f64,
    pub height: f64,
}

// Internal function to save window state without using CommandResponse
pub fn save_window_state_internal(params: WindowStateParams) -> Result<(), String> {
    let mut config = read_config().map_err(|e| e.to_string())?;

    // Initialize window_state if it's None
    if config.window_state.is_none() {
        config.window_state = Some(WindowState::default());
    }

    // Safe to unwrap here since we just set it to Some
    let window_state = config.window_state.as_mut().unwrap();

    // Update window position and size
    window_state.x = Some(params.x);
    window_state.y = Some(params.y);
    window_state.width = params.width;
    window_state.height = params.height;

    write_config(&config).map_err(|e| e.to_string())
}

pub async fn save_window_state(params: WindowStateParams) -> CommandResponse<()> {
    let mut config = match read_config() {
        Ok(config) => config,
        Err(e) => return CommandResponse::error(e),
    };
    println!("state param = {:?}", params);
    println!("state = {:?}", config.window_state);
    // Initialize window_state if it's None
    if config.window_state.is_none() {
        config.window_state = Some(WindowState::default());
    }

    // Safe to unwrap here since we just set it to Some
    let window_state = config.window_state.as_mut().unwrap();

    // Update window position and size
    window_state.x = Some(params.x);
    window_state.y = Some(params.y);
    window_state.width = params.width;
    window_state.height = params.height;

    let r = write_config(&config);
    println!("write config = {:?}", r);
    match r {
        Ok(_) => CommandResponse::success(()),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn get_window_state() -> CommandResponse<WindowState> {
    match read_config() {
        Ok(config) => {
            // Return the window state or the default if it's None
            CommandResponse::success(config.window_state.unwrap_or_default())
        }
        Err(e) => CommandResponse::error(e),
    }
}
