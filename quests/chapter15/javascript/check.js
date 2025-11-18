const { DistributedSystem, designScalableAPI, optimizeDatabase } = require("./architect");

function runCheck() {
  let passed = true;

  // Test 1: DistributedSystem initialization
  console.log("Test 1: DistributedSystem initialization...");
  try {
    const system = new DistributedSystem();

    const initialMetrics = system.getMetrics();
    if (
      typeof initialMetrics.cacheHits !== 'number' ||
      typeof initialMetrics.cacheSize !== 'number' ||
      typeof initialMetrics.queueSize !== 'number' ||
      typeof initialMetrics.totalRequests !== 'number'
    ) {
      console.error("getMetrics() should return object with numeric properties");
      passed = false;
    }

    if (initialMetrics.cacheHits !== 0 || initialMetrics.totalRequests !== 0) {
      console.error("Initial metrics should have 0 hits and 0 requests");
      passed = false;
    }
  } catch (error) {
    console.error("DistributedSystem initialization failed:", error);
    passed = false;
  }

  // Test 2: Process single request
  console.log("Test 2: Process single request...");
  try {
    const system = new DistributedSystem();

    const request = {
      id: "req-1",
      data: { value: "test" },
      priority: 1,
      timestamp: Date.now()
    };

    const response = system.processRequest(request);

    if (!response) {
      console.error("processRequest() should return a response, got null");
      passed = false;
    } else if (response.requestId !== request.id) {
      console.error(`Response should have requestId matching request.id: ${request.id}`);
      passed = false;
    }
  } catch (error) {
    console.error("Process single request failed:", error);
    passed = false;
  }

  // Test 3: Cache functionality
  console.log("Test 3: Cache functionality...");
  try {
    const system = new DistributedSystem();

    const request1 = {
      id: "req-cached",
      data: { value: "cached-data" },
      priority: 1,
      timestamp: Date.now()
    };

    const response1 = system.processRequest(request1);
    const metrics1 = system.getMetrics();
    const initialSize = metrics1.cacheSize;

    // Process same request again - should hit cache
    const response2 = system.processRequest(request1);
    const metrics2 = system.getMetrics();

    if (response1.cachedFromCache === true) {
      console.error("First request shouldn't be from cache");
      passed = false;
    }

    if (response2.cachedFromCache !== true) {
      console.error("Second request with same ID should be from cache");
      passed = false;
    }

    if (metrics2.cacheHits < 1) {
      console.error("Cache hits should increase on cache hit");
      passed = false;
    }
  } catch (error) {
    console.error("Cache functionality test failed:", error);
    passed = false;
  }

  // Test 4: Multiple requests with load balancing
  console.log("Test 4: Load balancing across multiple requests...");
  try {
    const system = new DistributedSystem();

    const requests = [];
    for (let i = 0; i < 5; i++) {
      requests.push({
        id: `req-lb-${i}`,
        data: { index: i },
        priority: i % 3,
        timestamp: Date.now()
      });
    }

    const servers = new Set();
    for (const req of requests) {
      const response = system.processRequest(req);
      if (response && response.server) {
        servers.add(response.server);
      }
    }

    if (servers.size === 0) {
      console.error("Load balancer should assign servers to requests");
      passed = false;
    }
  } catch (error) {
    console.error("Load balancing test failed:", error);
    passed = false;
  }

  // Test 5: Queue processing
  console.log("Test 5: Queue processing...");
  try {
    const system = new DistributedSystem();

    // Add multiple requests to queue
    for (let i = 0; i < 15; i++) {
      system.processRequest({
        id: `queue-req-${i}`,
        data: { index: i },
        priority: 1,
        timestamp: Date.now()
      });
    }

    const metricsBefore = system.getMetrics();
    system.processQueue();
    const metricsAfter = system.getMetrics();

    if (metricsAfter.totalRequests <= metricsBefore.totalRequests) {
      console.error("processQueue() should increase totalRequests");
      passed = false;
    }
  } catch (error) {
    console.error("Queue processing test failed:", error);
    passed = false;
  }

  // Test 6: Design scalable API
  console.log("Test 6: Design scalable API...");
  try {
    const requirements1 = {
      expectedRPS: 5000,
      dataSize: "large",
      caching: true,
      auth: true
    };

    const design1 = designScalableAPI(requirements1);

    if (!Array.isArray(design1.components)) {
      console.error("designScalableAPI should return components array");
      passed = false;
    }

    if (design1.estimated_servers < 5) {
      console.error("High RPS (5000) should require many servers (got " + design1.estimated_servers + ")");
      passed = false;
    }

    if (!design1.architecture) {
      console.error("designScalableAPI should return architecture description");
      passed = false;
    }

    // Should include load balancer for high RPS
    if (!design1.components.includes("load-balancer")) {
      console.error("Design with 5000 RPS should include load-balancer");
      passed = false;
    }

    // Should include cache for large data
    if (!design1.components.includes("cache")) {
      console.error("Design with large data should include cache");
      passed = false;
    }
  } catch (error) {
    console.error("Design scalable API test failed:", error);
    passed = false;
  }

  // Test 7: Low-scale API design
  console.log("Test 7: Low-scale API design...");
  try {
    const requirements2 = {
      expectedRPS: 100,
      dataSize: "small",
      caching: false,
      auth: false
    };

    const design2 = designScalableAPI(requirements2);

    if (design2.estimated_servers > 2) {
      console.error("Low RPS (100) should require fewer servers");
      passed = false;
    }

    if (design2.components.length === 0) {
      console.error("Design should still include some components");
      passed = false;
    }
  } catch (error) {
    console.error("Low-scale API design test failed:", error);
    passed = false;
  }

  // Test 8: Optimize database queries
  console.log("Test 8: Optimize database queries...");
  try {
    const queries = [
      "SELECT * FROM users",
      "SELECT * FROM orders WHERE status = 'pending'",
      "SELECT id, name FROM users WHERE id = 123",
      "SELECT * FROM products JOIN inventory ON products.id = inventory.product_id",
      "SELECT COUNT(*) FROM logs WHERE YEAR(date) = 2024"
    ];

    const optimization = optimizeDatabase(queries);

    if (!Array.isArray(optimization.slowQueries)) {
      console.error("optimizeDatabase should return slowQueries array");
      passed = false;
    }

    if (!Array.isArray(optimization.suggestions)) {
      console.error("optimizeDatabase should return suggestions array");
      passed = false;
    }

    if (optimization.slowQueries.length === 0) {
      console.error("Should identify slow queries from the list");
      passed = false;
    }

    if (optimization.suggestions.length === 0) {
      console.error("Should provide optimization suggestions");
      passed = false;
    }
  } catch (error) {
    console.error("Database optimization test failed:", error);
    passed = false;
  }

  // Test 9: Priority queue handling
  console.log("Test 9: Priority queue handling...");
  try {
    const system = new DistributedSystem();

    const highPriority = {
      id: "high-priority",
      data: { urgent: true },
      priority: 10,
      timestamp: Date.now()
    };

    const lowPriority = {
      id: "low-priority",
      data: { urgent: false },
      priority: 1,
      timestamp: Date.now()
    };

    const resp1 = system.processRequest(lowPriority);
    const resp2 = system.processRequest(highPriority);

    if (!resp1 || !resp2) {
      console.error("Both requests should be processed");
      passed = false;
    }

    const metrics = system.getMetrics();
    if (metrics.totalRequests < 2) {
      console.error("Should track both requests");
      passed = false;
    }
  } catch (error) {
    console.error("Priority queue handling test failed:", error);
    passed = false;
  }

  // Test 10: Metrics tracking
  console.log("Test 10: Metrics tracking...");
  try {
    const system = new DistributedSystem();

    for (let i = 0; i < 10; i++) {
      system.processRequest({
        id: `metrics-req-${i}`,
        data: { index: i },
        priority: 1,
        timestamp: Date.now()
      });
    }

    const metrics = system.getMetrics();

    if (metrics.totalRequests < 10) {
      console.error("Should track all 10 requests");
      passed = false;
    }

    if (metrics.cacheSize < 0) {
      console.error("Cache size cannot be negative");
      passed = false;
    }

    if (metrics.cacheHits < 0) {
      console.error("Cache hits cannot be negative");
      passed = false;
    }
  } catch (error) {
    console.error("Metrics tracking test failed:", error);
    passed = false;
  }

  if (passed) {
    console.log("\n✅ All Chapter 15 tests passed! You have mastered the Core of the Architect!");
  } else {
    console.log("\n❌ Some tests failed. Review the SOLUTIONS.js for reference implementation.");
    process.exit(1);
  }

  return passed;
}

// Allow running as standalone script
if (require.main === module) {
  runCheck();
}

module.exports = { runCheck };
