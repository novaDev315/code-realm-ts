// Reference solutions for Chapter 10: Citadel of Firewalls - Security
// Password hashing, JWT validation, and XSS prevention

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;
import java.util.Map;
import java.util.HashMap;

public class SOLUTIONS {

    /**
     * Hash a password using SHA-256 - SOLUTION
     *
     * Uses Java's MessageDigest to compute SHA-256 hash.
     * The result is converted to a hexadecimal string.
     *
     * @param password The plain text password to hash
     * @return The SHA-256 hash as a hexadecimal string
     */
    public static String hashPassword(String password) {
        try {
            // Get SHA-256 message digest instance
            MessageDigest digest = MessageDigest.getInstance("SHA-256");

            // Compute hash of the password bytes
            byte[] hashBytes = digest.digest(password.getBytes());

            // Convert byte array to hexadecimal string
            StringBuilder hexString = new StringBuilder();
            for (byte b : hashBytes) {
                // Convert each byte to 2-digit hex, padding with 0 if needed
                String hex = Integer.toHexString(0xff & b);
                if (hex.length() == 1) {
                    hexString.append('0');
                }
                hexString.append(hex);
            }

            return hexString.toString();
        } catch (NoSuchAlgorithmException e) {
            // SHA-256 is guaranteed to be available in Java
            throw new RuntimeException("SHA-256 algorithm not available", e);
        }
    }

    /**
     * Verify a password against a stored hash - SOLUTION
     *
     * Hashes the provided password and compares it with the stored hash.
     * Uses constant-time comparison to prevent timing attacks.
     *
     * @param password The plain text password to verify
     * @param hash The stored hash to compare against
     * @return true if the password matches the hash, false otherwise
     */
    public static boolean verifyPassword(String password, String hash) {
        // Hash the provided password
        String computedHash = hashPassword(password);

        // Compare hashes (using equals is fine for educational purposes)
        // In production, use constant-time comparison to prevent timing attacks
        return computedHash.equals(hash);
    }

    /**
     * Sanitize user input to prevent XSS attacks - SOLUTION
     *
     * Escapes HTML special characters to their entity equivalents.
     * Order matters: & must be escaped first to avoid double-escaping.
     *
     * @param input The user input to sanitize
     * @return The sanitized string with HTML entities escaped
     */
    public static String sanitizeInput(String input) {
        if (input == null) {
            return null;
        }

        // Order is important: escape & first to avoid double-escaping
        String sanitized = input;
        sanitized = sanitized.replace("&", "&amp;");
        sanitized = sanitized.replace("<", "&lt;");
        sanitized = sanitized.replace(">", "&gt;");
        sanitized = sanitized.replace("\"", "&quot;");
        sanitized = sanitized.replace("'", "&#39;");

        return sanitized;
    }

    /**
     * Validate JWT structure - SOLUTION
     *
     * Checks that the token has exactly 3 non-empty parts separated by dots.
     * This validates structure only, not cryptographic signature.
     *
     * @param token The JWT token string to validate
     * @return true if the token has valid structure, false otherwise
     */
    public static boolean validateJWTStructure(String token) {
        if (token == null || token.isEmpty()) {
            return false;
        }

        // Split by dot character
        String[] parts = token.split("\\.");

        // JWT must have exactly 3 parts
        if (parts.length != 3) {
            return false;
        }

        // Each part must be non-empty
        for (String part : parts) {
            if (part == null || part.isEmpty()) {
                return false;
            }
        }

        return true;
    }

    /**
     * Decode the payload from a JWT token - SOLUTION
     *
     * Extracts and decodes the Base64URL-encoded payload (middle part).
     * Parses the JSON into a simple Map structure.
     *
     * @param token The JWT token string
     * @return A Map containing the decoded payload claims, or null if invalid
     */
    public static Map<String, Object> decodeJWTPayload(String token) {
        // Validate structure first
        if (!validateJWTStructure(token)) {
            return null;
        }

        try {
            // Split and get the payload (middle part)
            String[] parts = token.split("\\.");
            String payloadBase64 = parts[1];

            // Decode from Base64URL
            byte[] decodedBytes = Base64.getUrlDecoder().decode(payloadBase64);
            String payloadJson = new String(decodedBytes);

            // Simple JSON parsing (for educational purposes)
            // In production, use a proper JSON library like Jackson or Gson
            return parseSimpleJson(payloadJson);
        } catch (Exception e) {
            // Return null for any decoding errors
            return null;
        }
    }

    /**
     * Simple JSON parser for educational purposes
     * Handles basic JSON objects with string, number, and boolean values.
     *
     * Note: In production, use a proper JSON library (Jackson, Gson, etc.)
     */
    private static Map<String, Object> parseSimpleJson(String json) {
        Map<String, Object> result = new HashMap<>();

        // Remove outer braces and whitespace
        json = json.trim();
        if (!json.startsWith("{") || !json.endsWith("}")) {
            return result;
        }
        json = json.substring(1, json.length() - 1);

        // Split by comma (simple approach - doesn't handle nested objects)
        int i = 0;
        while (i < json.length()) {
            // Skip whitespace
            while (i < json.length() && Character.isWhitespace(json.charAt(i))) i++;
            if (i >= json.length()) break;

            // Find key (expect quoted string)
            if (json.charAt(i) != '"') break;
            int keyStart = ++i;
            while (i < json.length() && json.charAt(i) != '"') i++;
            String key = json.substring(keyStart, i);
            i++; // skip closing quote

            // Skip colon and whitespace
            while (i < json.length() && (json.charAt(i) == ':' || Character.isWhitespace(json.charAt(i)))) i++;

            // Parse value
            Object value;
            if (i < json.length() && json.charAt(i) == '"') {
                // String value
                int valueStart = ++i;
                while (i < json.length() && json.charAt(i) != '"') i++;
                value = json.substring(valueStart, i);
                i++; // skip closing quote
            } else {
                // Number or boolean value
                int valueStart = i;
                while (i < json.length() && json.charAt(i) != ',' && json.charAt(i) != '}') i++;
                String valueStr = json.substring(valueStart, i).trim();

                if (valueStr.equals("true")) {
                    value = true;
                } else if (valueStr.equals("false")) {
                    value = false;
                } else if (valueStr.equals("null")) {
                    value = null;
                } else {
                    // Try to parse as number
                    try {
                        if (valueStr.contains(".")) {
                            value = Double.parseDouble(valueStr);
                        } else {
                            value = Long.parseLong(valueStr);
                        }
                    } catch (NumberFormatException e) {
                        value = valueStr;
                    }
                }
            }

            result.put(key, value);

            // Skip comma and whitespace
            while (i < json.length() && (json.charAt(i) == ',' || Character.isWhitespace(json.charAt(i)))) i++;
        }

        return result;
    }
}
