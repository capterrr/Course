pub enum ItemQuantityError {
    QuantityIsZeroOrNegative,
}

#[derive(Clone, Copy, Debug)]
pub struct ItemQuantity(u32);

impl ItemQuantity {
    pub fn new(value: u32) -> Result<Self, ItemQuantityError> {
        if value == 0 {
            return Err(ItemQuantityError::QuantityIsZeroOrNegative);
        }
        Ok(ItemQuantity(value))
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
