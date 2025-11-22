// Chapter 11: Tower of Constructs - URL Shortener
// Design a scalable URL shortening system with base62 encoding

package main

// Base62Chars contains the character set for base62 encoding
// 0-9 (10) + A-Z (26) + a-z (26) = 62 characters
const Base62Chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"

// Base62Encode converts a number to a base62 string
// Used for generating short codes from database IDs
// Example: 0 -> "0", 62 -> "10", 3844 -> "100"
func Base62Encode(id int64) string {
	// TODO: Convert number to base62 string
	// - Handle edge case where id is 0 (return "0")
	// - Build result by repeatedly dividing by 62
	// - Use Base62Chars to map remainders to characters
	// - Remember to reverse or prepend characters
	return ""
}

// Base62Decode converts a base62 string back to a number
// Used for looking up the original ID from a short code
// Example: "0" -> 0, "10" -> 62, "100" -> 3844
func Base62Decode(shortCode string) int64 {
	// TODO: Convert base62 string to number
	// - Process each character from left to right
	// - Find the index of each character in Base62Chars
	// - Multiply running total by 62 and add the index
	return 0
}
