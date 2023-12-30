pub struct LoadContent {
    pub file_path: String,
}

impl LoadContent {
    pub fn execute(&self) -> Result<String, String> {
        println!("Loading content from {}", self.file_path);
        std::fs::read_to_string(&self.file_path).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
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
            Err("No such file or directory (os error 2)".to_string())
        );
    }
}
