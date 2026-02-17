use std::path::Path;

const SUPPORTED_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "ogg", "aac", "m4a", "opus", "wma", "aiff", "aif", "ape",
];

#[derive(Debug)]
pub enum FileError {
    NotFound(String),
    UnsupportedFormat(String),
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "File not found: {path}"),
            FileError::UnsupportedFormat(path) => write!(f, "Unsupported audio format: {path}"),
        }
    }
}

pub fn validate_audio_file(path: &str) -> Result<(), FileError> {
    let p = Path::new(path);

    if !p.exists() {
        return Err(FileError::NotFound(path.to_string()));
    }

    if !is_audio_file(p) {
        return Err(FileError::UnsupportedFormat(path.to_string()));
    }

    Ok(())
}

fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_not_found() {
        let result = validate_audio_file("/nonexistent/path/song.mp3");
        assert!(matches!(result, Err(FileError::NotFound(_))));
    }

    #[test]
    fn test_unsupported_format() {
        let tmp = std::env::temp_dir().join("test_audio.txt");
        fs::write(&tmp, b"").unwrap();
        let result = validate_audio_file(tmp.to_str().unwrap());
        fs::remove_file(&tmp).unwrap();
        assert!(matches!(result, Err(FileError::UnsupportedFormat(_))));
    }

    #[test]
    fn test_supported_formats() {
        for ext in SUPPORTED_EXTENSIONS {
            let tmp = std::env::temp_dir().join(format!("laffey_fmt_{ext}.{ext}"));
            fs::write(&tmp, b"").unwrap();
            let result = validate_audio_file(tmp.to_str().unwrap());
            fs::remove_file(&tmp).unwrap();
            assert!(result.is_ok(), "Extension .{ext} should be supported");
        }
    }

    #[test]
    fn test_case_insensitive_extension() {
        let tmp = std::env::temp_dir().join("laffey_case.MP3");
        fs::write(&tmp, b"").unwrap();
        let result = validate_audio_file(tmp.to_str().unwrap());
        fs::remove_file(&tmp).unwrap();
        assert!(result.is_ok());
    }
}
