use base64;
use hex;

pub fn hex_to_base64(hex: &str) -> Result<String, hex::FromHexError> {
    let bytes = hex::decode(hex)?;
    let base64 = base64::encode(bytes);
    Ok(base64)
}
