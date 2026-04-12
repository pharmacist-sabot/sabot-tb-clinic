use tauri::State;
use sqlx::SqlitePool;
use chrono::{Datelike, Local, NaiveDate};
use crate::commands::settings::MySqlState;
use crate::models::alert::PatientAlert;
use crate::db;

#[tauri::command]
pub async fn get_patient_alerts(
    sqlite: State<'_, SqlitePool>,
    mysql: State<'_, MySqlState>,
) -> Result<Vec<PatientAlert>, String> {
    let patients = db::sqlite::get_active_patients(&sqlite)
        .await
        .map_err(|e| e.to_string())?;

    let mysql_guard = mysql.lock().await;
    let mysql_pool = mysql_guard.as_ref();

    let today = Local::now().date_naive();
    let mut all_alerts: Vec<PatientAlert> = Vec::new();

    for patient in &patients {
        let current_plan = db::sqlite::get_current_treatment_plan(&sqlite, &patient.hn)
            .await
            .ok()
            .flatten();

        let first_start = db::sqlite::get_first_phase_start(&sqlite, &patient.hn)
            .await
            .ok()
            .flatten();

        let current_month = first_start.as_ref().and_then(|s| {
            NaiveDate::parse_from_str(s, "%Y-%m-%d").ok().map(|start| {
                let months = (today.year() - start.year()) * 12
                    + (today.month() as i32 - start.month() as i32);
                (months + 1).max(1) as i64
            })
        });

        let total_months = db::sqlite::get_all_treatment_plans(&sqlite, &patient.hn)
            .await
            .ok()
            .map(|plans| plans.iter().map(|p| p.duration_months).sum::<i64>());

        let days_since_last = if let Some(pool) = mysql_pool {
            db::mysql::get_last_dispensing_date(pool, &patient.hn)
                .await
                .ok()
                .flatten()
                .and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok())
                .map(|d| (today - d).num_days())
        } else {
            None
        };

        // 1. Overdue dispensing (> 35 days, not yet lost to follow-up)
        if let Some(days) = days_since_last {
            if days > 35 && days <= 60 {
                all_alerts.push(PatientAlert {
                    hn: patient.hn.clone(),
                    alert_type: "overdue".to_string(),
                    severity: "red".to_string(),
                    message: format!("ไม่ได้รับยานาน {} วัน", days),
                    details: None,
                });
            }
        }

        // 2. Lost to follow-up (> 60 days)
        if let Some(days) = days_since_last {
            if days > 60 {
                all_alerts.push(PatientAlert {
                    hn: patient.hn.clone(),
                    alert_type: "lost_to_followup".to_string(),
                    severity: "red".to_string(),
                    message: format!("ขาดการติดตาม {} วัน", days),
                    details: None,
                });
            }
        }

        // 3. Ethambutol overrun & phase-transition alerts (requires MySQL)
        if let Some(pool) = mysql_pool {
            // 3a. Patient is in continuation phase but E was dispensed recently
            if let Some(plan) = &current_plan {
                if plan.phase == "continuation" {
                    if let Ok(true) =
                        db::mysql::was_ethambutol_dispensed_recently(pool, &patient.hn, 30).await
                    {
                        all_alerts.push(PatientAlert {
                            hn: patient.hn.clone(),
                            alert_type: "ethambutol_overrun".to_string(),
                            severity: "red".to_string(),
                            message: "ได้รับ Ethambutol เกินระยะ Intensive Phase".to_string(),
                            details: None,
                        });
                    }
                }
            }

            // 3b. Still in intensive phase but expected end date has passed
            if let Some(plan) = &current_plan {
                if plan.phase == "intensive" {
                    if let Some(end_str) = &plan.phase_end_expected {
                        if let Ok(end_date) = NaiveDate::parse_from_str(end_str, "%Y-%m-%d") {
                            if today > end_date {
                                all_alerts.push(PatientAlert {
                                    hn: patient.hn.clone(),
                                    alert_type: "phase_transition".to_string(),
                                    severity: "yellow".to_string(),
                                    message: "ถึงเวลาเปลี่ยนเป็น Continuation Phase".to_string(),
                                    details: Some(format!("Phase end expected: {}", end_str)),
                                });
                            }
                        }
                    }
                }
            }
        }

        // 4. Total treatment duration exceeded
        if let (Some(cur_month), Some(total)) = (current_month, total_months) {
            if cur_month > total {
                all_alerts.push(PatientAlert {
                    hn: patient.hn.clone(),
                    alert_type: "treatment_complete".to_string(),
                    severity: "yellow".to_string(),
                    message: "ครบกำหนดระยะการรักษาแล้ว".to_string(),
                    details: None,
                });
            }
        }
    }

    Ok(all_alerts)
}