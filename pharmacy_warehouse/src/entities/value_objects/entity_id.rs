use uuid::Uuid;

pub enum EntityIdError {
    EmptyId,
    InvalidId,
}

#[derive(Clone, Copy, Debug)]
pub struct EntityId(Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn create(id: Uuid) -> Result<Self, EntityIdError> {
        if id.is_nil() {
            return Err(EntityIdError::EmptyId);
        }
        if id.get_variant() != uuid::Variant::RFC4122 || id.get_version() != Some(uuid::Version::Random) {
            return Err(EntityIdError::InvalidId);
        }
        Ok(Self(id))
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}
