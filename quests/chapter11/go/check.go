// Test runner for Chapter 11 - URL Shortener Base62 Encoding

package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	passed := true

	// Test Base62Encode
	fmt.Println("Testing Base62Encode...")
	encodeCases := []struct {
		input  int64
		expect string
		name   string
	}{
		{0, "0", "Zero"},
		{1, "1", "One"},
		{9, "9", "Single digit (9)"},
		{10, "A", "Ten (should be 'A')"},
		{35, "Z", "35 (uppercase Z)"},
		{36, "a", "36 (lowercase a)"},
		{61, "z", "61 (lowercase z)"},
		{62, "10", "62 (should be '10')"},
		{3844, "100", "3844 (62^2)"},
		{238327, "zzz", "238327 (61*62^2 + 61*62 + 61)"},
		{238328, "1000", "238328 (62^3)"},
		{12345678, "pnfq", "Large number"},
	}

	for _, tc := range encodeCases {
		result := Base62Encode(tc.input)
		if result != tc.expect {
			fmt.Printf("  X Base62Encode(%d) - %s: expected \"%s\" but got \"%s\"\n", tc.input, tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("  OK Base62Encode(%d) = \"%s\" (%s)\n", tc.input, result, tc.name)
		}
	}

	// Test Base62Decode
	fmt.Println("\nTesting Base62Decode...")
	decodeCases := []struct {
		input  string
		expect int64
		name   string
	}{
		{"0", 0, "Zero"},
		{"1", 1, "One"},
		{"9", 9, "Single digit (9)"},
		{"A", 10, "Letter 'A' (uppercase)"},
		{"Z", 35, "Letter 'Z' (uppercase)"},
		{"a", 36, "Letter 'a' (lowercase)"},
		{"z", 61, "Letter 'z' (lowercase)"},
		{"10", 62, "Two chars '10'"},
		{"100", 3844, "62^2"},
		{"zzz", 238327, "61*62^2 + 61*62 + 61"},
		{"1000", 238328, "62^3"},
		{"pnfq", 12345678, "Large number"},
	}

	for _, tc := range decodeCases {
		result := Base62Decode(tc.input)
		if result != tc.expect {
			fmt.Printf("  X Base62Decode(\"%s\") - %s: expected %d but got %d\n", tc.input, tc.name, tc.expect, result)
			passed = false
		} else {
			fmt.Printf("  OK Base62Decode(\"%s\") = %d (%s)\n", tc.input, result, tc.name)
		}
	}

	// Test round-trip encoding/decoding
	fmt.Println("\nTesting encode/decode round-trip...")
	roundTripCases := []int64{0, 1, 10, 35, 36, 61, 62, 100, 1000, 3844, 10000, 238327, 238328, 12345678, 999999999}

	for _, num := range roundTripCases {
		encoded := Base62Encode(num)
		decoded := Base62Decode(encoded)
		if decoded != num {
			fmt.Printf("  X Round-trip failed for %d: encode -> \"%s\" -> decode -> %d\n", num, encoded, decoded)
			passed = false
		} else {
			fmt.Printf("  OK %d -> \"%s\" -> %d\n", num, encoded, decoded)
		}
	}

	// Test character set validity
	fmt.Println("\nTesting character set validity...")
	testNums := []int64{123, 456789, 9876543210}
	for _, num := range testNums {
		encoded := Base62Encode(num)
		valid := true
		for _, c := range encoded {
			if !strings.ContainsRune(Base62Chars, c) {
				valid = false
				break
			}
		}
		if !valid {
			fmt.Printf("  X Base62Encode(%d) = \"%s\" contains invalid characters\n", num, encoded)
			passed = false
		} else {
			fmt.Printf("  OK Base62Encode(%d) = \"%s\" uses valid characters\n", num, encoded)
		}
	}

	if passed {
		fmt.Println("\nAll tests passed!")
	} else {
		fmt.Println("\nSome tests failed.")
		os.Exit(1)
	}
}
