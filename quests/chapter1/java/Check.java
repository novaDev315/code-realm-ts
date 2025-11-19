import java.util.*;

public class Check {
    private static int testCount = 0;
    private static int passCount = 0;

    public static void main(String[] args) {
        System.out.println("Testing Recursion implementations...\n");

        testFibonacci();
        testFactorial();
        testStringPermutations();

        System.out.println("\n" + passCount + "/" + testCount + " tests passed");

        if (passCount != testCount) {
            System.exit(1);
        }
    }

    private static void testFibonacci() {
        System.out.println("Testing fibonacci:");

        // Test fibonacci(0)
        testAssert("fibonacci(0)", Recursion.fibonacci(0), 0);

        // Test fibonacci(1)
        testAssert("fibonacci(1)", Recursion.fibonacci(1), 1);

        // Test fibonacci(5)
        testAssert("fibonacci(5)", Recursion.fibonacci(5), 5);

        // Test fibonacci(10)
        testAssert("fibonacci(10)", Recursion.fibonacci(10), 55);

        // Test fibonacci(15)
        testAssert("fibonacci(15)", Recursion.fibonacci(15), 610);
    }

    private static void testFactorial() {
        System.out.println("\nTesting factorial:");

        // Test factorial(0)
        testAssert("factorial(0)", Recursion.factorial(0), 1);

        // Test factorial(1)
        testAssert("factorial(1)", Recursion.factorial(1), 1);

        // Test factorial(5)
        testAssert("factorial(5)", Recursion.factorial(5), 120);

        // Test factorial(10)
        testAssert("factorial(10)", Recursion.factorial(10), 3628800);

        // Test factorial(12)
        testAssert("factorial(12)", Recursion.factorial(12), 479001600);
    }

    private static void testStringPermutations() {
        System.out.println("\nTesting stringPermutations:");

        // Test empty string
        List<String> result1 = Recursion.stringPermutations("");
        testAssert("permutations(\"\").size()", result1.size(), 1);
        testAssert("permutations(\"\").contains(\"\")", result1.contains(""), true);

        // Test single character
        List<String> result2 = Recursion.stringPermutations("a");
        testAssert("permutations(\"a\").size()", result2.size(), 1);
        testAssert("permutations(\"a\").contains(\"a\")", result2.contains("a"), true);

        // Test two characters
        List<String> result3 = Recursion.stringPermutations("ab");
        testAssert("permutations(\"ab\").size()", result3.size(), 2);
        testAssert("permutations(\"ab\").contains(\"ab\")", result3.contains("ab"), true);
        testAssert("permutations(\"ab\").contains(\"ba\")", result3.contains("ba"), true);

        // Test three characters
        List<String> result4 = Recursion.stringPermutations("abc");
        testAssert("permutations(\"abc\").size()", result4.size(), 6);
        testAssert("permutations(\"abc\").contains(\"abc\")", result4.contains("abc"), true);
        testAssert("permutations(\"abc\").contains(\"bac\")", result4.contains("bac"), true);
        testAssert("permutations(\"abc\").contains(\"cab\")", result4.contains("cab"), true);
    }

    private static void testAssert(String testName, int actual, int expected) {
        testCount++;
        if (actual == expected) {
            System.out.println("  ✓ " + testName + " = " + actual);
            passCount++;
        } else {
            System.out.println("  ✗ " + testName + ": expected " + expected + ", got " + actual);
        }
    }

    private static void testAssert(String testName, boolean actual, boolean expected) {
        testCount++;
        if (actual == expected) {
            System.out.println("  ✓ " + testName);
            passCount++;
        } else {
            System.out.println("  ✗ " + testName + ": expected " + expected + ", got " + actual);
        }
    }

    private static void testAssert(String testName, int actual, int expected, String extra) {
        testCount++;
        if (actual == expected) {
            System.out.println("  ✓ " + testName + " " + extra);
            passCount++;
        } else {
            System.out.println("  ✗ " + testName + ": expected " + expected + ", got " + actual);
        }
    }
}
