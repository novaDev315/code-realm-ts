// Test file for Chapter 3: Mirror Maze

#include <iostream>
#include <vector>
#include <string>
#include "sliding.cpp"

void printVector(const std::vector<int>& vec) {
    std::cout << "[";
    for (size_t i = 0; i < vec.size(); i++) {
        std::cout << vec[i];
        if (i < vec.size() - 1) std::cout << ", ";
    }
    std::cout << "]";
}

bool runCheck() {
    bool passed = true;

    // Test maxSumSubarray
    std::cout << "Testing maxSumSubarray..." << std::endl;

    std::vector<std::vector<int>> maxSumArrays = {
        {2, 1, 5, 1, 3, 2},
        {2, 3, 4, 1, 5},
        {1, 4, 2, 10, 23, 3, 1, 0, 20}
    };
    std::vector<int> maxSumKs = {3, 2, 4};
    std::vector<int> maxSumExpected = {9, 7, 39};

    for (size_t i = 0; i < maxSumArrays.size(); i++) {
        int result = maxSumSubarray(maxSumArrays[i], maxSumKs[i]);
        if (result != maxSumExpected[i]) {
            std::cerr << "❌ maxSumSubarray(";
            printVector(maxSumArrays[i]);
            std::cerr << ", " << maxSumKs[i] << ") expected " << maxSumExpected[i]
                     << " but got " << result << std::endl;
            passed = false;
        } else {
            std::cout << "✓ maxSumSubarray k=" << maxSumKs[i] << std::endl;
        }
    }

    // Test twoSum
    std::cout << "\nTesting twoSum..." << std::endl;

    std::vector<std::vector<int>> twoSumArrays = {
        {1, 2, 3, 4, 5},
        {2, 7, 11, 15}
    };
    std::vector<int> twoSumTargets = {9, 9};
    std::vector<int> twoSumExpectSum = {9, 9};

    for (size_t i = 0; i < twoSumArrays.size(); i++) {
        std::vector<int> result = twoSum(twoSumArrays[i], twoSumTargets[i]);
        if (result.empty()) {
            std::cerr << "❌ twoSum(";
            printVector(twoSumArrays[i]);
            std::cerr << ", " << twoSumTargets[i] << ") returned empty vector" << std::endl;
            passed = false;
        } else if (twoSumArrays[i][result[0]] + twoSumArrays[i][result[1]] != twoSumExpectSum[i]) {
            std::cerr << "❌ twoSum(";
            printVector(twoSumArrays[i]);
            std::cerr << ", " << twoSumTargets[i] << ") indices don't sum to "
                     << twoSumExpectSum[i] << std::endl;
            passed = false;
        } else {
            std::cout << "✓ twoSum target=" << twoSumTargets[i] << std::endl;
        }
    }

    // Test threeSum
    std::cout << "\nTesting threeSum..." << std::endl;
    std::vector<int> threeSumArray = {-1, 0, 1, 2, -1, -4};
    std::vector<std::vector<int>> threeSumResult = threeSum(threeSumArray, 0);

    if (threeSumResult.empty()) {
        std::cerr << "❌ threeSum should find at least one triplet" << std::endl;
        passed = false;
    } else {
        bool validTriplets = true;
        for (const auto& triplet : threeSumResult) {
            if (triplet[0] + triplet[1] + triplet[2] != 0) {
                validTriplets = false;
                break;
            }
        }
        if (validTriplets) {
            std::cout << "✓ threeSum found " << threeSumResult.size()
                     << " valid triplet(s)" << std::endl;
        } else {
            std::cerr << "❌ threeSum returned invalid triplets" << std::endl;
            passed = false;
        }
    }

    if (passed) {
        std::cout << "\n✅ All tests passed!" << std::endl;
    } else {
        std::cout << "\n❌ Some tests failed." << std::endl;
        return false;
    }

    return passed;
}

int main() {
    bool result = runCheck();
    return result ? 0 : 1;
}
