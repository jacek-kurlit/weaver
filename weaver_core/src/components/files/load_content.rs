use crate::errors::Error;

pub struct LoadContent {
    pub file_path: String,
}

impl LoadContent {
    pub fn execute(&self) -> Result<String, Error> {
        println!("Loading content from {}", self.file_path);
        let content = std::fs::read_to_string(&self.file_path).map_err(|e| e.to_string())?;
        Ok(content)
    }
}
