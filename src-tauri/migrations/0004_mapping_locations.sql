CREATE TABLE IF NOT EXISTS tb_patient_locations (
    hn                  TEXT PRIMARY KEY,
    raw_address         TEXT NOT NULL,
    normalized_address  TEXT,
    lat                 REAL,
    lng                 REAL,
    jittered_lat        REAL,
    jittered_lng        REAL,
    geocode_status      TEXT NOT NULL DEFAULT 'pending',
    geocode_error       TEXT,
    geocode_attempts    INTEGER NOT NULL DEFAULT 0,
    geocoded_at         TEXT,
    updated_at          TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tb_patient_locations_status
    ON tb_patient_locations (geocode_status);

CREATE INDEX IF NOT EXISTS idx_tb_patient_locations_coords
    ON tb_patient_locations (jittered_lat, jittered_lng);
