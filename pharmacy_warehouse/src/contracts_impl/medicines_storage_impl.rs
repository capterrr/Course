use crate::contracts::MedicinesStorage;
use crate::entities::medicine::Medicine;
use async_sqlite::Client;
use uuid::Uuid;

pub struct MedicinesSqliteStorage {
    pub connection: Client,
}

impl MedicinesSqliteStorage {
    pub fn new(connection: Client) -> Self {
        Self { connection }
    }
}

impl MedicinesStorage for MedicinesSqliteStorage {
    async fn add(&self, medicine: Medicine) -> Result<(), Box<dyn std::error::Error>> {
        // Конкретная SQL логика добавления лекарства
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<Medicine>, Box<dyn std::error::Error>> {
        // Конкретная SQL логика выборки из таблицы medicines
        Ok(None)
    }

    async fn update(&self, medicine: &Medicine) -> Result<(), Box<dyn std::error::Error>> {
        // Конкретная SQL логика обновления данных препарата
        Ok(())
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        // Конкретная SQL логика DELETE
        Ok(())
    }
}
