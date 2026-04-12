use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
}

pub type MySqlState = Arc<Mutex<Option<MySqlPool>>>;

#[tauri::command]
pub async fn test_mysql_connection(config: DbConfig) -> Result<bool, String> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );
    match MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&url)
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn connect_mysql(
    mysql: State<'_, MySqlState>,
    config: DbConfig,
) -> Result<(), String> {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .map_err(|e| e.to_string())?;
    let mut guard = mysql.lock().await;
    *guard = Some(pool);
    Ok(())
}

#[tauri::command]
pub async fn get_mysql_status(mysql: State<'_, MySqlState>) -> Result<bool, String> {
    let guard = mysql.lock().await;
    Ok(guard.is_some())
}

#[tauri::command]
pub async fn backup_sqlite(app: tauri::AppHandle) -> Result<String, String> {
    use tauri::Manager;
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("tb_clinic.db");
    Ok(db_path.to_string_lossy().to_string())
}