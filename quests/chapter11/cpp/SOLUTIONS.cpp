// Reference solutions for Chapter 11: Tower of Constructs - URL Shortener
// Base62 encoding and decoding for URL shortening

#include <string>
#include <stdexcept>

// Base62 character set: 0-9, A-Z, a-z
const std::string BASE62_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/**
 * Encode a numeric ID to a Base62 string - SOLUTION
 *
 * Base62 encoding converts a number to a string using 62 different characters.
 * This is essentially a base conversion from decimal (base 10) to base 62.
 *
 * Algorithm:
 * 1. Handle the special case of 0
 * 2. Repeatedly divide by 62, using the remainder to select a character
 * 3. Build the string from right to left (least significant digit first)
 *
 * @param id The numeric ID to encode (non-negative)
 * @return The Base62 encoded string
 */
std::string base62Encode(long long id) {
    // Special case: 0 encodes to "0"
    if (id == 0) {
        return "0";
    }

    std::string result = "";

    while (id > 0) {
        // Get the index for the current digit (remainder when divided by 62)
        int remainder = static_cast<int>(id % 62);
        // Prepend the character (building from right to left)
        result = BASE62_CHARS[remainder] + result;
        // Move to the next digit
        id = id / 62;
    }

    return result;
}

/**
 * Decode a Base62 string back to a numeric ID - SOLUTION
 *
 * This is the inverse of encoding - converts from base 62 back to decimal.
 *
 * Algorithm:
 * 1. Start with result = 0
 * 2. For each character from left to right:
 *    - Multiply result by 62 (shift left in base 62)
 *    - Add the value of the current character
 *
 * @param shortCode The Base62 encoded string
 * @return The decoded numeric ID
 * @throws std::invalid_argument if the string contains invalid characters
 */
long long base62Decode(const std::string& shortCode) {
    long long result = 0;

    for (size_t i = 0; i < shortCode.length(); i++) {
        char c = shortCode[i];
        size_t value = BASE62_CHARS.find(c);

        if (value == std::string::npos) {
            throw std::invalid_argument(std::string("Invalid Base62 character: ") + c);
        }

        // Shift left by one base-62 digit and add the new value
        result = result * 62 + static_cast<long long>(value);
    }

    return result;
}
