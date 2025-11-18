const { LRUCache, cacheHitRate } = require("./cache");

function runCheck() {
  let passed = true;

  // Test LRUCache basic operations
  console.log("Testing LRUCache basic operations...");
  const cache = new LRUCache(2);

  // Test put and get
  cache.put("a", 1);
  cache.put("b", 2);

  const getA = cache.get("a");
  if (getA !== 1) {
    console.error(`❌ Expected cache.get("a") = 1, got ${getA}`);
    passed = false;
  } else {
    console.log("✓ Put and get work correctly");
  }

  // Test capacity and eviction
  console.log("\nTesting eviction policy...");
  cache.put("c", 3); // Should evict "b" since "a" was just accessed

  const getB = cache.get("b");
  if (getB !== null) {
    console.error(`❌ Expected cache.get("b") = null (evicted), got ${getB}`);
    passed = false;
  } else {
    console.log("✓ LRU eviction policy works correctly");
  }

  // Test cache size
  console.log("\nTesting cache size...");
  if (cache.size() !== 2) {
    console.error(`❌ Expected cache size = 2, got ${cache.size()}`);
    passed = false;
  } else {
    console.log("✓ Cache size correct");
  }

  // Test has method
  console.log("\nTesting has method...");
  if (!cache.has("a")) {
    console.error(`❌ Expected cache.has("a") = true`);
    passed = false;
  } else if (cache.has("b")) {
    console.error(`❌ Expected cache.has("b") = false`);
    passed = false;
  } else {
    console.log("✓ Has method works correctly");
  }

  // Test updating existing key doesn't evict
  console.log("\nTesting update of existing key...");
  cache.put("a", 10);
  if (cache.get("a") !== 10) {
    console.error(`❌ Expected cache.get("a") = 10 after update`);
    passed = false;
  } else {
    console.log("✓ Update of existing key works");
  }

  // Test cache hit rate calculation
  console.log("\nTesting cache hit rate calculation...");
  const operations = [
    { op: "put", key: "x", value: 1 },
    { op: "put", key: "y", value: 2 },
    { op: "get", key: "x" },  // hit
    { op: "get", key: "y" },  // hit
    { op: "get", key: "z" },  // miss
    { op: "get", key: "x" },  // hit
  ];

  const hitRate = cacheHitRate(operations);
  const expectedHitRate = 75; // 3 hits out of 4 gets = 75%

  if (Math.abs(hitRate - expectedHitRate) > 0.01) {
    console.error(`❌ Expected hit rate ≈ ${expectedHitRate}%, got ${hitRate}%`);
    passed = false;
  } else {
    console.log(`✓ Cache hit rate calculation correct (${hitRate}%)`);
  }

  // Test with different capacity
  console.log("\nTesting with capacity = 3...");
  const cache3 = new LRUCache(3);
  cache3.put("1", "a");
  cache3.put("2", "b");
  cache3.put("3", "c");
  cache3.get("1"); // Access "1" to make it recently used
  cache3.put("4", "d"); // Should evict "2"

  if (cache3.has("2")) {
    console.error(`❌ Expected "2" to be evicted`);
    passed = false;
  } else if (!cache3.has("1")) {
    console.error(`❌ Expected "1" to still be in cache`);
    passed = false;
  } else {
    console.log("✓ Larger cache capacity works correctly");
  }

  if (passed) {
    console.log("\n✅ All tests passed!");
  } else {
    console.log("\n❌ Some tests failed.");
    process.exit(1);
  }

  return passed;
}

if (require.main === module) {
  runCheck();
}

module.exports = { runCheck };
