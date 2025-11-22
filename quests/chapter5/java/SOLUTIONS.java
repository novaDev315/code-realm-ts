// Reference solutions for Chapter 5: Labyrinth of Nodes - Graph Algorithms

import java.util.*;

public class SOLUTIONS {
    /**
     * Depth-First Search (DFS) - SOLUTION
     * Uses recursion with a visited set to track explored nodes.
     */
    public static List<Integer> dfs(Map<Integer, List<Integer>> graph, int start) {
        Set<Integer> visited = new HashSet<>();
        List<Integer> result = new ArrayList<>();

        dfsHelper(graph, start, visited, result);
        return result;
    }

    private static void dfsHelper(Map<Integer, List<Integer>> graph, int node,
                                   Set<Integer> visited, List<Integer> result) {
        if (visited.contains(node)) {
            return;
        }

        visited.add(node);
        result.add(node);

        List<Integer> neighbors = graph.getOrDefault(node, Collections.emptyList());
        for (int neighbor : neighbors) {
            dfsHelper(graph, neighbor, visited, result);
        }
    }

    /**
     * Breadth-First Search (BFS) - SOLUTION
     * Uses a queue to process nodes level by level.
     */
    public static List<Integer> bfs(Map<Integer, List<Integer>> graph, int start) {
        Set<Integer> visited = new HashSet<>();
        Queue<Integer> queue = new LinkedList<>();
        List<Integer> result = new ArrayList<>();

        visited.add(start);
        queue.add(start);

        while (!queue.isEmpty()) {
            int node = queue.poll();
            result.add(node);

            List<Integer> neighbors = graph.getOrDefault(node, Collections.emptyList());
            for (int neighbor : neighbors) {
                if (!visited.contains(neighbor)) {
                    visited.add(neighbor);
                    queue.add(neighbor);
                }
            }
        }

        return result;
    }

    /**
     * Shortest Path (Unweighted Graph) - SOLUTION
     * Uses BFS with path tracking to find the shortest path.
     */
    public static List<Integer> shortestPath(Map<Integer, List<Integer>> graph, int start, int end) {
        if (start == end) {
            return Collections.singletonList(start);
        }

        Set<Integer> visited = new HashSet<>();
        Queue<PathNode> queue = new LinkedList<>();

        visited.add(start);
        queue.add(new PathNode(start, Collections.singletonList(start)));

        while (!queue.isEmpty()) {
            PathNode current = queue.poll();

            List<Integer> neighbors = graph.getOrDefault(current.node, Collections.emptyList());
            for (int neighbor : neighbors) {
                if (neighbor == end) {
                    List<Integer> path = new ArrayList<>(current.path);
                    path.add(end);
                    return path;
                }

                if (!visited.contains(neighbor)) {
                    visited.add(neighbor);
                    List<Integer> newPath = new ArrayList<>(current.path);
                    newPath.add(neighbor);
                    queue.add(new PathNode(neighbor, newPath));
                }
            }
        }

        return Collections.emptyList();
    }

    private static class PathNode {
        int node;
        List<Integer> path;

        PathNode(int node, List<Integer> path) {
            this.node = node;
            this.path = path;
        }
    }

    /**
     * Cycle Detection - SOLUTION
     * Uses DFS with a recursion stack to detect back edges.
     */
    public static boolean hasCycle(Map<Integer, List<Integer>> graph) {
        Set<Integer> visited = new HashSet<>();
        Set<Integer> recursionStack = new HashSet<>();

        for (int node : graph.keySet()) {
            if (!visited.contains(node)) {
                if (hasCycleHelper(graph, node, visited, recursionStack)) {
                    return true;
                }
            }
        }

        return false;
    }

    private static boolean hasCycleHelper(Map<Integer, List<Integer>> graph, int node,
                                           Set<Integer> visited, Set<Integer> recursionStack) {
        visited.add(node);
        recursionStack.add(node);

        List<Integer> neighbors = graph.getOrDefault(node, Collections.emptyList());
        for (int neighbor : neighbors) {
            if (!visited.contains(neighbor)) {
                if (hasCycleHelper(graph, neighbor, visited, recursionStack)) {
                    return true;
                }
            } else if (recursionStack.contains(neighbor)) {
                return true;
            }
        }

        recursionStack.remove(node);
        return false;
    }
}
