#include <iostream>
#include <vector>
#include <random>
#include <string>
#include "SOLUTIONS.cpp"

/**
 * Test suite for Chapter 2 sorting algorithms
 */

struct TestCase {
    std::vector<int> input;
    std::vector<int> expected;
    std::string name;
};

bool vectorsEqual(const std::vector<int>& a, const std::vector<int>& b) {
    if (a.size() != b.size()) return false;
    for (size_t i = 0; i < a.size(); i++) {
        if (a[i] != b[i]) return false;
    }
    return true;
}

// isSorted is defined in SOLUTIONS.cpp

std::string vectorToString(const std::vector<int>& vec) {
    std::string result = "[";
    for (size_t i = 0; i < vec.size(); i++) {
        result += std::to_string(vec[i]);
        if (i < vec.size() - 1) result += ", ";
    }
    result += "]";
    return result;
}

int main() {
    bool passed = true;

    std::vector<TestCase> testCases = {
        {{5, 2, 8, 1, 9}, {1, 2, 5, 8, 9}, "basic array"},
        {{3, 3, 1, 2}, {1, 2, 3, 3}, "duplicates"},
        {{1}, {1}, "single element"},
        {{}, {}, "empty array"},
        {{5, 4, 3, 2, 1}, {1, 2, 3, 4, 5}, "reverse sorted"},
        {{1, 2, 3, 4, 5}, {1, 2, 3, 4, 5}, "already sorted"}
    };

    std::cout << "Testing MergeSort..." << std::endl;
    for (const auto& tc : testCases) {
        std::vector<int> result = mergeSort(tc.input);
        if (!vectorsEqual(result, tc.expected)) {
            std::cerr << "X mergeSort(" << vectorToString(tc.input) << ") ["
                      << tc.name << "] expected " << vectorToString(tc.expected)
                      << " but got " << vectorToString(result) << std::endl;
            passed = false;
        } else {
            std::cout << "✓ mergeSort " << tc.name << std::endl;
        }
    }

    std::cout << "\nTesting QuickSort..." << std::endl;
    for (const auto& tc : testCases) {
        std::vector<int> result = quickSort(tc.input);
        if (!vectorsEqual(result, tc.expected)) {
            std::cerr << "X quickSort(" << vectorToString(tc.input) << ") ["
                      << tc.name << "] expected " << vectorToString(tc.expected)
                      << " but got " << vectorToString(result) << std::endl;
            passed = false;
        } else {
            std::cout << "✓ quickSort " << tc.name << std::endl;
        }
    }

    // Test performance on large arrays
    std::cout << "\nTesting performance on large array..." << std::endl;
    std::mt19937 gen(42);
    std::uniform_int_distribution<> dis(0, 999);
    std::vector<int> largeArray(1000);
    for (int& val : largeArray) {
        val = dis(gen);
    }

    std::vector<int> mergeSorted = mergeSort(largeArray);
    std::vector<int> quickSorted = quickSort(largeArray);

    if (!isSorted(mergeSorted)) {
        std::cerr << "X mergeSort failed to sort large array" << std::endl;
        passed = false;
    } else {
        std::cout << "✓ mergeSort handles large arrays" << std::endl;
    }

    if (!isSorted(quickSorted)) {
        std::cerr << "X quickSort failed to sort large array" << std::endl;
        passed = false;
    } else {
        std::cout << "✓ quickSort handles large arrays" << std::endl;
    }

    if (passed) {
        std::cout << "\n✅ All tests passed!" << std::endl;
    } else {
        std::cout << "\n❌ Some tests failed." << std::endl;
        return 1;
    }

    return 0;
}
