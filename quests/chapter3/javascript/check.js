const { maxSumSubarray, twoSum, threeSum } = require("./sliding");

function runCheck() {
  let passed = true;

  console.log("Testing maxSumSubarray...");
  const maxSumCases = [
    { arr: [2, 1, 5, 1, 3, 2], k: 3, expect: 9 },
    { arr: [2, 3, 4, 1, 5], k: 2, expect: 7 },
    { arr: [1, 4, 2, 10, 23, 3, 1, 0, 20], k: 4, expect: 39 },
  ];

  for (const tc of maxSumCases) {
    const result = maxSumSubarray(tc.arr, tc.k);
    if (result !== tc.expect) {
      console.error(`❌ maxSumSubarray(${JSON.stringify(tc.arr)}, ${tc.k}) expected ${tc.expect} but got ${result}`);
      passed = false;
    } else {
      console.log(`✓ maxSumSubarray k=${tc.k}`);
    }
  }

  console.log("\nTesting twoSum...");
  const twoSumCases = [
    { arr: [1, 2, 3, 4, 5], target: 9, expectSum: 9 },
    { arr: [2, 7, 11, 15], target: 9, expectSum: 9 },
  ];

  for (const tc of twoSumCases) {
    const result = twoSum(tc.arr, tc.target);
    if (result === null) {
      console.error(`❌ twoSum(${JSON.stringify(tc.arr)}, ${tc.target}) returned null`);
      passed = false;
    } else if (tc.arr[result[0]] + tc.arr[result[1]] !== tc.expectSum) {
      console.error(`❌ twoSum(${JSON.stringify(tc.arr)}, ${tc.target}) indices don't sum to ${tc.expectSum}`);
      passed = false;
    } else {
      console.log(`✓ twoSum target=${tc.target}`);
    }
  }

  console.log("\nTesting threeSum...");
  const threeSumResult = threeSum([-1, 0, 1, 2, -1, -4], 0);
  if (threeSumResult.length === 0) {
    console.error("❌ threeSum should find at least one triplet");
    passed = false;
  } else {
    let validTriplets = true;
    for (const triplet of threeSumResult) {
      if (triplet[0] + triplet[1] + triplet[2] !== 0) {
        validTriplets = false;
        break;
      }
    }
    if (validTriplets) {
      console.log(`✓ threeSum found ${threeSumResult.length} valid triplet(s)`);
    } else {
      console.error("❌ threeSum returned invalid triplets");
      passed = false;
    }
  }

  if (passed) {
    console.log("\n✅ All tests passed!");
  } else {
    console.log("\n❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

runCheck();
