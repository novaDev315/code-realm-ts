// Reference solutions for Chapter 11 - URL Shortener Base62 Encoding

/// Character set for base62 encoding
/// 0-9 (10) + A-Z (26) + a-z (26) = 62 characters
pub const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Convert a number to a base62 string
/// Used for generating short codes from database IDs
pub fn base62_encode(id: u64) -> String {
    // Handle edge case for zero
    if id == 0 {
        return "0".to_string();
    }

    let mut num = id;
    let mut result = Vec::new();
    let base: u64 = 62;

    // Build the base62 string by repeatedly dividing by 62
    while num > 0 {
        let remainder = (num % base) as usize;
        result.push(BASE62_CHARS[remainder] as char);
        num /= base;
    }

    // Reverse the string since we built it backwards
    result.reverse();
    result.into_iter().collect()
}

/// Convert a base62 string back to a number
/// Used for looking up the original ID from a short code
pub fn base62_decode(short_code: &str) -> u64 {
    let mut result: u64 = 0;
    let base: u64 = 62;

    // Process each character from left to right
    for c in short_code.chars() {
        // Find the index of this character in our character set
        let index = BASE62_CHARS.iter().position(|&x| x == c as u8);
        match index {
            Some(idx) => {
                result = result * base + idx as u64;
            }
            None => {
                // Invalid character, return 0 (could also panic or return Result)
                return 0;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_zero() {
        assert_eq!(base62_encode(0), "0");
    }

    #[test]
    fn test_encode_single_digits() {
        assert_eq!(base62_encode(1), "1");
        assert_eq!(base62_encode(9), "9");
        assert_eq!(base62_encode(10), "A");
        assert_eq!(base62_encode(35), "Z");
        assert_eq!(base62_encode(36), "a");
        assert_eq!(base62_encode(61), "z");
    }

    #[test]
    fn test_encode_multi_digit() {
        assert_eq!(base62_encode(62), "10");
        assert_eq!(base62_encode(3844), "100");
        assert_eq!(base62_encode(238327), "zzz");
        assert_eq!(base62_encode(238328), "1000");
    }

    #[test]
    fn test_encode_large_numbers() {
        assert_eq!(base62_encode(12345678), "pnfq");
        assert_eq!(base62_encode(999999999), "15ftgF");
    }

    #[test]
    fn test_decode_zero() {
        assert_eq!(base62_decode("0"), 0);
    }

    #[test]
    fn test_decode_single_chars() {
        assert_eq!(base62_decode("1"), 1);
        assert_eq!(base62_decode("9"), 9);
        assert_eq!(base62_decode("A"), 10);
        assert_eq!(base62_decode("Z"), 35);
        assert_eq!(base62_decode("a"), 36);
        assert_eq!(base62_decode("z"), 61);
    }

    #[test]
    fn test_decode_multi_char() {
        assert_eq!(base62_decode("10"), 62);
        assert_eq!(base62_decode("100"), 3844);
        assert_eq!(base62_decode("zzz"), 238327);
        assert_eq!(base62_decode("1000"), 238328);
    }

    #[test]
    fn test_decode_large_numbers() {
        assert_eq!(base62_decode("pnfq"), 12345678);
        assert_eq!(base62_decode("15ftgF"), 999999999);
    }

    #[test]
    fn test_round_trip() {
        let test_values = vec![0, 1, 62, 3844, 12345678, 999999999, 9876543210];
        for num in test_values {
            let encoded = base62_encode(num);
            let decoded = base62_decode(&encoded);
            assert_eq!(decoded, num, "Round-trip failed for {}: encoded as '{}'", num, encoded);
        }
    }

    #[test]
    fn test_character_validity() {
        let test_values = vec![123, 456789, 9876543210];
        for num in test_values {
            let encoded = base62_encode(num);
            for c in encoded.chars() {
                assert!(BASE62_CHARS.contains(&(c as u8)), "Invalid character '{}' in encoding of {}", c, num);
            }
        }
    }
}
