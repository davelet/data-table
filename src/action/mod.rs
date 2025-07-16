//! Commands module for Tauri commands

mod config;
mod database;
mod window;

pub use config::*;
pub use database::*;
pub use window::*;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T) -> Self {
        CommandResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(err: impl ToString) -> Self {
        CommandResponse {
            success: false,
            data: None,
            error: Some(err.to_string()),
        }
    }
}
