// Chapter 5: Labyrinth of Nodes - Graphs
// Master Project

export type Graph = Map<number, number[]>;

export function dfs(graph: Graph, start: number): number[] {
  // TODO: Perform depth-first search and return visited nodes in order
  return [];
}

export function bfs(graph: Graph, start: number): number[] {
  // TODO: Perform breadth-first search and return visited nodes in order
  return [];
}

export function shortestPath(graph: Graph, start: number, end: number): number[] {
  // TODO: Find shortest path between start and end using BFS
  // Return path as array of nodes, or empty if no path
  return [];
}

export function hasCycle(graph: Graph): boolean {
  // TODO: Detect if graph has a cycle using DFS
  return false;
}
