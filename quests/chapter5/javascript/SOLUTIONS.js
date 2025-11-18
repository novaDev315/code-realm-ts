// Reference solutions for Chapter 5: Labyrinth of Nodes

function dfs(graph, start) {
  const visited = new Set();
  const result = [];

  function traverse(node) {
    if (visited.has(node)) return;
    visited.add(node);
    result.push(node);

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      traverse(neighbor);
    }
  }

  traverse(start);
  return result;
}

function bfs(graph, start) {
  const visited = new Set();
  const queue = [start];
  const result = [];

  visited.add(start);

  while (queue.length > 0) {
    const node = queue.shift();
    result.push(node);

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      if (!visited.has(neighbor)) {
        visited.add(neighbor);
        queue.push(neighbor);
      }
    }
  }

  return result;
}

function shortestPath(graph, start, end) {
  if (start === end) return [start];

  const visited = new Set();
  const queue = [{ node: start, path: [start] }];

  visited.add(start);

  while (queue.length > 0) {
    const { node, path } = queue.shift();

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      if (neighbor === end) {
        return [...path, end];
      }

      if (!visited.has(neighbor)) {
        visited.add(neighbor);
        queue.push({
          node: neighbor,
          path: [...path, neighbor],
        });
      }
    }
  }

  return [];
}

function hasCycle(graph) {
  const visited = new Set();
  const recursionStack = new Set();

  function dfsHelper(node) {
    visited.add(node);
    recursionStack.add(node);

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      if (!visited.has(neighbor)) {
        if (dfsHelper(neighbor)) return true;
      } else if (recursionStack.has(neighbor)) {
        return true;
      }
    }

    recursionStack.delete(node);
    return false;
  }

  for (const node of graph.keys()) {
    if (!visited.has(node)) {
      if (dfsHelper(node)) return true;
    }
  }

  return false;
}

module.exports = { dfs, bfs, shortestPath, hasCycle };
