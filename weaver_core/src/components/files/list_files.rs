use std::{
    fs::{self},
    path::Path,
};

pub struct ListFiles {
    //TODO: recursive flag? (and depth?)
    pub path: String,
}

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

pub struct FileInfo {
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
        // This function will return an error in the following situations, but is not
        // limited to just these cases:
        //
        // * The provided `path` doesn't exist.
        // * The process lacks permissions to view the contents.
        // * The `path` points at a non-directory file.
        // TODO: which means we may skip this check?
        // TODO: add that crate that helps with errors
        //thiserror = "1.0.44"
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
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "".to_string());
    Ok(FileInfo {
        path: path.to_string_lossy().to_string(),
        size: metadata.len(),
        extension,
        file_type: FileType::from(&metadata),
    })
}

#[cfg(test)]
mod test {

    #[test]
    fn should_list_files_in_directory() {
        //
    }

    #[test]
    fn should_return_error_if_path_is_not_a_directory() {
        //
    }
}
