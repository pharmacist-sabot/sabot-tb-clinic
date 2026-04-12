CREATE TABLE IF NOT EXISTS tb_patients (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    hn              TEXT NOT NULL UNIQUE,
    enrolled_at     TEXT NOT NULL,
    enrolled_by     TEXT,
    status          TEXT NOT NULL DEFAULT 'active',
    tb_type         TEXT,
    diagnosis_date  TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tb_treatment_plans (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    hn                  TEXT NOT NULL,
    regimen             TEXT NOT NULL,
    phase               TEXT NOT NULL,
    phase_start         TEXT NOT NULL,
    phase_end_expected  TEXT,
    drugs               TEXT NOT NULL,
    duration_months     INTEGER NOT NULL,
    is_current          INTEGER NOT NULL DEFAULT 1,
    notes               TEXT,
    created_at          TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tb_followups (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    hn              TEXT NOT NULL,
    followup_date   TEXT NOT NULL,
    month_number    INTEGER,
    weight_kg       REAL,
    sputum_result   TEXT,
    xray_result     TEXT,
    side_effects    TEXT,
    adherence       TEXT,
    dispensed_drugs TEXT,
    notes           TEXT,
    created_by      TEXT,
    created_at      TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tb_outcomes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    hn              TEXT NOT NULL UNIQUE,
    outcome         TEXT NOT NULL,
    outcome_date    TEXT NOT NULL,
    treatment_end   TEXT,
    notes           TEXT,
    created_by      TEXT,
    created_at      TEXT NOT NULL
);