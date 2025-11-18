const { RoundRobinBalancer, CircuitBreaker, rateLimitSlidingWindow } = require("./loadbalancer");

function runCheck() {
  let passed = true;

  // Test 1: Round-Robin Balancer
  console.log("\n⚖️  Testing Round-Robin Balancer...");
  const servers = ["server1", "server2", "server3"];
  const balancer = new RoundRobinBalancer(servers);

  const roundRobinTests = [
    { expectedIndex: 0, description: "First call should return first server" },
    { expectedIndex: 1, description: "Second call should return second server" },
    { expectedIndex: 2, description: "Third call should return third server" },
    { expectedIndex: 0, description: "Fourth call should wrap around to first server" },
    { expectedIndex: 1, description: "Fifth call should return second server" }
  ];

  for (const test of roundRobinTests) {
    const result = balancer.getNextServer();
    const expectedServer = servers[test.expectedIndex];

    if (result !== expectedServer) {
      console.error(`Round-robin: ${test.description}, expected "${expectedServer}", got "${result}"`);
      passed = false;
    } else {
      console.log(`✓ ${test.description}: "${result}"`);
    }
  }

  // Test 2: Circuit Breaker
  console.log("\n🔌 Testing Circuit Breaker...");
  const breaker = new CircuitBreaker(3);

  // Test initial state
  if (breaker.getState() !== 'CLOSED') {
    console.error(`Initial state should be 'CLOSED', got '${breaker.getState()}'`);
    passed = false;
  } else {
    console.log(`✓ Initial circuit breaker state: CLOSED`);
  }

  // Test failures and state transitions
  const circuitTests = [
    { failures: 1, expectedState: 'CLOSED', description: "1 failure should keep CLOSED" },
    { failures: 2, expectedState: 'CLOSED', description: "2 failures should keep CLOSED" },
    { failures: 3, expectedState: 'OPEN', description: "3 failures should OPEN circuit" }
  ];

  let totalFailures = 0;
  for (const test of circuitTests) {
    const newFailures = test.failures - totalFailures;
    for (let i = 0; i < newFailures; i++) {
      breaker.call(() => false); // Simulate failure
    }
    totalFailures = test.failures;

    const state = breaker.getState();
    if (state !== test.expectedState) {
      console.error(`After ${test.failures} failures, state should be '${test.expectedState}', got '${state}'`);
      passed = false;
    } else {
      console.log(`✓ ${test.description}: ${state}`);
    }
  }

  // Test 3: Sliding Window Rate Limiting
  console.log("\n📊 Testing Sliding Window Rate Limiting...");
  const now = Date.now();

  const rateLimitTests = [
    {
      requests: [now, now - 100, now - 200],
      windowSize: 1000,
      limit: 2,
      shouldExceed: true,
      description: "3 requests in 1000ms window with limit 2 should exceed"
    },
    {
      requests: [now, now - 100],
      windowSize: 1000,
      limit: 3,
      shouldExceed: false,
      description: "2 requests in 1000ms window with limit 3 should not exceed"
    },
    {
      requests: [now],
      windowSize: 1000,
      limit: 1,
      shouldExceed: false,
      description: "1 request in 1000ms window with limit 1 should not exceed"
    },
    {
      requests: [now, now - 500, now - 1500],
      windowSize: 1000,
      limit: 2,
      shouldExceed: false,
      description: "Request outside window should not be counted"
    }
  ];

  for (const test of rateLimitTests) {
    const result = rateLimitSlidingWindow(test.requests, test.windowSize, test.limit);

    if (result !== test.shouldExceed) {
      console.error(`${test.description}, expected ${test.shouldExceed}, got ${result}`);
      passed = false;
    } else {
      console.log(`✓ ${test.description}: ${result}`);
    }
  }

  // Summary
  console.log("\n" + "=".repeat(50));
  if (passed) {
    console.log("✅ All load balancing tests passed!");
  } else {
    console.log("❌ Some load balancing tests failed.");
    process.exit(1);
  }

  return passed;
}

// Allow running as standalone script
if (require.main === module) {
  runCheck();
}

module.exports = { runCheck };
