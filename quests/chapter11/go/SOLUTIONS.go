// Reference solutions for Chapter 11 - URL Shortener Base62 Encoding

package main

import "strings"

// Base62Chars contains the character set for base62 encoding
// 0-9 (10) + A-Z (26) + a-z (26) = 62 characters
const Base62Chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"

// Base62Encode converts a number to a base62 string
// Used for generating short codes from database IDs
func Base62Encode(id int64) string {
	// Handle edge case for zero
	if id == 0 {
		return "0"
	}

	var result strings.Builder
	base := int64(62)

	// Build the base62 string by repeatedly dividing by 62
	for id > 0 {
		remainder := id % base
		result.WriteByte(Base62Chars[remainder])
		id = id / base
	}

	// Reverse the string since we built it backwards
	encoded := result.String()
	runes := []rune(encoded)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}

	return string(runes)
}

// Base62Decode converts a base62 string back to a number
// Used for looking up the original ID from a short code
func Base62Decode(shortCode string) int64 {
	var result int64 = 0
	base := int64(62)

	// Process each character from left to right
	for _, char := range shortCode {
		// Find the index of this character in our character set
		index := int64(strings.IndexRune(Base62Chars, char))
		if index == -1 {
			// Invalid character, return 0 (could also panic or return error)
			return 0
		}
		// Multiply by base and add the new digit
		result = result*base + index
	}

	return result
}
