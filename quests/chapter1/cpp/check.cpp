#include <iostream>
#include <vector>
#include <string>
#include <algorithm>

// Forward declarations
int fibonacci(int n);
int factorial(int n);
std::vector<std::string> stringPermutations(const std::string& str);

int testCount = 0;
int passCount = 0;

void testIntResult(const std::string& testName, int actual, int expected) {
    testCount++;
    if (actual == expected) {
        std::cout << "  ✓ " << testName << " = " << actual << std::endl;
        passCount++;
    } else {
        std::cout << "  ✗ " << testName << ": expected " << expected << ", got " << actual << std::endl;
    }
}

void testBoolResult(const std::string& testName, bool actual, bool expected) {
    testCount++;
    if (actual == expected) {
        std::cout << "  ✓ " << testName << std::endl;
        passCount++;
    } else {
        std::cout << "  ✗ " << testName << ": expected " << (expected ? "true" : "false")
                  << ", got " << (actual ? "true" : "false") << std::endl;
    }
}

void testFibonacci() {
    std::cout << "Testing fibonacci:" << std::endl;

    testIntResult("fibonacci(0)", fibonacci(0), 0);
    testIntResult("fibonacci(1)", fibonacci(1), 1);
    testIntResult("fibonacci(5)", fibonacci(5), 5);
    testIntResult("fibonacci(10)", fibonacci(10), 55);
    testIntResult("fibonacci(15)", fibonacci(15), 610);
}

void testFactorial() {
    std::cout << "\nTesting factorial:" << std::endl;

    testIntResult("factorial(0)", factorial(0), 1);
    testIntResult("factorial(1)", factorial(1), 1);
    testIntResult("factorial(5)", factorial(5), 120);
    testIntResult("factorial(10)", factorial(10), 3628800);
    testIntResult("factorial(12)", factorial(12), 479001600);
}

void testStringPermutations() {
    std::cout << "\nTesting stringPermutations:" << std::endl;

    // Test empty string
    std::vector<std::string> result1 = stringPermutations("");
    testIntResult("permutations(\"\").size()", result1.size(), 1);
    testBoolResult("permutations(\"\").contains(\"\")",
                   std::find(result1.begin(), result1.end(), "") != result1.end(), true);

    // Test single character
    std::vector<std::string> result2 = stringPermutations("a");
    testIntResult("permutations(\"a\").size()", result2.size(), 1);
    testBoolResult("permutations(\"a\").contains(\"a\")",
                   std::find(result2.begin(), result2.end(), "a") != result2.end(), true);

    // Test two characters
    std::vector<std::string> result3 = stringPermutations("ab");
    testIntResult("permutations(\"ab\").size()", result3.size(), 2);
    testBoolResult("permutations(\"ab\").contains(\"ab\")",
                   std::find(result3.begin(), result3.end(), "ab") != result3.end(), true);
    testBoolResult("permutations(\"ab\").contains(\"ba\")",
                   std::find(result3.begin(), result3.end(), "ba") != result3.end(), true);

    // Test three characters
    std::vector<std::string> result4 = stringPermutations("abc");
    testIntResult("permutations(\"abc\").size()", result4.size(), 6);
    testBoolResult("permutations(\"abc\").contains(\"abc\")",
                   std::find(result4.begin(), result4.end(), "abc") != result4.end(), true);
    testBoolResult("permutations(\"abc\").contains(\"bac\")",
                   std::find(result4.begin(), result4.end(), "bac") != result4.end(), true);
    testBoolResult("permutations(\"abc\").contains(\"cab\")",
                   std::find(result4.begin(), result4.end(), "cab") != result4.end(), true);
}

int main() {
    std::cout << "Testing Recursion implementations...\n" << std::endl;

    testFibonacci();
    testFactorial();
    testStringPermutations();

    std::cout << "\n" << passCount << "/" << testCount << " tests passed" << std::endl;

    if (passCount != testCount) {
        return 1;
    }

    return 0;
}
