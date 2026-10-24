CREATE TABLE IF NOT EXISTS pharmacies (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_pharmacies_phone ON pharmacies(phone_number);
