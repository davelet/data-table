//! Commands module for Tauri commands

mod config;
mod database;
mod window;

pub use config::*;
// pub use database::*;
// pub use window::*;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CommandResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(err: impl ToString) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(err.to_string()),
        }
    }

    pub fn into_result(self) -> Result<T, String> {
        if self.success {
            self.data
                .ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(self.error.unwrap_or_else(|| "Unknown error".to_string()))
        }
    }
}
