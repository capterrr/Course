pub mod value_objects;
pub mod medicine;
pub mod pharmacy;
pub mod order;
pub mod delivery;

pub use value_objects::*;

pub enum DomainError {
    EntityIdError(EntityIdError),
    NameError(NameError),
    PriceError(PriceError),
    ItemQuantityError(ItemQuantityError),
    PhoneNumberError(PhoneNumberError),
    RecordLifetimeError(RecordLifetimeError),
}

impl From<EntityIdError> for DomainError { fn from(err: EntityIdError) -> Self { DomainError::EntityIdError(err) } }
impl From<NameError> for DomainError { fn from(err: NameError) -> Self { DomainError::NameError(err) } }
impl From<PriceError> for DomainError { fn from(err: PriceError) -> Self { DomainError::PriceError(err) } }
impl From<ItemQuantityError> for DomainError { fn from(err: ItemQuantityError) -> Self { DomainError::ItemQuantityError(err) } }
impl From<PhoneNumberError> for DomainError { fn from(err: PhoneNumberError) -> Self { DomainError::PhoneNumberError(err) } }
impl From<RecordLifetimeError> for DomainError { fn from(err: RecordLifetimeError) -> Self { DomainError::RecordLifetimeError(err) } }
