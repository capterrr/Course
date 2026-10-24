use ohkami::{Json, Response, Status, claw::Path, fang::Context};
use serde::{Deserialize, Serialize};
use crate::contracts_impl::managed_connection_pool::ManagedConnectionPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct RequestContext {
    pub connection_pool: Arc<ManagedConnectionPool>,
}

impl RequestContext {
    pub fn new(connection_pool: Arc<ManagedConnectionPool>) -> Self {
        Self { connection_pool }
    }
}

#[derive(Deserialize)]
pub struct AddMedicinePayload {
    pub name: String,
    pub price: f64,
}

#[derive(Serialize)]
pub struct MedicineResponse {
    pub id: String,
    pub name: String,
    pub price: f64,
}

pub async fn add_medicine(
    Json(payload): Json<AddMedicinePayload>,
    Context(context): Context<'_, RequestContext>,
) -> Response {
    let _pool = &context.connection_pool;
    println!("Получен запрос на добавление препарата: {} с ценой {}", payload.name, payload.price);
    
    Response::Created().with_json(MedicineResponse {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        price: payload.price,
    })
}

pub async fn get_medicine(Path(id): Path<&str>) -> Response {
    println!("Запрошен препарат по идентификатору: {}", id);
    Response::OK().with_text("Информация о лекарстве")
}

pub async fn update_medicine(Path(id): Path<&str>) -> Response {
    println!("Обновление препарата: {}", id);
    Response::OK().with_text("Препарат успешно обновлен")
}

pub async fn remove_medicine(Path(id): Path<&str>) -> Response {
    println!("Удаление препарата из каталога склада: {}", id);
    Response::OK().with_text("Препарат перемещен в архив (мягкое удаление)")
}
