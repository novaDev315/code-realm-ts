// Reference solutions for Chapter 10 - Security (Hashing, JWT, XSS Prevention)

package main

import (
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"regexp"
	"strings"
)

// HashPassword creates a simple hash of password + salt using SHA256
// Note: This is for educational purposes only - use bcrypt in production!
func HashPassword(password, salt string) string {
	// Combine password and salt
	combined := password + salt

	// Create SHA256 hash
	hasher := sha256.New()
	hasher.Write([]byte(combined))
	hashBytes := hasher.Sum(nil)

	// Return hex-encoded hash
	return hex.EncodeToString(hashBytes)
}

// VerifyPassword checks if a password matches a hash
func VerifyPassword(password, salt, hash string) bool {
	// Hash the password with salt
	computed := HashPassword(password, salt)

	// Compare with provided hash (constant-time comparison would be better in production)
	return computed == hash
}

// SanitizeInput removes/escapes dangerous characters to prevent XSS attacks
func SanitizeInput(input string) string {
	result := input

	// Remove script tags and their content (case-insensitive)
	// Go's regexp doesn't support lookahead, so we use a simple approach
	scriptRegex := regexp.MustCompile(`(?i)<script[^>]*>[\s\S]*?</script>`)
	result = scriptRegex.ReplaceAllString(result, "")

	// Remove other HTML tags
	tagRegex := regexp.MustCompile(`<[^>]*>`)
	result = tagRegex.ReplaceAllString(result, "")

	// Escape dangerous characters
	result = strings.ReplaceAll(result, `"`, "&quot;")
	result = strings.ReplaceAll(result, "'", "&#39;")

	return result
}

// ValidateJWTStructure checks if a JWT token has valid structure
// JWT format: header.payload.signature (3 base64-encoded parts separated by dots)
func ValidateJWTStructure(token string) bool {
	// Handle empty token
	if token == "" {
		return false
	}

	// Split by dots
	parts := strings.Split(token, ".")

	// Must have exactly 3 parts
	if len(parts) != 3 {
		return false
	}

	// Each part must be non-empty
	for _, part := range parts {
		if part == "" {
			return false
		}
	}

	return true
}

// DecodeJWTPayload extracts and decodes the payload from a JWT token
// Note: This does NOT verify the signature - for educational purposes only!
func DecodeJWTPayload(token string) (map[string]interface{}, error) {
	// Validate structure first
	if !ValidateJWTStructure(token) {
		return nil, errors.New("invalid JWT structure")
	}

	// Split token and get the payload (second part)
	parts := strings.Split(token, ".")
	payloadB64 := parts[1]

	// Add padding if needed (base64 URL encoding may strip padding)
	switch len(payloadB64) % 4 {
	case 2:
		payloadB64 += "=="
	case 3:
		payloadB64 += "="
	}

	// Base64 URL decode the payload
	payloadBytes, err := base64.URLEncoding.DecodeString(payloadB64)
	if err != nil {
		// Try standard base64 as fallback
		payloadBytes, err = base64.StdEncoding.DecodeString(payloadB64)
		if err != nil {
			return nil, errors.New("failed to decode payload: " + err.Error())
		}
	}

	// JSON unmarshal into map
	var payload map[string]interface{}
	if err := json.Unmarshal(payloadBytes, &payload); err != nil {
		return nil, errors.New("failed to parse payload JSON: " + err.Error())
	}

	return payload, nil
}
