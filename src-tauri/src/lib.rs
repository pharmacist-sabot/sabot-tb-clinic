mod models;
mod db;
mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;
use sqlx::sqlite::SqlitePoolOptions;
use tauri::Manager;
use commands::settings::MySqlState;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                // Resolve the OS-appropriate app data directory and ensure it exists
                let app_data_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("Failed to get app data dir");
                std::fs::create_dir_all(&app_data_dir)
                    .expect("Failed to create app data dir");

                let db_path = app_data_dir.join("tb_clinic.db");
                let db_url = format!(
                    "sqlite://{}?mode=rwc",
                    db_path.to_str().expect("db path is not valid UTF-8")
                );

                let sqlite_pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to connect to SQLite");

                // Run embedded migrations from src-tauri/migrations/
                // Path is relative to CARGO_MANIFEST_DIR (= src-tauri/)
                sqlx::migrate!("./migrations")
                    .run(&sqlite_pool)
                    .await
                    .expect("Failed to run SQLite migrations");

                app_handle.manage(sqlite_pool);

                // MySQL pool starts as None; the frontend connects via settings commands
                let mysql_state: MySqlState = Arc::new(Mutex::new(None));
                app_handle.manage(mysql_state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Screening
            commands::screening::search_tb_patients,
            commands::screening::get_dispensing_history,
            // Patients
            commands::patients::enroll_patient,
            commands::patients::get_active_patients,
            commands::patients::get_patient_detail,
            commands::patients::discharge_patient,
            commands::patients::get_discharged_patients,
            // Follow-ups & treatment plans
            commands::followups::add_followup,
            commands::followups::update_treatment_phase,
            // Alerts
            commands::alerts::get_patient_alerts,
            // Settings / connection management
            commands::settings::test_mysql_connection,
            commands::settings::connect_mysql,
            commands::settings::get_mysql_status,
            commands::settings::backup_sqlite,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}