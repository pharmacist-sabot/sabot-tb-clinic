use tauri::State;
use sqlx::SqlitePool;
use crate::models::treatment::{FollowupInput, TreatmentPlanUpdate};

#[tauri::command]
pub async fn add_followup(
    db: State<'_, SqlitePool>,
    followup: FollowupInput,
) -> Result<i64, String> {
    crate::db::sqlite::add_followup(&db, &followup)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_treatment_phase(
    db: State<'_, SqlitePool>,
    plan: TreatmentPlanUpdate,
) -> Result<(), String> {
    crate::db::sqlite::update_treatment_phase(&db, &plan)
        .await
        .map_err(|e| e.to_string())
}