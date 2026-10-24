use crate::entities::medicine::Medicine;
use uuid::Uuid;
use std::future::Future;

pub trait MedicinesStorage {
    fn add(&self, medicine: Medicine) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
    fn get(&self, id: Uuid) -> impl Future<Output = Result<Option<Medicine>, Box<dyn std::error::Error>>>;
    fn update(&self, medicine: &Medicine) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
    fn remove(&self, id: Uuid) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>>;
}
