use std::error;

pub mod debug;
pub mod files;

//TODO: I don't have idea for componetn result, maybe simple struct is better and simplier???
pub type ComponentResult<T> = Result<T, dyn error::Error>;
