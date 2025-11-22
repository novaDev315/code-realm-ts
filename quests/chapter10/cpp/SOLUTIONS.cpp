// Reference solutions for Chapter 10: Citadel of Firewalls - Security
// Password hashing, JWT validation, and XSS prevention

#include <string>
#include <map>
#include <vector>
#include <sstream>
#include <iomanip>
#include <stdexcept>
#include <cstdint>

/**
 * Simple hash function for educational purposes
 * Uses FNV-1a hash algorithm - a non-cryptographic but deterministic hash
 *
 * Note: In production, use proper cryptographic libraries (OpenSSL, Crypto++, etc.)
 * SHA-256 or bcrypt/scrypt would be appropriate for real password hashing
 */
std::string hashPassword(const std::string& password) {
    // FNV-1a hash constants for 64-bit
    const uint64_t FNV_PRIME = 1099511628211ULL;
    const uint64_t FNV_OFFSET = 14695981039346656037ULL;

    uint64_t hash = FNV_OFFSET;

    // Process each byte
    for (char c : password) {
        hash ^= static_cast<uint64_t>(static_cast<unsigned char>(c));
        hash *= FNV_PRIME;
    }

    // Convert to hexadecimal string
    std::ostringstream oss;
    oss << std::hex << std::setfill('0') << std::setw(16) << hash;
    return oss.str();
}

/**
 * Verify a password against a stored hash - SOLUTION
 *
 * Hashes the provided password and compares it with the stored hash.
 *
 * @param password The plain text password to verify
 * @param hash The stored hash to compare against
 * @return true if the password matches the hash, false otherwise
 */
bool verifyPassword(const std::string& password, const std::string& hash) {
    // Hash the provided password
    std::string computedHash = hashPassword(password);

    // Compare hashes
    return computedHash == hash;
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
std::string sanitizeInput(const std::string& input) {
    std::string result;
    result.reserve(input.size() * 2); // Reserve space for potential escapes

    for (char c : input) {
        switch (c) {
            case '&':
                result += "&amp;";
                break;
            case '<':
                result += "&lt;";
                break;
            case '>':
                result += "&gt;";
                break;
            case '"':
                result += "&quot;";
                break;
            case '\'':
                result += "&#39;";
                break;
            default:
                result += c;
                break;
        }
    }

    return result;
}

/**
 * Helper function to split a string by delimiter
 */
static std::vector<std::string> splitString(const std::string& str, char delimiter) {
    std::vector<std::string> parts;
    std::stringstream ss(str);
    std::string part;

    while (std::getline(ss, part, delimiter)) {
        parts.push_back(part);
    }

    // Handle trailing delimiter case
    if (!str.empty() && str.back() == delimiter) {
        parts.push_back("");
    }

    return parts;
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
bool validateJWTStructure(const std::string& token) {
    if (token.empty()) {
        return false;
    }

    // Split by dot character
    std::vector<std::string> parts = splitString(token, '.');

    // JWT must have exactly 3 parts
    if (parts.size() != 3) {
        return false;
    }

    // Each part must be non-empty
    for (const auto& part : parts) {
        if (part.empty()) {
            return false;
        }
    }

    return true;
}

/**
 * Base64URL decode helper
 * Converts Base64URL to standard Base64, then decodes
 */
static std::string base64UrlDecode(const std::string& input) {
    // Convert Base64URL to standard Base64
    std::string base64 = input;
    for (char& c : base64) {
        if (c == '-') c = '+';
        else if (c == '_') c = '/';
    }

    // Add padding if needed
    while (base64.size() % 4 != 0) {
        base64 += '=';
    }

    // Base64 decoding table
    static const std::string base64Chars =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    std::string decoded;
    int val = 0;
    int bits = 0;

    for (char c : base64) {
        if (c == '=') break;

        size_t pos = base64Chars.find(c);
        if (pos == std::string::npos) {
            continue; // Skip invalid characters
        }

        val = (val << 6) | static_cast<int>(pos);
        bits += 6;

        if (bits >= 8) {
            bits -= 8;
            decoded += static_cast<char>((val >> bits) & 0xFF);
        }
    }

    return decoded;
}

/**
 * Simple JSON parser for extracting string values
 * For educational purposes - parses basic key-value pairs
 */
static std::map<std::string, std::string> parseSimpleJson(const std::string& json) {
    std::map<std::string, std::string> result;

    std::string trimmed = json;
    // Remove outer braces
    size_t start = trimmed.find('{');
    size_t end = trimmed.rfind('}');
    if (start == std::string::npos || end == std::string::npos || start >= end) {
        return result;
    }
    trimmed = trimmed.substr(start + 1, end - start - 1);

    // Parse key-value pairs
    size_t i = 0;
    while (i < trimmed.size()) {
        // Skip whitespace
        while (i < trimmed.size() && std::isspace(trimmed[i])) i++;
        if (i >= trimmed.size()) break;

        // Find key (expect quoted string)
        if (trimmed[i] != '"') break;
        size_t keyStart = ++i;
        while (i < trimmed.size() && trimmed[i] != '"') i++;
        std::string key = trimmed.substr(keyStart, i - keyStart);
        i++; // skip closing quote

        // Skip colon and whitespace
        while (i < trimmed.size() && (trimmed[i] == ':' || std::isspace(trimmed[i]))) i++;

        // Parse value
        std::string value;
        if (i < trimmed.size() && trimmed[i] == '"') {
            // String value
            size_t valueStart = ++i;
            while (i < trimmed.size() && trimmed[i] != '"') i++;
            value = trimmed.substr(valueStart, i - valueStart);
            i++; // skip closing quote
        } else {
            // Number or other value
            size_t valueStart = i;
            while (i < trimmed.size() && trimmed[i] != ',' && trimmed[i] != '}') i++;
            value = trimmed.substr(valueStart, i - valueStart);
            // Trim whitespace from value
            size_t valEnd = value.find_last_not_of(" \t\n\r");
            if (valEnd != std::string::npos) {
                value = value.substr(0, valEnd + 1);
            }
        }

        result[key] = value;

        // Skip comma and whitespace
        while (i < trimmed.size() && (trimmed[i] == ',' || std::isspace(trimmed[i]))) i++;
    }

    return result;
}

/**
 * Decode the payload from a JWT token - SOLUTION
 *
 * Extracts and decodes the Base64URL-encoded payload (middle part).
 * Parses the JSON into a map structure.
 *
 * @param token The JWT token string
 * @return A map containing the decoded payload claims (empty if invalid)
 */
std::map<std::string, std::string> decodeJWTPayload(const std::string& token) {
    // Validate structure first
    if (!validateJWTStructure(token)) {
        return {};
    }

    try {
        // Split and get the payload (middle part)
        std::vector<std::string> parts = splitString(token, '.');
        std::string payloadBase64 = parts[1];

        // Decode from Base64URL
        std::string payloadJson = base64UrlDecode(payloadBase64);

        // Parse JSON
        return parseSimpleJson(payloadJson);
    } catch (...) {
        // Return empty map for any decoding errors
        return {};
    }
}
