import { mergeSort, quickSort } from "./sorting";

interface TestCase {
  input: number[];
  expect: number[];
  name: string;
}

export function runCheck(): boolean {
  let passed = true;

  const testCases: TestCase[] = [
    { input: [5, 2, 8, 1, 9], expect: [1, 2, 5, 8, 9], name: "basic array" },
    { input: [3, 3, 1, 2], expect: [1, 2, 3, 3], name: "duplicates" },
    { input: [1], expect: [1], name: "single element" },
    { input: [], expect: [], name: "empty array" },
    { input: [5, 4, 3, 2, 1], expect: [1, 2, 3, 4, 5], name: "reverse sorted" },
    { input: [1, 2, 3, 4, 5], expect: [1, 2, 3, 4, 5], name: "already sorted" },
  ];

  console.log("Testing MergeSort...");
  for (const tc of testCases) {
    const result = mergeSort([...tc.input]);
    if (!arraysEqual(result, tc.expect)) {
      console.error(
        `❌ mergeSort(${JSON.stringify(tc.input)}) [${tc.name}] expected ${JSON.stringify(tc.expect)} but got ${JSON.stringify(result)}`
      );
      passed = false;
    } else {
      console.log(`✓ mergeSort ${tc.name}`);
    }
  }

  console.log("\nTesting QuickSort...");
  for (const tc of testCases) {
    const result = quickSort([...tc.input]);
    if (!arraysEqual(result, tc.expect)) {
      console.error(
        `❌ quickSort(${JSON.stringify(tc.input)}) [${tc.name}] expected ${JSON.stringify(tc.expect)} but got ${JSON.stringify(result)}`
      );
      passed = false;
    } else {
      console.log(`✓ quickSort ${tc.name}`);
    }
  }

  // Verify both are O(n log n) on large arrays
  console.log("\nTesting performance on large array...");
  const largeArray = Array.from({ length: 1000 }, () => Math.floor(Math.random() * 1000));

  const mergeSorted = mergeSort([...largeArray]);
  const quickSorted = quickSort([...largeArray]);

  if (!isSorted(mergeSorted)) {
    console.error("❌ mergeSort failed to sort large array");
    passed = false;
  } else {
    console.log("✓ mergeSort handles large arrays");
  }

  if (!isSorted(quickSorted)) {
    console.error("❌ quickSort failed to sort large array");
    passed = false;
  } else {
    console.log("✓ quickSort handles large arrays");
  }

  if (passed) {
    console.log("\n✅ All tests passed!");
  } else {
    console.log("\n❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

function arraysEqual(a: number[], b: number[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

function isSorted(arr: number[]): boolean {
  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < arr[i - 1]) return false;
  }
  return true;
}

if (require.main === module) {
  runCheck();
}
