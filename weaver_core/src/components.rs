use crate::errors::ComponentError;

pub mod debug;
pub mod files;

pub trait Component<OUTPUT> {
    fn execute(&self) -> Result<OUTPUT, ComponentError>;
}
