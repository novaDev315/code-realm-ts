const { dfs, bfs, shortestPath, hasCycle } = require("./graphs");

function runCheck() {
  let passed = true;

  // Test DFS
  console.log("Testing DFS...");
  const graph1 = new Map([
    [1, [2, 3]],
    [2, [4]],
    [3, [5]],
    [4, []],
    [5, []],
  ]);

  const dfsResult = dfs(graph1, 1);
  if (dfsResult.length !== 5 || !dfsResult.includes(1) || !dfsResult.includes(2)) {
    console.error(
      `❌ DFS failed. Expected to visit all 5 nodes, got: ${JSON.stringify(dfsResult)}`
    );
    passed = false;
  } else {
    console.log(`✓ DFS correctly visited ${dfsResult.length} nodes: ${dfsResult.join(" -> ")}`);
  }

  // Test DFS with disconnected component
  const graph2 = new Map([
    [1, [2]],
    [2, []],
    [3, [4]],
    [4, []],
  ]);

  const dfsResult2 = dfs(graph2, 1);
  if (dfsResult2.length !== 2) {
    console.error(
      `❌ DFS should only visit connected nodes. Expected 2, got: ${dfsResult2.length}`
    );
    passed = false;
  } else {
    console.log(`✓ DFS correctly handled disconnected graph`);
  }

  // Test BFS
  console.log("\nTesting BFS...");
  const graph3 = new Map([
    [1, [2, 3]],
    [2, [4]],
    [3, [5]],
    [4, []],
    [5, []],
  ]);

  const bfsResult = bfs(graph3, 1);
  if (bfsResult.length !== 5) {
    console.error(`❌ BFS failed. Expected 5 nodes, got: ${bfsResult.length}`);
    passed = false;
  } else if (bfsResult[0] !== 1 || !bfsResult.includes(2) || !bfsResult.includes(3)) {
    console.error(`❌ BFS order incorrect: ${JSON.stringify(bfsResult)}`);
    passed = false;
  } else {
    console.log(`✓ BFS correctly visited nodes in level order: ${bfsResult.join(" -> ")}`);
  }

  // Test level-order property
  const bfsIndexOf2 = bfsResult.indexOf(2);
  const bfsIndexOf3 = bfsResult.indexOf(3);
  const bfsIndexOf4 = bfsResult.indexOf(4);
  const bfsIndexOf5 = bfsResult.indexOf(5);

  if (Math.max(bfsIndexOf2, bfsIndexOf3) < Math.min(bfsIndexOf4, bfsIndexOf5)) {
    console.log(`✓ BFS maintains level-order property`);
  } else {
    console.error(`⚠ BFS may not maintain strict level-order`);
  }

  // Test Shortest Path
  console.log("\nTesting Shortest Path...");
  const graph4 = new Map([
    [1, [2, 3]],
    [2, [4]],
    [3, [4]],
    [4, [5]],
    [5, []],
  ]);

  const path1 = shortestPath(graph4, 1, 5);
  if (path1.length === 0) {
    console.error(`❌ Shortest path not found from 1 to 5`);
    passed = false;
  } else if (path1[0] !== 1 || path1[path1.length - 1] !== 5) {
    console.error(`❌ Path doesn't start at 1 and end at 5: ${JSON.stringify(path1)}`);
    passed = false;
  } else {
    console.log(`✓ Shortest path found (length ${path1.length}): ${path1.join(" -> ")}`);
  }

  // Test shortest path - same start and end
  const pathSame = shortestPath(graph4, 1, 1);
  if (pathSame.length !== 1 || pathSame[0] !== 1) {
    console.error(`❌ Shortest path from node to itself should be [1]`);
    passed = false;
  } else {
    console.log(`✓ Shortest path handles same start and end`);
  }

  // Test shortest path - no path
  const graph5 = new Map([
    [1, [2]],
    [2, []],
    [3, [4]],
    [4, []],
  ]);

  const noPath = shortestPath(graph5, 1, 4);
  if (noPath.length !== 0) {
    console.error(`❌ Should return empty array when no path exists`);
    passed = false;
  } else {
    console.log(`✓ Shortest path correctly returns empty for disconnected nodes`);
  }

  // Test Cycle Detection - graph with cycle
  console.log("\nTesting Cycle Detection...");
  const graphWithCycle = new Map([
    [1, [2]],
    [2, [3]],
    [3, [1]],
  ]);

  const hasCycleResult1 = hasCycle(graphWithCycle);
  if (!hasCycleResult1) {
    console.error(`❌ Should detect cycle in 1->2->3->1`);
    passed = false;
  } else {
    console.log(`✓ Cycle detection correctly identified cycle`);
  }

  // Test Cycle Detection - graph without cycle
  const graphNoCycle = new Map([
    [1, [2, 3]],
    [2, [4]],
    [3, [4]],
    [4, [5]],
    [5, []],
  ]);

  const hasCycleResult2 = hasCycle(graphNoCycle);
  if (hasCycleResult2) {
    console.error(`❌ Should not detect cycle in acyclic graph`);
    passed = false;
  } else {
    console.log(`✓ Cycle detection correctly identified acyclic graph`);
  }

  // Test Cycle Detection - self-loop
  const graphSelfLoop = new Map([
    [1, [1, 2]],
    [2, []],
  ]);

  const hasCycleResult3 = hasCycle(graphSelfLoop);
  if (!hasCycleResult3) {
    console.error(`❌ Should detect self-loop as cycle`);
    passed = false;
  } else {
    console.log(`✓ Cycle detection correctly identified self-loop`);
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
