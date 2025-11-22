// Test file for Chapter 10: Citadel of Firewalls - Security

#include <iostream>
#include <string>
#include <map>
#include "security.cpp"

using namespace std;

int main() {
    bool passed = true;

    // Test 1: Password Hashing
    cout << "Testing hashPassword..." << endl;

    // Test that hashing produces consistent results
    string hash1 = hashPassword("secret123");
    string hash2 = hashPassword("secret123");
    if (hash1.empty()) {
        cerr << "hashPassword(\"secret123\"): returned empty string" << endl;
        passed = false;
    } else if (hash1 != hash2) {
        cerr << "hashPassword should return same hash for same input" << endl;
        passed = false;
    } else {
        cout << "  hashPassword(\"secret123\") = \"" << hash1.substr(0, 16) << "...\"" << endl;
    }

    // Test that different passwords produce different hashes
    string hash3 = hashPassword("different");
    if (hash1 == hash3) {
        cerr << "hashPassword should return different hashes for different inputs" << endl;
        passed = false;
    } else {
        cout << "  Different passwords produce different hashes" << endl;
    }

    // Test 2: Password Verification
    cout << "\nTesting verifyPassword..." << endl;

    string testPassword = "mypassword";
    string testHash = hashPassword(testPassword);

    // Correct password should verify
    if (!verifyPassword(testPassword, testHash)) {
        cerr << "verifyPassword should return true for correct password" << endl;
        passed = false;
    } else {
        cout << "  verifyPassword(\"mypassword\", hash) = true" << endl;
    }

    // Wrong password should not verify
    if (verifyPassword("wrongpassword", testHash)) {
        cerr << "verifyPassword should return false for wrong password" << endl;
        passed = false;
    } else {
        cout << "  verifyPassword(\"wrongpassword\", hash) = false" << endl;
    }

    // Test 3: Input Sanitization (XSS Prevention)
    cout << "\nTesting sanitizeInput..." << endl;

    // Test HTML tag escaping
    string xssInput = "<script>alert('XSS')</script>";
    string sanitized = sanitizeInput(xssInput);
    if (sanitized.find('<') != string::npos || sanitized.find('>') != string::npos) {
        cerr << "sanitizeInput should escape < and > characters" << endl;
        cerr << "  Input: " << xssInput << endl;
        cerr << "  Output: " << sanitized << endl;
        passed = false;
    } else {
        cout << "  sanitizeInput(\"<script>...\") escapes angle brackets" << endl;
    }

    // Test ampersand escaping
    string ampInput = "Tom & Jerry";
    string ampSanitized = sanitizeInput(ampInput);
    if (ampSanitized.find("&amp;") == string::npos) {
        cerr << "sanitizeInput should escape & to &amp;" << endl;
        passed = false;
    } else {
        cout << "  sanitizeInput(\"Tom & Jerry\") = \"" << ampSanitized << "\"" << endl;
    }

    // Test quote escaping
    string quoteInput = "He said \"hello\"";
    string quoteSanitized = sanitizeInput(quoteInput);
    if (quoteSanitized.find("&quot;") == string::npos) {
        cerr << "sanitizeInput should escape \" to &quot;" << endl;
        passed = false;
    } else {
        cout << "  sanitizeInput with quotes escapes correctly" << endl;
    }

    // Test single quote escaping
    string singleQuoteInput = "It's a test";
    string singleQuoteSanitized = sanitizeInput(singleQuoteInput);
    if (singleQuoteSanitized.find("&#39;") == string::npos) {
        cerr << "sanitizeInput should escape ' to &#39;" << endl;
        passed = false;
    } else {
        cout << "  sanitizeInput with single quotes escapes correctly" << endl;
    }

    // Test safe input passes through
    string safeInput = "Hello World";
    string safeSanitized = sanitizeInput(safeInput);
    if (safeSanitized != safeInput) {
        cerr << "sanitizeInput should not modify safe input" << endl;
        passed = false;
    } else {
        cout << "  Safe input passes through unchanged" << endl;
    }

    // Test 4: JWT Structure Validation
    cout << "\nTesting validateJWTStructure..." << endl;

    // Valid JWT structure
    string validJWT = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
    if (!validateJWTStructure(validJWT)) {
        cerr << "validateJWTStructure should return true for valid JWT" << endl;
        passed = false;
    } else {
        cout << "  Valid JWT structure recognized" << endl;
    }

    // Invalid: only 2 parts
    string twoParts = "header.payload";
    if (validateJWTStructure(twoParts)) {
        cerr << "validateJWTStructure should return false for 2-part token" << endl;
        passed = false;
    } else {
        cout << "  Two-part token rejected" << endl;
    }

    // Invalid: empty part
    string emptyPart = "header..signature";
    if (validateJWTStructure(emptyPart)) {
        cerr << "validateJWTStructure should return false for empty part" << endl;
        passed = false;
    } else {
        cout << "  Token with empty part rejected" << endl;
    }

    // Invalid: empty string
    if (validateJWTStructure("")) {
        cerr << "validateJWTStructure should return false for empty string" << endl;
        passed = false;
    } else {
        cout << "  Empty string rejected" << endl;
    }

    // Test 5: JWT Payload Decoding
    cout << "\nTesting decodeJWTPayload..." << endl;

    // Decode valid JWT
    map<string, string> payload = decodeJWTPayload(validJWT);
    if (payload.empty()) {
        cerr << "decodeJWTPayload should return payload for valid JWT" << endl;
        passed = false;
    } else {
        cout << "  Decoded payload:" << endl;
        for (const auto& pair : payload) {
            cout << "    " << pair.first << ": " << pair.second << endl;
        }

        // Check for expected fields
        if (payload.find("sub") == payload.end()) {
            cerr << "Payload should contain 'sub' field" << endl;
            passed = false;
        }
        if (payload.find("name") == payload.end()) {
            cerr << "Payload should contain 'name' field" << endl;
            passed = false;
        }
        if (payload.find("sub") != payload.end() && payload.find("name") != payload.end()) {
            cout << "  Payload contains expected fields (sub, name)" << endl;
        }
    }

    // Invalid JWT should return empty map
    map<string, string> invalidPayload = decodeJWTPayload("invalid");
    if (!invalidPayload.empty()) {
        cerr << "decodeJWTPayload should return empty map for invalid JWT" << endl;
        passed = false;
    } else {
        cout << "  Invalid JWT returns empty map" << endl;
    }

    // Summary
    cout << endl << string(50, '=') << endl;
    if (passed) {
        cout << "All security tests passed!" << endl;
        return 0;
    } else {
        cout << "Some security tests failed." << endl;
        return 1;
    }
}
