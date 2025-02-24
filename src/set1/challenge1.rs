use crate::common::encoding::hex_to_base64;

/// Converts a hex-encoded string to a base64-encoded string.
/// Returns a result wrapping the abse64 string or an error if the conversion fails.
#[allow(dead_code)]
fn challenge1(hex_string: &str) -> Result<String, hex::FromHexError> {
    hex_to_base64(hex_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge1() {
        let hex_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let result = challenge1(hex_string).unwrap();
        assert_eq!(result, expected);
    }
}
