use std::path::Path;

#[derive(Debug)]
pub enum FileError {
    InvalidInput,
    FileNotFound
}

impl std::fmt::Display for FileError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::InvalidInput => write!(f, "Invalid file path input."),
            FileError::FileNotFound => write!(f, "File not found.")
        }
    }
}

impl std::error::Error for FileError {}

pub fn is_file_valid(path: &Path) -> Result<(), FileError>{
    let Some(path_string) = path.to_str() else {
        return Err(FileError::InvalidInput);
    };
    if path_string == "test.mp3" {
        return Ok(())
    }
    else {
        return Err(FileError::FileNotFound);
    }
}