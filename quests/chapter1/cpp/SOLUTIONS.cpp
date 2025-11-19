#include <iostream>
#include <vector>
#include <string>
#include <set>
#include <algorithm>

/**
 * Calculate fibonacci number at position n
 * Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21...
 * Each number is the sum of the two preceding ones
 *
 * @param n the position in the fibonacci sequence
 * @return the fibonacci number at position n
 */
int fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

/**
 * Calculate factorial of n
 * Factorial of n (n!) is the product of all positive integers less than or equal to n
 * 0! = 1, 1! = 1, 5! = 120
 *
 * @param n the number to calculate factorial for
 * @return the factorial of n
 */
int factorial(int n) {
    if (n <= 1) {
        return 1;
    }
    return n * factorial(n - 1);
}

/**
 * Helper function for generating permutations
 * Uses a recursive approach to generate all permutations
 *
 * @param str the input string
 * @param perms the set to store unique permutations
 */
void generatePermutations(std::string str, int l, int r, std::set<std::string>& perms) {
    if (l == r) {
        perms.insert(str);
    } else {
        for (int i = l; i <= r; i++) {
            std::swap(str[l], str[i]);
            generatePermutations(str, l + 1, r, perms);
            std::swap(str[l], str[i]);
        }
    }
}

/**
 * Generate all permutations of a string
 * A permutation is an arrangement of all characters in the string
 * For example, "abc" has 6 permutations: abc, acb, bac, bca, cab, cba
 * Uses std::set to ensure uniqueness in case of duplicate characters
 *
 * @param str the input string
 * @return vector of all unique permutations
 */
std::vector<std::string> stringPermutations(const std::string& str) {
    std::set<std::string> uniquePerms;

    if (str.empty()) {
        return std::vector<std::string>{""};
    }

    std::string temp = str;
    generatePermutations(temp, 0, temp.length() - 1, uniquePerms);

    return std::vector<std::string>(uniquePerms.begin(), uniquePerms.end());
}
