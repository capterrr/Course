use regex::Regex;
use std::sync::LazyLock;

static PHONE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\A7[0-9]{10}\z").expect("Ошибочное регулярное выражение номера телефона")
});

pub enum PhoneNumberError {
    EmptyNumber,
    InvalidFormat,
}

#[derive(Clone, Debug)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(value: String) -> Result<Self, PhoneNumberError> {
        if value.trim().is_empty() {
            return Err(PhoneNumberError::EmptyNumber);
        }
        if !PHONE_REGEX.is_match(&value) {
            return Err(PhoneNumberError::InvalidFormat);
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
