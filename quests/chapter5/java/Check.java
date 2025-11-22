// Test suite for Chapter 5: Labyrinth of Nodes - Graph Algorithms

import java.util.*;

public class Check {
    public static void main(String[] args) {
        boolean passed = true;

        // Test DFS
        System.out.println("Testing DFS...");
        Map<Integer, List<Integer>> graph1 = new HashMap<>();
        graph1.put(1, Arrays.asList(2, 3));
        graph1.put(2, Arrays.asList(4));
        graph1.put(3, Arrays.asList(5));
        graph1.put(4, Arrays.asList());
        graph1.put(5, Arrays.asList());

        List<Integer> dfsResult = Graphs.dfs(graph1, 1);
        if (dfsResult.size() != 5 || !dfsResult.contains(1) || !dfsResult.contains(2)) {
            System.err.println("x DFS failed. Expected to visit all 5 nodes, got: " + dfsResult);
            passed = false;
        } else {
            System.out.println("v DFS correctly visited " + dfsResult.size() + " nodes: " + formatPath(dfsResult));
        }

        // Test DFS with disconnected component
        Map<Integer, List<Integer>> graph2 = new HashMap<>();
        graph2.put(1, Arrays.asList(2));
        graph2.put(2, Arrays.asList());
        graph2.put(3, Arrays.asList(4));
        graph2.put(4, Arrays.asList());

        List<Integer> dfsResult2 = Graphs.dfs(graph2, 1);
        if (dfsResult2.size() != 2) {
            System.err.println("x DFS should only visit connected nodes. Expected 2, got: " + dfsResult2.size());
            passed = false;
        } else {
            System.out.println("v DFS correctly handled disconnected graph");
        }

        // Test BFS
        System.out.println("\nTesting BFS...");
        Map<Integer, List<Integer>> graph3 = new HashMap<>();
        graph3.put(1, Arrays.asList(2, 3));
        graph3.put(2, Arrays.asList(4));
        graph3.put(3, Arrays.asList(5));
        graph3.put(4, Arrays.asList());
        graph3.put(5, Arrays.asList());

        List<Integer> bfsResult = Graphs.bfs(graph3, 1);
        if (bfsResult.size() != 5) {
            System.err.println("x BFS failed. Expected 5 nodes, got: " + bfsResult.size());
            passed = false;
        } else if (bfsResult.get(0) != 1 || !bfsResult.contains(2) || !bfsResult.contains(3)) {
            System.err.println("x BFS order incorrect: " + bfsResult);
            passed = false;
        } else {
            System.out.println("v BFS correctly visited nodes in level order: " + formatPath(bfsResult));
        }

        // Test level-order property (BFS should visit all level-1 before level-2)
        int bfsIndexOf2 = bfsResult.indexOf(2);
        int bfsIndexOf3 = bfsResult.indexOf(3);
        int bfsIndexOf4 = bfsResult.indexOf(4);
        int bfsIndexOf5 = bfsResult.indexOf(5);

        if (Math.max(bfsIndexOf2, bfsIndexOf3) < Math.min(bfsIndexOf4, bfsIndexOf5)) {
            System.out.println("v BFS maintains level-order property");
        } else {
            System.err.println("! BFS may not maintain strict level-order");
        }

        // Test Shortest Path
        System.out.println("\nTesting Shortest Path...");
        Map<Integer, List<Integer>> graph4 = new HashMap<>();
        graph4.put(1, Arrays.asList(2, 3));
        graph4.put(2, Arrays.asList(4));
        graph4.put(3, Arrays.asList(4));
        graph4.put(4, Arrays.asList(5));
        graph4.put(5, Arrays.asList());

        List<Integer> path1 = Graphs.shortestPath(graph4, 1, 5);
        if (path1.isEmpty()) {
            System.err.println("x Shortest path not found from 1 to 5");
            passed = false;
        } else if (path1.get(0) != 1 || path1.get(path1.size() - 1) != 5) {
            System.err.println("x Path doesn't start at 1 and end at 5: " + path1);
            passed = false;
        } else {
            System.out.println("v Shortest path found (length " + path1.size() + "): " + formatPath(path1));
        }

        // Test shortest path - same start and end
        List<Integer> pathSame = Graphs.shortestPath(graph4, 1, 1);
        if (pathSame.size() != 1 || pathSame.get(0) != 1) {
            System.err.println("x Shortest path from node to itself should be [1]");
            passed = false;
        } else {
            System.out.println("v Shortest path handles same start and end");
        }

        // Test shortest path - no path
        Map<Integer, List<Integer>> graph5 = new HashMap<>();
        graph5.put(1, Arrays.asList(2));
        graph5.put(2, Arrays.asList());
        graph5.put(3, Arrays.asList(4));
        graph5.put(4, Arrays.asList());

        List<Integer> noPath = Graphs.shortestPath(graph5, 1, 4);
        if (!noPath.isEmpty()) {
            System.err.println("x Should return empty list when no path exists");
            passed = false;
        } else {
            System.out.println("v Shortest path correctly returns empty for disconnected nodes");
        }

        // Test Cycle Detection - graph with cycle
        System.out.println("\nTesting Cycle Detection...");
        Map<Integer, List<Integer>> graphWithCycle = new HashMap<>();
        graphWithCycle.put(1, Arrays.asList(2));
        graphWithCycle.put(2, Arrays.asList(3));
        graphWithCycle.put(3, Arrays.asList(1));

        boolean hasCycleResult1 = Graphs.hasCycle(graphWithCycle);
        if (!hasCycleResult1) {
            System.err.println("x Should detect cycle in 1->2->3->1");
            passed = false;
        } else {
            System.out.println("v Cycle detection correctly identified cycle");
        }

        // Test Cycle Detection - graph without cycle
        Map<Integer, List<Integer>> graphNoCycle = new HashMap<>();
        graphNoCycle.put(1, Arrays.asList(2, 3));
        graphNoCycle.put(2, Arrays.asList(4));
        graphNoCycle.put(3, Arrays.asList(4));
        graphNoCycle.put(4, Arrays.asList(5));
        graphNoCycle.put(5, Arrays.asList());

        boolean hasCycleResult2 = Graphs.hasCycle(graphNoCycle);
        if (hasCycleResult2) {
            System.err.println("x Should not detect cycle in acyclic graph");
            passed = false;
        } else {
            System.out.println("v Cycle detection correctly identified acyclic graph");
        }

        // Test Cycle Detection - self-loop
        Map<Integer, List<Integer>> graphSelfLoop = new HashMap<>();
        graphSelfLoop.put(1, Arrays.asList(1, 2));
        graphSelfLoop.put(2, Arrays.asList());

        boolean hasCycleResult3 = Graphs.hasCycle(graphSelfLoop);
        if (!hasCycleResult3) {
            System.err.println("x Should detect self-loop as cycle");
            passed = false;
        } else {
            System.out.println("v Cycle detection correctly identified self-loop");
        }

        if (passed) {
            System.out.println("\n[PASS] All tests passed!");
        } else {
            System.out.println("\n[FAIL] Some tests failed.");
            System.exit(1);
        }
    }

    private static String formatPath(List<Integer> path) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < path.size(); i++) {
            sb.append(path.get(i));
            if (i < path.size() - 1) {
                sb.append(" -> ");
            }
        }
        return sb.toString();
    }
}
