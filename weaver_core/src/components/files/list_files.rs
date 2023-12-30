use std::{
    fs::{self},
    path::Path,
};

use crate::{components::Component, errors::ComponentError};

//TODO: input maybe provided from context not as a struct field?
#[derive(Debug)]
pub struct ListFiles {
    //TODO: recursive flag? (and depth?)
    pub path: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
}

impl FileType {
    pub fn from(metadata: &fs::Metadata) -> Self {
        if metadata.is_dir() {
            FileType::Directory
        } else {
            FileType::File
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub file_type: FileType,
}

const DIR_READ: &str = "DIRECTORY_READ";
const NOT_A_DIR: &str = "NOT_A_DIRECTORY";
const ACCESS_ERROR: &str = "ACCESS_ERROR";

impl Component<Vec<FileInfo>> for ListFiles {
    fn execute(&self) -> Result<Vec<FileInfo>, ComponentError> {
        println!("Listing files in {}", self.path);
        let path = Path::new(&self.path);
        if !path.is_dir() {
            return Err(ComponentError::new(
                NOT_A_DIR,
                format!("{} is not a directory", self.path),
            ));
        }
        let mut files = Vec::new();
        for file in fs::read_dir(path).map_err(|e| ComponentError::new(DIR_READ, e.to_string()))? {
            files.push(as_file_info(file)?);
        }
        Ok(files)
    }
}

fn as_file_info(file: Result<fs::DirEntry, std::io::Error>) -> Result<FileInfo, ComponentError> {
    let file = file.map_err(|e| ComponentError::new(ACCESS_ERROR, e.to_string()))?;
    let metadata = file
        .metadata()
        .map_err(|e| ComponentError::new(ACCESS_ERROR, e.to_string()))?;
    let path = file.path();
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or("".to_string());
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "".to_string());
    Ok(FileInfo {
        name,
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        extension,
        file_type: FileType::from(&metadata),
    })
}

#[cfg(test)]
mod test {
    use crate::{
        components::{files::list_files::NOT_A_DIR, Component},
        errors::ComponentError,
    };

    use super::*;

    #[test]
    fn should_list_files_in_directory() {
        let component = ListFiles {
            path: "tests/resources".to_string(),
        };

        let result = component.execute();

        let mut result = result.expect("Failed to list files in test dir");
        result.sort_unstable_by_key(|fi| fi.name.clone());

        assert_eq!(
            result,
            vec![
                FileInfo {
                    name: "dir1".to_string(),
                    path: "tests/resources/dir1".to_string(),
                    size: 18,
                    extension: "".to_string(),
                    file_type: FileType::Directory
                },
                FileInfo {
                    name: "file1.txt".to_string(),
                    path: "tests/resources/file1.txt".to_string(),
                    size: 15,
                    extension: "txt".to_string(),
                    file_type: FileType::File
                },
                FileInfo {
                    name: "file2.txt".to_string(),
                    path: "tests/resources/file2.txt".to_string(),
                    size: 15,
                    extension: "txt".to_string(),
                    file_type: FileType::File
                },
                FileInfo {
                    name: "file3.txt".to_string(),
                    path: "tests/resources/file3.txt".to_string(),
                    size: 15,
                    extension: "txt".to_string(),
                    file_type: FileType::File
                }
            ]
        );
    }

    #[test]
    fn should_return_error_if_path_is_not_a_directory() {
        let component = ListFiles {
            path: "list-files.rs".to_string(),
        };

        let result = component.execute();
        assert_eq!(
            result.unwrap_err(),
            ComponentError::new(NOT_A_DIR, "list-files.rs is not a directory".to_string())
        );
    }
}
