CREATE TABLE IF NOT EXISTS orders (
    id BLOB PRIMARY KEY NOT NULL,
    pharmacy_id BLOB NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT fk_orders_pharmacy FOREIGN KEY (pharmacy_id) REFERENCES pharmacies(id)
);
