use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// Decodes a Base64 string and returns the decoded content as a String.
pub fn decode_from_base64(encoded: &str) -> Result<String, Box<dyn std::error::Error>> {
    let decoded_bytes = STANDARD.decode(encoded.trim_matches('~'))?;
    let decoded_string = String::from_utf8(decoded_bytes)?;
    Ok(decoded_string)
}