use crate::errors::ComponentError;

use super::Component;

pub struct Debug {
    pub message: String,
}

impl Component<String> for Debug {
    fn execute(&self) -> Result<String, ComponentError> {
        Ok(self.message.clone())
    }
}
