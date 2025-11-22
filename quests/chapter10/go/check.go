// Test runner for Chapter 10 - Security (Hashing, JWT, XSS Prevention)

package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	passed := true

	// Test HashPassword
	fmt.Println("Testing HashPassword...")
	hashCases := []struct {
		password string
		salt     string
		name     string
	}{
		{"password123", "randomsalt", "Basic password"},
		{"", "salt", "Empty password"},
		{"pass", "", "Empty salt"},
		{"MySecureP@ssw0rd!", "pepper123", "Complex password"},
	}

	for _, tc := range hashCases {
		result := HashPassword(tc.password, tc.salt)
		if result == "" {
			fmt.Printf("  X HashPassword(%q, %q) - %s: returned empty string\n", tc.password, tc.salt, tc.name)
			passed = false
		} else if len(result) != 64 { // SHA256 hex is 64 chars
			fmt.Printf("  X HashPassword(%q, %q) - %s: expected 64 char hex, got %d chars\n", tc.password, tc.salt, tc.name, len(result))
			passed = false
		} else {
			fmt.Printf("  OK HashPassword(%q, %q) = %s... (%s)\n", tc.password, tc.salt, result[:16], tc.name)
		}
	}

	// Test hash consistency
	fmt.Println("\nTesting hash consistency...")
	hash1 := HashPassword("test", "salt")
	hash2 := HashPassword("test", "salt")
	if hash1 != hash2 {
		fmt.Printf("  X Same inputs should produce same hash\n")
		passed = false
	} else if hash1 != "" {
		fmt.Printf("  OK Same inputs produce consistent hash\n")
	}

	// Different inputs should produce different hashes
	hash3 := HashPassword("test", "salt1")
	hash4 := HashPassword("test", "salt2")
	if hash3 == hash4 && hash3 != "" {
		fmt.Printf("  X Different salts should produce different hashes\n")
		passed = false
	} else if hash3 != "" && hash4 != "" {
		fmt.Printf("  OK Different salts produce different hashes\n")
	}

	// Test VerifyPassword
	fmt.Println("\nTesting VerifyPassword...")
	verifyCases := []struct {
		password string
		salt     string
		correct  bool
		name     string
	}{
		{"password123", "salt", true, "Correct password"},
		{"wrongpassword", "salt", false, "Wrong password"},
		{"password123", "wrongsalt", false, "Wrong salt"},
	}

	for _, tc := range verifyCases {
		correctHash := HashPassword("password123", "salt")
		result := VerifyPassword(tc.password, tc.salt, correctHash)
		if result != tc.correct {
			fmt.Printf("  X VerifyPassword(%q, %q, hash) - %s: expected %v but got %v\n", tc.password, tc.salt, tc.name, tc.correct, result)
			passed = false
		} else {
			fmt.Printf("  OK VerifyPassword(%q, %q, hash) = %v (%s)\n", tc.password, tc.salt, result, tc.name)
		}
	}

	// Test SanitizeInput
	fmt.Println("\nTesting SanitizeInput...")
	sanitizeCases := []struct {
		input    string
		contains []string
		excludes []string
		name     string
	}{
		{"<script>alert('xss')</script>", []string{}, []string{"<script>", "</script>", "alert"}, "Script tag removal"},
		{"<b>Bold</b>", []string{"Bold"}, []string{"<b>", "</b>"}, "HTML tag removal"},
		{"Hello <World>", []string{"Hello"}, []string{"<", ">"}, "Angle bracket removal"},
		{`Say "Hello"`, []string{"&quot;"}, []string{`"`}, "Quote escaping"},
		{"It's fine", []string{"&#39;"}, []string{"'"}, "Apostrophe escaping"},
		{"Normal text", []string{"Normal text"}, []string{}, "Plain text unchanged"},
	}

	for _, tc := range sanitizeCases {
		result := SanitizeInput(tc.input)
		success := true

		for _, should := range tc.contains {
			if !strings.Contains(result, should) {
				fmt.Printf("  X SanitizeInput(%q) - %s: should contain %q but got %q\n", tc.input, tc.name, should, result)
				success = false
				passed = false
				break
			}
		}

		if success {
			for _, shouldNot := range tc.excludes {
				if strings.Contains(result, shouldNot) {
					fmt.Printf("  X SanitizeInput(%q) - %s: should NOT contain %q but got %q\n", tc.input, tc.name, shouldNot, result)
					success = false
					passed = false
					break
				}
			}
		}

		if success {
			fmt.Printf("  OK SanitizeInput(%q) = %q (%s)\n", tc.input, result, tc.name)
		}
	}

	// Test ValidateJWTStructure
	fmt.Println("\nTesting ValidateJWTStructure...")
	// Example valid JWT (structure only, not cryptographically valid)
	validJWT := "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"

	jwtStructureCases := []struct {
		token  string
		valid  bool
		name   string
	}{
		{validJWT, true, "Valid JWT structure"},
		{"header.payload.signature", true, "Simple 3-part structure"},
		{"only.two", false, "Only 2 parts"},
		{"one", false, "Only 1 part"},
		{"", false, "Empty string"},
		{"a.b.c.d", false, "4 parts (too many)"},
		{".payload.signature", false, "Empty header"},
		{"header..signature", false, "Empty payload"},
		{"header.payload.", false, "Empty signature"},
	}

	for _, tc := range jwtStructureCases {
		result := ValidateJWTStructure(tc.token)
		if result != tc.valid {
			fmt.Printf("  X ValidateJWTStructure(%q) - %s: expected %v but got %v\n", tc.token[:min(len(tc.token), 30)], tc.name, tc.valid, result)
			passed = false
		} else {
			display := tc.token
			if len(display) > 30 {
				display = display[:30] + "..."
			}
			fmt.Printf("  OK ValidateJWTStructure(%q) = %v (%s)\n", display, result, tc.name)
		}
	}

	// Test DecodeJWTPayload
	fmt.Println("\nTesting DecodeJWTPayload...")
	payload, err := DecodeJWTPayload(validJWT)
	if err != nil {
		fmt.Printf("  X DecodeJWTPayload(validJWT) returned error: %v\n", err)
		passed = false
	} else if payload == nil {
		fmt.Printf("  X DecodeJWTPayload(validJWT) returned nil payload\n")
		passed = false
	} else {
		// Check for expected fields
		if sub, ok := payload["sub"].(string); !ok || sub != "1234567890" {
			fmt.Printf("  X DecodeJWTPayload: expected sub=1234567890, got %v\n", payload["sub"])
			passed = false
		} else {
			fmt.Printf("  OK DecodeJWTPayload extracted sub=%s\n", sub)
		}

		if name, ok := payload["name"].(string); !ok || name != "John Doe" {
			fmt.Printf("  X DecodeJWTPayload: expected name='John Doe', got %v\n", payload["name"])
			passed = false
		} else {
			fmt.Printf("  OK DecodeJWTPayload extracted name=%s\n", name)
		}
	}

	// Test invalid JWT decoding
	_, err = DecodeJWTPayload("invalid.jwt")
	if err == nil {
		// It's acceptable to return nil, nil for invalid JWTs
		// but if we got here, the validation might be too lenient
		fmt.Printf("  ? DecodeJWTPayload(invalid) returned no error (check structure validation)\n")
	} else {
		fmt.Printf("  OK DecodeJWTPayload(invalid) correctly returned error\n")
	}

	if passed {
		fmt.Println("\nAll tests passed!")
	} else {
		fmt.Println("\nSome tests failed.")
		os.Exit(1)
	}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
