// Chapter 11: Tower of Constructs - URL Shortener
// Your task: Implement Base62 encoding and decoding for URL shortening

#include <string>
#include <stdexcept>

// Base62 character set: 0-9, A-Z, a-z
const std::string BASE62_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/**
 * Encode a numeric ID to a Base62 string
 *
 * Base62 encoding uses 62 characters (0-9, A-Z, a-z) to represent numbers.
 * This is commonly used in URL shorteners because:
 * - It's URL-safe (no special characters)
 * - It's compact (62^6 = 56+ billion unique codes with just 6 characters)
 * - It's human-readable
 *
 * TODO: Implement Base62 encoding
 * - Handle the special case when id is 0
 * - Use modulo 62 to get each digit
 * - Use integer division to move to the next digit
 * - Build the result string from least significant to most significant digit
 *
 * Example:
 *   0 -> "0"
 *   1 -> "1"
 *   62 -> "10"
 *   3844 -> "100"
 *
 * @param id The numeric ID to encode (non-negative)
 * @return The Base62 encoded string
 */
std::string base62Encode(long long id) {
    // TODO: Implement Base62 encoding
    // Hint: Similar to converting decimal to any other base
    // Build the result by prepending characters
    throw std::runtime_error("Not implemented yet");
}

/**
 * Decode a Base62 string back to a numeric ID
 *
 * This reverses the encoding process to retrieve the original number.
 *
 * TODO: Implement Base62 decoding
 * - Iterate through each character in the string
 * - Find the index of each character in BASE62_CHARS
 * - Multiply accumulated result by 62 and add the character value
 *
 * Example:
 *   "0" -> 0
 *   "1" -> 1
 *   "10" -> 62
 *   "100" -> 3844
 *
 * @param shortCode The Base62 encoded string
 * @return The decoded numeric ID
 * @throws std::invalid_argument if the string contains invalid characters
 */
long long base62Decode(const std::string& shortCode) {
    // TODO: Implement Base62 decoding
    // Hint: Process each character from left to right
    // result = result * 62 + charValue
    throw std::runtime_error("Not implemented yet");
}
