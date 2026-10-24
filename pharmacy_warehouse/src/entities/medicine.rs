use crate::entities::{EntityId, Name, Price, RecordLifetime, DomainError};
use uuid::Uuid;
use time::UtcDateTime;

pub struct Medicine {
    pub id: EntityId,
    pub name: Name,
    pub price: Price,
    pub lifetime: RecordLifetime,
}

impl Medicine {
    pub fn create(
        id: Uuid,
        name: String,
        price: f64,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, DomainError> {
        let id = EntityId::create(id)?;
        let name = Name::new(name)?;
        let price = Price::new(price)?;
        let lifetime = RecordLifetime::create(created_at, updated_at, deleted_at)?;

        Ok(Self { id, name, price, lifetime })
    }
}
