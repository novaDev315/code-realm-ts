// Test file for Chapter 11: Tower of Constructs - URL Shortener

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test base62Encode
        System.out.println("Testing base62Encode...");

        // Test case: 0
        String encoded0 = Shortener.base62Encode(0);
        if (!encoded0.equals("0")) {
            System.err.println("base62Encode(0): expected \"0\", got \"" + encoded0 + "\"");
            passed = false;
        } else {
            System.out.println("  base62Encode(0) = \"0\"");
        }

        // Test case: 1
        String encoded1 = Shortener.base62Encode(1);
        if (!encoded1.equals("1")) {
            System.err.println("base62Encode(1): expected \"1\", got \"" + encoded1 + "\"");
            passed = false;
        } else {
            System.out.println("  base62Encode(1) = \"1\"");
        }

        // Test case: 61 (should be 'z')
        String encoded61 = Shortener.base62Encode(61);
        if (!encoded61.equals("z")) {
            System.err.println("base62Encode(61): expected \"z\", got \"" + encoded61 + "\"");
            passed = false;
        } else {
            System.out.println("  base62Encode(61) = \"z\"");
        }

        // Test case: 62 (should be '10')
        String encoded62 = Shortener.base62Encode(62);
        if (!encoded62.equals("10")) {
            System.err.println("base62Encode(62): expected \"10\", got \"" + encoded62 + "\"");
            passed = false;
        } else {
            System.out.println("  base62Encode(62) = \"10\"");
        }

        // Test case: 3844 (62^2 = 3844, should be '100')
        String encoded3844 = Shortener.base62Encode(3844);
        if (!encoded3844.equals("100")) {
            System.err.println("base62Encode(3844): expected \"100\", got \"" + encoded3844 + "\"");
            passed = false;
        } else {
            System.out.println("  base62Encode(3844) = \"100\"");
        }

        // Test case: large number (1000000)
        String encodedLarge = Shortener.base62Encode(1000000);
        if (encodedLarge == null || encodedLarge.isEmpty()) {
            System.err.println("base62Encode(1000000): returned empty or null");
            passed = false;
        } else {
            System.out.println("  base62Encode(1000000) = \"" + encodedLarge + "\"");
        }

        // Test base62Decode
        System.out.println("\nTesting base62Decode...");

        // Test case: "0"
        long decoded0 = Shortener.base62Decode("0");
        if (decoded0 != 0) {
            System.err.println("base62Decode(\"0\"): expected 0, got " + decoded0);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"0\") = 0");
        }

        // Test case: "1"
        long decoded1 = Shortener.base62Decode("1");
        if (decoded1 != 1) {
            System.err.println("base62Decode(\"1\"): expected 1, got " + decoded1);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"1\") = 1");
        }

        // Test case: "z" (should be 61)
        long decodedZ = Shortener.base62Decode("z");
        if (decodedZ != 61) {
            System.err.println("base62Decode(\"z\"): expected 61, got " + decodedZ);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"z\") = 61");
        }

        // Test case: "10" (should be 62)
        long decoded10 = Shortener.base62Decode("10");
        if (decoded10 != 62) {
            System.err.println("base62Decode(\"10\"): expected 62, got " + decoded10);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"10\") = 62");
        }

        // Test case: "100" (should be 3844)
        long decoded100 = Shortener.base62Decode("100");
        if (decoded100 != 3844) {
            System.err.println("base62Decode(\"100\"): expected 3844, got " + decoded100);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"100\") = 3844");
        }

        // Round-trip tests
        System.out.println("\nTesting encode/decode round-trip...");
        long[] roundTripTests = {0, 1, 61, 62, 3844, 1000000, 123456789L, Long.MAX_VALUE / 1000};

        for (long num : roundTripTests) {
            String encoded = Shortener.base62Encode(num);
            long decoded = Shortener.base62Decode(encoded);
            if (decoded != num) {
                System.err.println("Round-trip failed for " + num + ": encode -> \"" + encoded + "\" -> decode -> " + decoded);
                passed = false;
            } else {
                System.out.println("  " + num + " -> \"" + encoded + "\" -> " + decoded);
            }
        }

        // Test specific character mappings
        System.out.println("\nTesting character mappings...");

        // 'A' should be index 10
        long decodedA = Shortener.base62Decode("A");
        if (decodedA != 10) {
            System.err.println("base62Decode(\"A\"): expected 10, got " + decodedA);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"A\") = 10");
        }

        // 'Z' should be index 35
        long decodedCapZ = Shortener.base62Decode("Z");
        if (decodedCapZ != 35) {
            System.err.println("base62Decode(\"Z\"): expected 35, got " + decodedCapZ);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"Z\") = 35");
        }

        // 'a' should be index 36
        long decodedLowA = Shortener.base62Decode("a");
        if (decodedLowA != 36) {
            System.err.println("base62Decode(\"a\"): expected 36, got " + decodedLowA);
            passed = false;
        } else {
            System.out.println("  base62Decode(\"a\") = 36");
        }

        if (passed) {
            System.out.println("\nAll tests passed!");
        } else {
            System.out.println("\nSome tests failed.");
            System.exit(1);
        }
    }
}
