// Test file for Chapter 10: Citadel of Firewalls - Security

import java.util.Map;

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test 1: Password Hashing
        System.out.println("Testing hashPassword...");

        // Test that hashing produces consistent results
        String hash1 = Security.hashPassword("secret123");
        String hash2 = Security.hashPassword("secret123");
        if (hash1 == null || hash1.isEmpty()) {
            System.err.println("hashPassword(\"secret123\"): returned empty or null");
            passed = false;
        } else if (!hash1.equals(hash2)) {
            System.err.println("hashPassword should return same hash for same input");
            passed = false;
        } else {
            System.out.println("  hashPassword(\"secret123\") = \"" + hash1.substring(0, 16) + "...\"");
        }

        // Test that different passwords produce different hashes
        String hash3 = Security.hashPassword("different");
        if (hash1.equals(hash3)) {
            System.err.println("hashPassword should return different hashes for different inputs");
            passed = false;
        } else {
            System.out.println("  Different passwords produce different hashes");
        }

        // Test that hash is 64 characters (SHA-256 hex = 256 bits = 64 hex chars)
        if (hash1.length() != 64) {
            System.err.println("SHA-256 hash should be 64 characters, got " + hash1.length());
            passed = false;
        } else {
            System.out.println("  Hash length is correct (64 characters)");
        }

        // Test 2: Password Verification
        System.out.println("\nTesting verifyPassword...");

        String testPassword = "mypassword";
        String testHash = Security.hashPassword(testPassword);

        // Correct password should verify
        if (!Security.verifyPassword(testPassword, testHash)) {
            System.err.println("verifyPassword should return true for correct password");
            passed = false;
        } else {
            System.out.println("  verifyPassword(\"mypassword\", hash) = true");
        }

        // Wrong password should not verify
        if (Security.verifyPassword("wrongpassword", testHash)) {
            System.err.println("verifyPassword should return false for wrong password");
            passed = false;
        } else {
            System.out.println("  verifyPassword(\"wrongpassword\", hash) = false");
        }

        // Test 3: Input Sanitization (XSS Prevention)
        System.out.println("\nTesting sanitizeInput...");

        // Test HTML tag escaping
        String xssInput = "<script>alert('XSS')</script>";
        String sanitized = Security.sanitizeInput(xssInput);
        if (sanitized.contains("<") || sanitized.contains(">")) {
            System.err.println("sanitizeInput should escape < and > characters");
            System.err.println("  Input: " + xssInput);
            System.err.println("  Output: " + sanitized);
            passed = false;
        } else {
            System.out.println("  sanitizeInput(\"<script>...\") escapes angle brackets");
        }

        // Test ampersand escaping
        String ampInput = "Tom & Jerry";
        String ampSanitized = Security.sanitizeInput(ampInput);
        if (!ampSanitized.contains("&amp;")) {
            System.err.println("sanitizeInput should escape & to &amp;");
            passed = false;
        } else {
            System.out.println("  sanitizeInput(\"Tom & Jerry\") = \"" + ampSanitized + "\"");
        }

        // Test quote escaping
        String quoteInput = "He said \"hello\"";
        String quoteSanitized = Security.sanitizeInput(quoteInput);
        if (quoteSanitized.contains("\"") && !quoteSanitized.contains("&quot;")) {
            System.err.println("sanitizeInput should escape \" to &quot;");
            passed = false;
        } else {
            System.out.println("  sanitizeInput with quotes escapes correctly");
        }

        // Test single quote escaping
        String singleQuoteInput = "It's a test";
        String singleQuoteSanitized = Security.sanitizeInput(singleQuoteInput);
        if (singleQuoteSanitized.contains("'") && !singleQuoteSanitized.contains("&#39;")) {
            System.err.println("sanitizeInput should escape ' to &#39;");
            passed = false;
        } else {
            System.out.println("  sanitizeInput with single quotes escapes correctly");
        }

        // Test safe input passes through
        String safeInput = "Hello World";
        String safeSanitized = Security.sanitizeInput(safeInput);
        if (!safeSanitized.equals(safeInput)) {
            System.err.println("sanitizeInput should not modify safe input");
            passed = false;
        } else {
            System.out.println("  Safe input passes through unchanged");
        }

        // Test 4: JWT Structure Validation
        System.out.println("\nTesting validateJWTStructure...");

        // Valid JWT structure
        String validJWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        if (!Security.validateJWTStructure(validJWT)) {
            System.err.println("validateJWTStructure should return true for valid JWT");
            passed = false;
        } else {
            System.out.println("  Valid JWT structure recognized");
        }

        // Invalid: only 2 parts
        String twoParts = "header.payload";
        if (Security.validateJWTStructure(twoParts)) {
            System.err.println("validateJWTStructure should return false for 2-part token");
            passed = false;
        } else {
            System.out.println("  Two-part token rejected");
        }

        // Invalid: empty part
        String emptyPart = "header..signature";
        if (Security.validateJWTStructure(emptyPart)) {
            System.err.println("validateJWTStructure should return false for empty part");
            passed = false;
        } else {
            System.out.println("  Token with empty part rejected");
        }

        // Invalid: null token
        if (Security.validateJWTStructure(null)) {
            System.err.println("validateJWTStructure should return false for null");
            passed = false;
        } else {
            System.out.println("  Null token rejected");
        }

        // Invalid: empty string
        if (Security.validateJWTStructure("")) {
            System.err.println("validateJWTStructure should return false for empty string");
            passed = false;
        } else {
            System.out.println("  Empty string rejected");
        }

        // Test 5: JWT Payload Decoding
        System.out.println("\nTesting decodeJWTPayload...");

        // Decode valid JWT
        Map<String, Object> payload = Security.decodeJWTPayload(validJWT);
        if (payload == null) {
            System.err.println("decodeJWTPayload should return payload for valid JWT");
            passed = false;
        } else {
            System.out.println("  Decoded payload: " + payload);

            // Check for expected fields
            if (!payload.containsKey("sub")) {
                System.err.println("Payload should contain 'sub' field");
                passed = false;
            }
            if (!payload.containsKey("name")) {
                System.err.println("Payload should contain 'name' field");
                passed = false;
            }
            if (payload.containsKey("sub") && payload.containsKey("name")) {
                System.out.println("  Payload contains expected fields (sub, name)");
            }
        }

        // Invalid JWT should return null
        Map<String, Object> invalidPayload = Security.decodeJWTPayload("invalid");
        if (invalidPayload != null) {
            System.err.println("decodeJWTPayload should return null for invalid JWT");
            passed = false;
        } else {
            System.out.println("  Invalid JWT returns null");
        }

        // Summary
        System.out.println("\n" + "=".repeat(50));
        if (passed) {
            System.out.println("All security tests passed!");
        } else {
            System.out.println("Some security tests failed.");
            System.exit(1);
        }
    }
}
