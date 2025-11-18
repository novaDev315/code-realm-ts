// Reference solutions for Chapter 5: Labyrinth of Nodes

export type Graph = Map<number, number[]>;

export function dfs(graph: Graph, start: number): number[] {
  const visited = new Set<number>();
  const result: number[] = [];

  function traverse(node: number) {
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

export function bfs(graph: Graph, start: number): number[] {
  const visited = new Set<number>();
  const queue: number[] = [start];
  const result: number[] = [];

  visited.add(start);

  while (queue.length > 0) {
    const node = queue.shift()!;
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

export function shortestPath(graph: Graph, start: number, end: number): number[] {
  if (start === end) return [start];

  const visited = new Set<number>();
  const queue: { node: number; path: number[] }[] = [{ node: start, path: [start] }];

  visited.add(start);

  while (queue.length > 0) {
    const { node, path } = queue.shift()!;

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

export function hasCycle(graph: Graph): boolean {
  const visited = new Set<number>();
  const recursionStack = new Set<number>();

  function dfs(node: number): boolean {
    visited.add(node);
    recursionStack.add(node);

    const neighbors = graph.get(node) || [];
    for (const neighbor of neighbors) {
      if (!visited.has(neighbor)) {
        if (dfs(neighbor)) return true;
      } else if (recursionStack.has(neighbor)) {
        return true;
      }
    }

    recursionStack.delete(node);
    return false;
  }

  for (const node of graph.keys()) {
    if (!visited.has(node)) {
      if (dfs(node)) return true;
    }
  }

  return false;
}
