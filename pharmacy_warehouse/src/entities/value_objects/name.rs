pub enum NameError {
    NameIsEmpty,
    NameTooLong,
    NameTooShort,
}

#[derive(Clone, Debug)]
pub struct Name(String);

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(NameError::NameIsEmpty);
        }
        if trimmed.len() < 3 {
            return Err(NameError::NameTooShort);
        }
        if trimmed.len() > 255 {
            return Err(NameError::NameTooLong);
        }
        Ok(Name(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
