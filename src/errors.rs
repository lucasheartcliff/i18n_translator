use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct TranslateError {
    code: u16,
    message: String,
}

impl TranslateError {
    pub fn new(code: u16, message: &str) -> Self {
        TranslateError {
            code,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for TranslateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl Error for TranslateError {}
