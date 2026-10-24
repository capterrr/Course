pub enum PriceError {
    PriceIsNegative,
}

#[derive(Clone, Copy, Debug)]
pub struct Price(f64);

impl Price {
    pub fn new(value: f64) -> Result<Self, PriceError> {
        if value < 0.0 {
            return Err(PriceError::PriceIsNegative);
        }
        Ok(Price(value))
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
