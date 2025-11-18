const { delayedSum, fetchMultiple, retryOperation, delayedFetch } = require("./async");

async function runCheck() {
  let passed = true;

  // Test delayedSum
  console.log("Testing delayedSum...");
  const sumCases = [
    { a: 5, b: 3, delayMs: 50, expect: 8 },
    { a: 10, b: 20, delayMs: 100, expect: 30 },
    { a: -5, b: 5, delayMs: 75, expect: 0 },
  ];

  for (const tc of sumCases) {
    const start = Date.now();
    const result = await delayedSum(tc.a, tc.b, tc.delayMs);
    const elapsed = Date.now() - start;

    if (result !== tc.expect) {
      console.error(
        `❌ delayedSum(${tc.a}, ${tc.b}, ${tc.delayMs}) expected ${tc.expect} but got ${result}`
      );
      passed = false;
    } else if (elapsed < tc.delayMs - 5) {
      console.error(
        `❌ delayedSum(${tc.a}, ${tc.b}, ${tc.delayMs}) completed too quickly (${elapsed}ms < ${tc.delayMs}ms)`
      );
      passed = false;
    } else {
      console.log(`✓ delayedSum(${tc.a}, ${tc.b}) = ${result} (${elapsed}ms)`);
    }
  }

  // Test fetchMultiple
  console.log("\nTesting fetchMultiple...");
  const urls = ["https://example.com", "https://api.example.com", "https://data.example.com"];
  const start = Date.now();
  const results = await fetchMultiple(urls);
  const fetchTime = Date.now() - start;

  if (results.length !== urls.length) {
    console.error(
      `❌ fetchMultiple(${urls.length} urls) expected ${urls.length} results but got ${results.length}`
    );
    passed = false;
  } else {
    let validResults = true;
    for (let i = 0; i < results.length; i++) {
      if (!results[i].includes("Data from")) {
        validResults = false;
        break;
      }
    }
    if (validResults) {
      console.log(`✓ fetchMultiple fetched ${urls.length} URLs in parallel (${fetchTime}ms)`);
    } else {
      console.error("❌ fetchMultiple returned invalid results");
      passed = false;
    }
  }

  // Test retryOperation - success case
  console.log("\nTesting retryOperation...");
  let attemptCount = 0;

  const successOperation = async () => {
    attemptCount++;
    return "success";
  };

  try {
    const result = await retryOperation(successOperation, 3);
    if (result === "success" && attemptCount === 1) {
      console.log(`✓ retryOperation succeeded on first try`);
    } else {
      console.error(
        `❌ retryOperation expected 1 attempt but got ${attemptCount}`
      );
      passed = false;
    }
  } catch (error) {
    console.error(`❌ retryOperation failed: ${error}`);
    passed = false;
  }

  // Test retryOperation - failure with retries
  console.log("\nTesting retryOperation with retries...");
  attemptCount = 0;

  const failThenSucceedOperation = async () => {
    attemptCount++;
    if (attemptCount < 3) {
      throw new Error("Failed attempt");
    }
    return "success";
  };

  try {
    const result = await retryOperation(failThenSucceedOperation, 5);
    if (result === "success" && attemptCount === 3) {
      console.log(`✓ retryOperation succeeded after ${attemptCount} attempts`);
    } else {
      console.error(
        `❌ retryOperation expected 3 attempts but got ${attemptCount}`
      );
      passed = false;
    }
  } catch (error) {
    console.error(`❌ retryOperation failed: ${error}`);
    passed = false;
  }

  // Test retryOperation - exhausts retries
  console.log("\nTesting retryOperation exhaustion...");
  attemptCount = 0;

  const alwaysFailOperation = async () => {
    attemptCount++;
    throw new Error("Always fails");
  };

  try {
    await retryOperation(alwaysFailOperation, 2);
    console.error("❌ retryOperation should have thrown after exhausting retries");
    passed = false;
  } catch (error) {
    if (attemptCount === 3) {
      // 1 initial attempt + 2 retries = 3 total
      console.log(`✓ retryOperation exhausted retries after ${attemptCount} attempts`);
    } else {
      console.error(
        `❌ retryOperation expected 3 attempts but got ${attemptCount}`
      );
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

runCheck().catch((error) => {
  console.error("Fatal error:", error);
  process.exit(1);
});
