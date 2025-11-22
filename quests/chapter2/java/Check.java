import java.util.Arrays;
import java.util.Random;

/**
 * Test suite for Chapter 2 sorting algorithms
 */
public class Check {

    private static class TestCase {
        int[] input;
        int[] expected;
        String name;

        TestCase(int[] input, int[] expected, String name) {
            this.input = input;
            this.expected = expected;
            this.name = name;
        }
    }

    public static void main(String[] args) {
        boolean passed = true;

        TestCase[] testCases = {
            new TestCase(
                new int[]{5, 2, 8, 1, 9},
                new int[]{1, 2, 5, 8, 9},
                "basic array"
            ),
            new TestCase(
                new int[]{3, 3, 1, 2},
                new int[]{1, 2, 3, 3},
                "duplicates"
            ),
            new TestCase(
                new int[]{1},
                new int[]{1},
                "single element"
            ),
            new TestCase(
                new int[]{},
                new int[]{},
                "empty array"
            ),
            new TestCase(
                new int[]{5, 4, 3, 2, 1},
                new int[]{1, 2, 3, 4, 5},
                "reverse sorted"
            ),
            new TestCase(
                new int[]{1, 2, 3, 4, 5},
                new int[]{1, 2, 3, 4, 5},
                "already sorted"
            )
        };

        System.out.println("Testing MergeSort...");
        for (TestCase tc : testCases) {
            int[] result = Sorting.mergeSort(Arrays.copyOf(tc.input, tc.input.length));
            if (!arraysEqual(result, tc.expected)) {
                System.err.println(
                    "X mergeSort(" + Arrays.toString(tc.input) + ") [" + tc.name +
                    "] expected " + Arrays.toString(tc.expected) +
                    " but got " + Arrays.toString(result)
                );
                passed = false;
            } else {
                System.out.println("✓ mergeSort " + tc.name);
            }
        }

        System.out.println("\nTesting QuickSort...");
        for (TestCase tc : testCases) {
            int[] result = Sorting.quickSort(Arrays.copyOf(tc.input, tc.input.length));
            if (!arraysEqual(result, tc.expected)) {
                System.err.println(
                    "X quickSort(" + Arrays.toString(tc.input) + ") [" + tc.name +
                    "] expected " + Arrays.toString(tc.expected) +
                    " but got " + Arrays.toString(result)
                );
                passed = false;
            } else {
                System.out.println("✓ quickSort " + tc.name);
            }
        }

        // Test performance on large arrays
        System.out.println("\nTesting performance on large array...");
        Random rand = new Random(42);
        int[] largeArray = new int[1000];
        for (int i = 0; i < largeArray.length; i++) {
            largeArray[i] = rand.nextInt(1000);
        }

        int[] mergeSorted = Sorting.mergeSort(Arrays.copyOf(largeArray, largeArray.length));
        int[] quickSorted = Sorting.quickSort(Arrays.copyOf(largeArray, largeArray.length));

        if (!isSorted(mergeSorted)) {
            System.err.println("X mergeSort failed to sort large array");
            passed = false;
        } else {
            System.out.println("✓ mergeSort handles large arrays");
        }

        if (!isSorted(quickSorted)) {
            System.err.println("X quickSort failed to sort large array");
            passed = false;
        } else {
            System.out.println("✓ quickSort handles large arrays");
        }

        if (passed) {
            System.out.println("\n✅ All tests passed!");
        } else {
            System.out.println("\n❌ Some tests failed.");
            System.exit(1);
        }
    }

    private static boolean arraysEqual(int[] a, int[] b) {
        if (a.length != b.length) return false;
        for (int i = 0; i < a.length; i++) {
            if (a[i] != b[i]) return false;
        }
        return true;
    }

    private static boolean isSorted(int[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] < arr[i - 1]) return false;
        }
        return true;
    }
}
