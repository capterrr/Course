use crate::entities::{EntityId, RecordLifetime, DomainError};
use uuid::Uuid;
use time::UtcDateTime;

pub struct Order {
    pub id: EntityId,
    pub pharmacy_id: EntityId,
    pub lifetime: RecordLifetime,
}

impl Order {
    pub fn create(
        id: Uuid,
        pharmacy_id: Uuid,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, DomainError> {
        let id = EntityId::create(id)?;
        let pharmacy_id = EntityId::create(pharmacy_id)?;
        let lifetime = RecordLifetime::create(created_at, updated_at, deleted_at)?;

        Ok(Self { id, pharmacy_id, lifetime })
    }
}
