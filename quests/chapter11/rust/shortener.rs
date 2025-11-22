// Chapter 11: Tower of Constructs - URL Shortener
// Design a scalable URL shortening system with base62 encoding

/// Character set for base62 encoding
/// 0-9 (10) + A-Z (26) + a-z (26) = 62 characters
pub const BASE62_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Convert a number to a base62 string
/// Used for generating short codes from database IDs
/// Example: 0 -> "0", 62 -> "10", 3844 -> "100"
pub fn base62_encode(id: u64) -> String {
    // TODO: Convert number to base62 string
    // - Handle edge case where id is 0 (return "0")
    // - Build result by repeatedly dividing by 62
    // - Use BASE62_CHARS to map remainders to characters
    // - Remember to reverse or prepend characters
    String::new()
}

/// Convert a base62 string back to a number
/// Used for looking up the original ID from a short code
/// Example: "0" -> 0, "10" -> 62, "100" -> 3844
pub fn base62_decode(short_code: &str) -> u64 {
    // TODO: Convert base62 string to number
    // - Process each character from left to right
    // - Find the index of each character in BASE62_CHARS
    // - Multiply running total by 62 and add the index
    0
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
        assert_eq!(base62_encode(238328), "1000");
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
        assert_eq!(base62_decode("1000"), 238328);
    }

    #[test]
    fn test_round_trip() {
        let test_values = vec![0, 1, 62, 3844, 12345678, 999999999];
        for num in test_values {
            let encoded = base62_encode(num);
            let decoded = base62_decode(&encoded);
            assert_eq!(decoded, num, "Round-trip failed for {}", num);
        }
    }
}
