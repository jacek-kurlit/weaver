use std::{
    fs::{self},
    path::Path,
};

//TODO: input maybe provided from context not as a struct field?
#[derive(Debug)]
pub struct ListFiles {
    //TODO: recursive flag? (and depth?)
    pub path: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub file_type: FileType,
}

impl ListFiles {
    //TODO: better error type
    pub fn execute(&self) -> Result<Vec<FileInfo>, String> {
        println!("Listing files in {}", self.path);
        let path = Path::new(&self.path);
        if !path.is_dir() {
            return Err(format!("{} is not a directory", self.path));
        }
        let mut files = Vec::new();
        for file in fs::read_dir(path).map_err(|e| e.to_string())? {
            files.push(as_file_info(file)?);
        }
        Ok(files)
    }
}

fn as_file_info(file: Result<fs::DirEntry, std::io::Error>) -> Result<FileInfo, String> {
    let file = file.map_err(|e| e.to_string())?;
    let metadata = file.metadata().map_err(|e| e.to_string())?;
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
    use super::ListFiles;

    #[test]
    fn should_list_files_in_directory() {
        let component = ListFiles {
            path: "tests/resources".to_string(),
        };

        let result = component.execute();

        let mut result = result
            .expect("Failed to list files in test dir")
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>();
        result.sort_unstable();

        assert_eq!(result, vec!["file1.txt", "file2.txt", "file3.txt"]);
    }

    #[test]
    fn should_return_error_if_path_is_not_a_directory() {
        let component = ListFiles {
            path: "list-files.rs".to_string(),
        };

        let result = component.execute();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "list-files.rs is not a directory".to_string()
        );
    }
}
