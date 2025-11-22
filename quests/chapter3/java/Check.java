// Test file for Chapter 3: Mirror Maze

import java.util.Arrays;
import java.util.List;

public class Check {

    public static boolean runCheck() {
        boolean passed = true;

        // Test maxSumSubarray
        System.out.println("Testing maxSumSubarray...");
        int[][] maxSumArrays = {
            {2, 1, 5, 1, 3, 2},
            {2, 3, 4, 1, 5},
            {1, 4, 2, 10, 23, 3, 1, 0, 20}
        };
        int[] maxSumKs = {3, 2, 4};
        int[] maxSumExpected = {9, 7, 39};

        for (int i = 0; i < maxSumArrays.length; i++) {
            int result = Sliding.maxSumSubarray(maxSumArrays[i], maxSumKs[i]);
            if (result != maxSumExpected[i]) {
                System.err.println("❌ maxSumSubarray(" + Arrays.toString(maxSumArrays[i]) +
                    ", " + maxSumKs[i] + ") expected " + maxSumExpected[i] + " but got " + result);
                passed = false;
            } else {
                System.out.println("✓ maxSumSubarray k=" + maxSumKs[i]);
            }
        }

        // Test twoSum
        System.out.println("\nTesting twoSum...");
        int[][] twoSumArrays = {
            {1, 2, 3, 4, 5},
            {2, 7, 11, 15}
        };
        int[] twoSumTargets = {9, 9};
        int[] twoSumExpectSum = {9, 9};

        for (int i = 0; i < twoSumArrays.length; i++) {
            int[] result = Sliding.twoSum(twoSumArrays[i], twoSumTargets[i]);
            if (result == null) {
                System.err.println("❌ twoSum(" + Arrays.toString(twoSumArrays[i]) +
                    ", " + twoSumTargets[i] + ") returned null");
                passed = false;
            } else if (twoSumArrays[i][result[0]] + twoSumArrays[i][result[1]] != twoSumExpectSum[i]) {
                System.err.println("❌ twoSum(" + Arrays.toString(twoSumArrays[i]) +
                    ", " + twoSumTargets[i] + ") indices don't sum to " + twoSumExpectSum[i]);
                passed = false;
            } else {
                System.out.println("✓ twoSum target=" + twoSumTargets[i]);
            }
        }

        // Test threeSum
        System.out.println("\nTesting threeSum...");
        int[] threeSumArray = {-1, 0, 1, 2, -1, -4};
        List<List<Integer>> threeSumResult = Sliding.threeSum(threeSumArray, 0);
        if (threeSumResult.isEmpty()) {
            System.err.println("❌ threeSum should find at least one triplet");
            passed = false;
        } else {
            boolean validTriplets = true;
            for (List<Integer> triplet : threeSumResult) {
                if (triplet.get(0) + triplet.get(1) + triplet.get(2) != 0) {
                    validTriplets = false;
                    break;
                }
            }
            if (validTriplets) {
                System.out.println("✓ threeSum found " + threeSumResult.size() + " valid triplet(s)");
            } else {
                System.err.println("❌ threeSum returned invalid triplets");
                passed = false;
            }
        }

        if (passed) {
            System.out.println("\n✅ All tests passed!");
        } else {
            System.out.println("\n❌ Some tests failed.");
            System.exit(1);
        }

        return passed;
    }

    public static void main(String[] args) {
        runCheck();
    }
}
