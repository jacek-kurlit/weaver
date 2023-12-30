use crate::{components::Component, errors::ComponentError};

pub struct LoadContent {
    pub file_path: String,
}

const CODE: &str = "FILE_READ_ERROR";

impl Component<String> for LoadContent {
    fn execute(&self) -> Result<String, ComponentError> {
        println!("Loading content from {}", self.file_path);
        std::fs::read_to_string(&self.file_path)
            .map_err(|e| ComponentError::new(CODE, e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::{files::load_content::CODE, Component},
        errors::ComponentError,
    };

    use super::LoadContent;

    #[test]
    fn should_load_file_content() {
        let component = LoadContent {
            file_path: "tests/resources/file1.txt".to_string(),
        };
        let result = component.execute();
        assert_eq!(result, Ok("File 1 content\n".to_string()));
    }

    #[test]
    fn should_fail_to_load_file_content_when_file_not_exist() {
        let component = LoadContent {
            file_path: "tests/resources/file_xxx.txt".to_string(),
        };
        let result = component.execute();
        assert_eq!(
            result,
            Err(ComponentError::new(
                CODE,
                "No such file or directory (os error 2)".to_string()
            ))
        );
    }
}
