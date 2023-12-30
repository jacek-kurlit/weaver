#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentError {
    pub code: String,
    pub message: String,
}

impl ComponentError {
    pub fn new(code: &str, message: String) -> Self {
        Self {
            code: code.to_string(),
            message,
        }
    }
}
