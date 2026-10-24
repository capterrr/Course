use crate::entities::{EntityId, Name, PhoneNumber, RecordLifetime, DomainError};
use uuid::Uuid;
use time::UtcDateTime;

pub struct Pharmacy {
    pub id: EntityId,
    pub name: Name,
    pub phone_number: PhoneNumber,
    pub lifetime: RecordLifetime,
}

impl Pharmacy {
    pub fn create(
        id: Uuid,
        name: String,
        phone_number: String,
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, DomainError> {
        let id = EntityId::create(id)?;
        let name = Name::new(name)?;
        let phone_number = PhoneNumber::new(phone_number)?;
        let lifetime = RecordLifetime::create(created_at, updated_at, deleted_at)?;

        Ok(Self { id, name, phone_number, lifetime })
    }
}
