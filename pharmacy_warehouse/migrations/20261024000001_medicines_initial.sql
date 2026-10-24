CREATE TABLE IF NOT EXISTS medicines (
    id BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_medicines_name ON medicines(name);
