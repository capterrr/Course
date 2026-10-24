use time::UtcDateTime;

pub enum RecordLifetimeError {
    UpdatedBeforeCreated,
    DeletedBeforeCreated,
}

#[derive(Clone, Copy, Debug)]
pub struct RecordLifetime {
    pub created_at: UtcDateTime,
    pub updated_at: Option<UtcDateTime>,
    pub deleted_at: Option<UtcDateTime>,
}

impl RecordLifetime {
    pub fn create(
        created_at: UtcDateTime,
        updated_at: Option<UtcDateTime>,
        deleted_at: Option<UtcDateTime>,
    ) -> Result<Self, RecordLifetimeError> {
        if let Some(up) = updated_at {
            if up < created_at {
                return Err(RecordLifetimeError::UpdatedBeforeCreated);
            }
        }
        if let Some(del) = deleted_at {
            if del < created_at {
                return Err(RecordLifetimeError::DeletedBeforeCreated);
            }
        }
        Ok(Self { created_at, updated_at, deleted_at })
    }
}
