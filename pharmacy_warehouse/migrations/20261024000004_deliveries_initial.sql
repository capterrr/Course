CREATE TABLE IF NOT EXISTS deliveries (
    id BLOB PRIMARY KEY NOT NULL,
    order_id BLOB NOT NULL,
    medicine_id BLOB NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    deleted_at TEXT,
    CONSTRAINT fk_deliveries_order FOREIGN KEY (order_id) REFERENCES orders(id),
    CONSTRAINT fk_deliveries_medicine FOREIGN KEY (medicine_id) REFERENCES medicines(id)
);
