use crate::entities::{EntityId, ItemQuantity, RecordLifetime, DomainError};
use uuid::Uuid;
use time::UtcDateTime;

pub struct Delivery {
    pub id: EntityId,
    pub order_id: EntityId,
    pub medicine_id: EntityId,
    pub quantity: ItemQuantity,
    pub lifetime: RecordLifetime,
}

impl Delivery {
    pub fn create(
        id: Uuid,
        order_id: Uuid,
        medicine_id: Uuid,
        quantity: u32,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, DomainError> {
        let id = EntityId::create(id)?;
        let order_id = EntityId::create(order_id)?;
        let medicine_id = EntityId::create(medicine_id)?;
        let quantity = ItemQuantity::new(quantity)?;
        let lifetime = RecordLifetime::create(created_at, updated_at, deleted_at)?;

        Ok(Self { id, order_id, medicine_id, quantity, lifetime })
    }
}
