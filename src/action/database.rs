//! Database-related commands

use super::CommandResponse;
use you_my_sql_db::{CONNECTION_MANAGER, ConnectionConfig, QueryResult};

use chrono::Utc;

pub async fn connect_to_database(mut config: ConnectionConfig) -> Result<ConnectionConfig, String> {
    let mut manager = CONNECTION_MANAGER
        .lock()
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    config.touch_time = Some(Utc::now().timestamp_millis());
    if config.create_time.is_none() {
        config.create_time = Some(Utc::now().timestamp_millis());
    }
    manager
        .create_connection(config.clone())
        .map_err(|e| e.to_string())?; // Clone config for create_connection
    Ok(config) // Return the modified config
}

pub async fn execute_query(conn_id: String, query: String) -> CommandResponse<QueryResult> {
    let manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.execute_query(&conn_id, &query) {
        Ok(result) => CommandResponse::success(result),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn list_databases(conn_id: String) -> CommandResponse<Vec<String>> {
    let manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.list_databases(&conn_id) {
        Ok(databases) => CommandResponse::success(databases),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn list_tables(conn_id: String, schema: String) -> CommandResponse<Vec<String>> {
    let manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.list_tables(&conn_id, &schema) {
        Ok(tables) => CommandResponse::success(tables),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn get_table_data(
    conn_id: String,
    schema: String,
    table: String,
    limit: usize,
) -> CommandResponse<QueryResult> {
    let manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.get_table_data(&conn_id, &schema, &table, limit) {
        Ok(result) => CommandResponse::success(result),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn close_connection(conn_id: String) -> CommandResponse<()> {
    let mut manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.close_connection(&conn_id) {
        Ok(_) => CommandResponse::success(()),
        Err(e) => CommandResponse::error(e),
    }
}

pub async fn close_all_connections() -> CommandResponse<()> {
    let mut manager = match CONNECTION_MANAGER.lock() {
        Ok(manager) => manager,
        Err(e) => return CommandResponse::error(format!("Failed to acquire lock: {}", e)),
    };

    match manager.close_all_connections() {
        Ok(_) => CommandResponse::success(()),
        Err(e) => CommandResponse::error(e),
    }
}
