use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::fs;
use std::io::{self, Read};

/// Encodes the content of the file to a Base64 string.
pub fn encode_to_base64(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(STANDARD.encode(&content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_encode_empty_file() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let file_path = temp_file.path().to_str().unwrap();
        let encoded = encode_to_base64(file_path)?;
        assert_eq!(encoded, "");
        Ok(())
    }

    #[test]
    fn test_encode_nonexistent_file() {
        let file_path = "nonexistent_file.txt";
        let result = encode_to_base64(file_path);
        assert!(result.is_err());
    }
}



