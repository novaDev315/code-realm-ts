// Reference solutions for Chapter 11: Tower of Constructs - URL Shortener
// Base62 encoding and decoding for URL shortening

public class SOLUTIONS {
    // Base62 character set: 0-9, A-Z, a-z
    static final String BASE62_CHARS = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

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
    public static String base62Encode(long id) {
        // Special case: 0 encodes to "0"
        if (id == 0) {
            return "0";
        }

        StringBuilder result = new StringBuilder();

        while (id > 0) {
            // Get the index for the current digit (remainder when divided by 62)
            int remainder = (int) (id % 62);
            // Prepend the character (building from right to left)
            result.insert(0, BASE62_CHARS.charAt(remainder));
            // Move to the next digit
            id = id / 62;
        }

        return result.toString();
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
     * @throws IllegalArgumentException if the string contains invalid characters
     */
    public static long base62Decode(String shortCode) {
        long result = 0;

        for (int i = 0; i < shortCode.length(); i++) {
            char c = shortCode.charAt(i);
            int value = BASE62_CHARS.indexOf(c);

            if (value == -1) {
                throw new IllegalArgumentException("Invalid Base62 character: " + c);
            }

            // Shift left by one base-62 digit and add the new value
            result = result * 62 + value;
        }

        return result;
    }
}
