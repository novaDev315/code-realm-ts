// Reference solutions for Chapter 10 - Security (Hashing, JWT, XSS Prevention)

use std::collections::HashMap;

/// Simple SHA256 implementation for educational purposes
/// In production, use the sha2 crate
fn simple_sha256(input: &[u8]) -> [u8; 32] {
    // SHA256 constants
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    // Initial hash values
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // Pre-processing: adding padding bits
    let ml = (input.len() as u64) * 8;
    let mut padded = input.to_vec();
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&ml.to_be_bytes());

    // Process each 512-bit chunk
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 64];

        // Copy chunk into first 16 words
        for (i, word) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }

        // Extend the first 16 words into the remaining 48 words
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        // Initialize working variables
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        // Compression function main loop
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        // Add the compressed chunk to the current hash value
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    // Produce the final hash value (big-endian)
    let mut result = [0u8; 32];
    for (i, &val) in h.iter().enumerate() {
        result[i*4..i*4+4].copy_from_slice(&val.to_be_bytes());
    }
    result
}

/// Hash a password with a salt using SHA256
/// Note: This is for educational purposes only - use bcrypt/argon2 in production!
pub fn hash_password(password: &str, salt: &str) -> String {
    // Combine password and salt
    let combined = format!("{}{}", password, salt);

    // Create SHA256 hash
    let hash_bytes = simple_sha256(combined.as_bytes());

    // Return hex-encoded hash
    hash_bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Verify if a password matches a hash
pub fn verify_password(password: &str, salt: &str, hash: &str) -> bool {
    // Hash the password with salt and compare
    hash_password(password, salt) == hash
}

/// Sanitize input to prevent XSS attacks
pub fn sanitize_input(input: &str) -> String {
    let mut result = input.to_string();

    // Remove script tags and their content (simple approach)
    // This handles basic cases - production would need a proper HTML parser
    while let Some(start) = result.to_lowercase().find("<script") {
        if let Some(end_tag) = result.to_lowercase().find("</script>") {
            let end = end_tag + "</script>".len();
            result = format!("{}{}", &result[..start], &result[end..]);
        } else {
            // No closing tag found, remove from <script to end
            result = result[..start].to_string();
            break;
        }
    }

    // Remove other HTML tags
    let mut clean = String::new();
    let mut in_tag = false;
    for ch in result.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => clean.push(ch),
            _ => {}
        }
    }

    // Escape dangerous characters
    clean = clean.replace('"', "&quot;");
    clean = clean.replace('\'', "&#39;");

    clean
}

/// Check if a JWT token has valid structure
/// JWT format: header.payload.signature (3 base64-encoded parts separated by dots)
pub fn validate_jwt_structure(token: &str) -> bool {
    // Handle empty token
    if token.is_empty() {
        return false;
    }

    // Split by dots
    let parts: Vec<&str> = token.split('.').collect();

    // Must have exactly 3 parts
    if parts.len() != 3 {
        return false;
    }

    // Each part must be non-empty
    parts.iter().all(|part| !part.is_empty())
}

/// Base64 URL decode (handles missing padding)
fn base64_url_decode(input: &str) -> Result<Vec<u8>, String> {
    // Standard base64 alphabet
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Convert URL-safe to standard base64
    let standard: String = input.chars().map(|c| {
        match c {
            '-' => '+',
            '_' => '/',
            _ => c,
        }
    }).collect();

    // Add padding if needed
    let padded = match standard.len() % 4 {
        2 => format!("{}==", standard),
        3 => format!("{}=", standard),
        _ => standard,
    };

    // Decode
    let mut result = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits_collected = 0;

    for ch in padded.chars() {
        if ch == '=' {
            break;
        }

        let value = ALPHABET.iter().position(|&c| c == ch as u8)
            .ok_or_else(|| format!("Invalid base64 character: {}", ch))?;

        buffer = (buffer << 6) | (value as u32);
        bits_collected += 6;

        if bits_collected >= 8 {
            bits_collected -= 8;
            result.push((buffer >> bits_collected) as u8);
            buffer &= (1 << bits_collected) - 1;
        }
    }

    Ok(result)
}

/// Simple JSON parser for flat objects with string values
/// Returns HashMap<String, String> for simplicity (educational purposes)
fn parse_simple_json(json: &str) -> Result<HashMap<String, String>, String> {
    let trimmed = json.trim();

    // Must start and end with braces
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return Err("Invalid JSON: must be an object".to_string());
    }

    let inner = &trimmed[1..trimmed.len()-1];
    let mut result = HashMap::new();

    // Simple state machine to parse key-value pairs
    let mut chars = inner.chars().peekable();

    loop {
        // Skip whitespace
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }

        // Check if we're done
        if chars.peek().is_none() {
            break;
        }

        // Expect opening quote for key
        if chars.next() != Some('"') {
            return Err("Expected '\"' for key".to_string());
        }

        // Read key
        let mut key = String::new();
        loop {
            match chars.next() {
                Some('"') => break,
                Some('\\') => {
                    // Handle escape sequences
                    if let Some(escaped) = chars.next() {
                        key.push(escaped);
                    }
                }
                Some(c) => key.push(c),
                None => return Err("Unexpected end of key".to_string()),
            }
        }

        // Skip whitespace and colon
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }
        if chars.next() != Some(':') {
            return Err("Expected ':' after key".to_string());
        }
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }

        // Read value (string or number)
        let value = if chars.peek() == Some(&'"') {
            chars.next(); // consume opening quote
            let mut val = String::new();
            loop {
                match chars.next() {
                    Some('"') => break,
                    Some('\\') => {
                        if let Some(escaped) = chars.next() {
                            val.push(escaped);
                        }
                    }
                    Some(c) => val.push(c),
                    None => return Err("Unexpected end of string value".to_string()),
                }
            }
            val
        } else {
            // Number or other literal
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c == ',' || c == '}' || c.is_whitespace() {
                    break;
                }
                val.push(chars.next().unwrap());
            }
            val
        };

        result.insert(key, value);

        // Skip whitespace
        while chars.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.next();
        }

        // Check for comma or end
        match chars.peek() {
            Some(',') => { chars.next(); }
            Some(_) | None => {}
        }
    }

    Ok(result)
}

/// Extract and decode the payload from a JWT token
/// Note: This does NOT verify the signature - for educational purposes only!
pub fn decode_jwt_payload(token: &str) -> Result<HashMap<String, String>, String> {
    // Validate structure first
    if !validate_jwt_structure(token) {
        return Err("Invalid JWT structure".to_string());
    }

    // Split token and get the payload (second part)
    let parts: Vec<&str> = token.split('.').collect();
    let payload_b64 = parts[1];

    // Base64 URL decode the payload
    let payload_bytes = base64_url_decode(payload_b64)?;

    // Convert to string
    let payload_str = String::from_utf8(payload_bytes)
        .map_err(|e| format!("Invalid UTF-8 in payload: {}", e))?;

    // Parse JSON into HashMap
    parse_simple_json(&payload_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_not_empty() {
        let hash = hash_password("password", "salt");
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_password_length() {
        let hash = hash_password("password", "salt");
        assert_eq!(hash.len(), 64); // SHA256 hex is 64 chars
    }

    #[test]
    fn test_hash_consistency() {
        let hash1 = hash_password("test", "salt");
        let hash2 = hash_password("test", "salt");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_salts_different_hashes() {
        let hash1 = hash_password("test", "salt1");
        let hash2 = hash_password("test", "salt2");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_correct() {
        let hash = hash_password("password123", "salt");
        assert!(verify_password("password123", "salt", &hash));
    }

    #[test]
    fn test_verify_password_wrong() {
        let hash = hash_password("password123", "salt");
        assert!(!verify_password("wrongpassword", "salt", &hash));
    }

    #[test]
    fn test_verify_password_wrong_salt() {
        let hash = hash_password("password123", "salt");
        assert!(!verify_password("password123", "wrongsalt", &hash));
    }

    #[test]
    fn test_sanitize_removes_script() {
        let result = sanitize_input("<script>alert('xss')</script>");
        assert!(!result.contains("<script>"));
        assert!(!result.contains("alert"));
    }

    #[test]
    fn test_sanitize_removes_html() {
        let result = sanitize_input("<b>Bold</b>");
        assert!(result.contains("Bold"));
        assert!(!result.contains("<b>"));
        assert!(!result.contains("</b>"));
    }

    #[test]
    fn test_sanitize_escapes_quotes() {
        let result = sanitize_input("Say \"Hello\"");
        assert!(result.contains("&quot;"));
        assert!(!result.contains("\""));
    }

    #[test]
    fn test_sanitize_escapes_apostrophes() {
        let result = sanitize_input("It's fine");
        assert!(result.contains("&#39;"));
        assert!(!result.contains("'"));
    }

    #[test]
    fn test_sanitize_preserves_normal_text() {
        let result = sanitize_input("Normal text");
        assert_eq!(result, "Normal text");
    }

    #[test]
    fn test_jwt_structure_valid() {
        assert!(validate_jwt_structure("header.payload.signature"));
    }

    #[test]
    fn test_jwt_structure_valid_real() {
        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature";
        assert!(validate_jwt_structure(jwt));
    }

    #[test]
    fn test_jwt_structure_invalid_two_parts() {
        assert!(!validate_jwt_structure("only.two"));
    }

    #[test]
    fn test_jwt_structure_invalid_one_part() {
        assert!(!validate_jwt_structure("one"));
    }

    #[test]
    fn test_jwt_structure_invalid_empty() {
        assert!(!validate_jwt_structure(""));
    }

    #[test]
    fn test_jwt_structure_invalid_four_parts() {
        assert!(!validate_jwt_structure("a.b.c.d"));
    }

    #[test]
    fn test_jwt_structure_invalid_empty_parts() {
        assert!(!validate_jwt_structure(".payload.signature"));
        assert!(!validate_jwt_structure("header..signature"));
        assert!(!validate_jwt_structure("header.payload."));
    }

    #[test]
    fn test_decode_jwt_payload() {
        let jwt = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = decode_jwt_payload(jwt);
        assert!(result.is_ok());

        let payload = result.unwrap();
        assert_eq!(payload.get("sub").unwrap(), "1234567890");
        assert_eq!(payload.get("name").unwrap(), "John Doe");
    }

    #[test]
    fn test_decode_jwt_payload_invalid() {
        let result = decode_jwt_payload("invalid.jwt");
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_json_parser() {
        let json = r#"{"key": "value", "num": 123}"#;
        let result = parse_simple_json(json).unwrap();
        assert_eq!(result.get("key").unwrap(), "value");
        assert_eq!(result.get("num").unwrap(), "123");
    }
}
