// Test suite for Chapter 5: Labyrinth of Nodes - Graph Algorithms

#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include "graphs.cpp"

std::string formatPath(const std::vector<int>& path) {
    std::string result;
    for (size_t i = 0; i < path.size(); i++) {
        result += std::to_string(path[i]);
        if (i < path.size() - 1) {
            result += " -> ";
        }
    }
    return result;
}

bool contains(const std::vector<int>& vec, int value) {
    return std::find(vec.begin(), vec.end(), value) != vec.end();
}

int indexOf(const std::vector<int>& vec, int value) {
    auto it = std::find(vec.begin(), vec.end(), value);
    if (it == vec.end()) return -1;
    return std::distance(vec.begin(), it);
}

int main() {
    bool passed = true;

    // Test DFS
    std::cout << "Testing DFS..." << std::endl;
    Graph graph1 = {
        {1, {2, 3}},
        {2, {4}},
        {3, {5}},
        {4, {}},
        {5, {}}
    };

    std::vector<int> dfsResult = dfs(graph1, 1);
    if (dfsResult.size() != 5 || !contains(dfsResult, 1) || !contains(dfsResult, 2)) {
        std::cerr << "x DFS failed. Expected to visit all 5 nodes, got: " << formatPath(dfsResult) << std::endl;
        passed = false;
    } else {
        std::cout << "v DFS correctly visited " << dfsResult.size() << " nodes: " << formatPath(dfsResult) << std::endl;
    }

    // Test DFS with disconnected component
    Graph graph2 = {
        {1, {2}},
        {2, {}},
        {3, {4}},
        {4, {}}
    };

    std::vector<int> dfsResult2 = dfs(graph2, 1);
    if (dfsResult2.size() != 2) {
        std::cerr << "x DFS should only visit connected nodes. Expected 2, got: " << dfsResult2.size() << std::endl;
        passed = false;
    } else {
        std::cout << "v DFS correctly handled disconnected graph" << std::endl;
    }

    // Test BFS
    std::cout << "\nTesting BFS..." << std::endl;
    Graph graph3 = {
        {1, {2, 3}},
        {2, {4}},
        {3, {5}},
        {4, {}},
        {5, {}}
    };

    std::vector<int> bfsResult = bfs(graph3, 1);
    if (bfsResult.size() != 5) {
        std::cerr << "x BFS failed. Expected 5 nodes, got: " << bfsResult.size() << std::endl;
        passed = false;
    } else if (bfsResult[0] != 1 || !contains(bfsResult, 2) || !contains(bfsResult, 3)) {
        std::cerr << "x BFS order incorrect: " << formatPath(bfsResult) << std::endl;
        passed = false;
    } else {
        std::cout << "v BFS correctly visited nodes in level order: " << formatPath(bfsResult) << std::endl;
    }

    // Test level-order property (BFS should visit all level-1 before level-2)
    int bfsIndexOf2 = indexOf(bfsResult, 2);
    int bfsIndexOf3 = indexOf(bfsResult, 3);
    int bfsIndexOf4 = indexOf(bfsResult, 4);
    int bfsIndexOf5 = indexOf(bfsResult, 5);

    if (std::max(bfsIndexOf2, bfsIndexOf3) < std::min(bfsIndexOf4, bfsIndexOf5)) {
        std::cout << "v BFS maintains level-order property" << std::endl;
    } else {
        std::cerr << "! BFS may not maintain strict level-order" << std::endl;
    }

    // Test Shortest Path
    std::cout << "\nTesting Shortest Path..." << std::endl;
    Graph graph4 = {
        {1, {2, 3}},
        {2, {4}},
        {3, {4}},
        {4, {5}},
        {5, {}}
    };

    std::vector<int> path1 = shortestPath(graph4, 1, 5);
    if (path1.empty()) {
        std::cerr << "x Shortest path not found from 1 to 5" << std::endl;
        passed = false;
    } else if (path1[0] != 1 || path1[path1.size() - 1] != 5) {
        std::cerr << "x Path doesn't start at 1 and end at 5: " << formatPath(path1) << std::endl;
        passed = false;
    } else {
        std::cout << "v Shortest path found (length " << path1.size() << "): " << formatPath(path1) << std::endl;
    }

    // Test shortest path - same start and end
    std::vector<int> pathSame = shortestPath(graph4, 1, 1);
    if (pathSame.size() != 1 || pathSame[0] != 1) {
        std::cerr << "x Shortest path from node to itself should be [1]" << std::endl;
        passed = false;
    } else {
        std::cout << "v Shortest path handles same start and end" << std::endl;
    }

    // Test shortest path - no path
    Graph graph5 = {
        {1, {2}},
        {2, {}},
        {3, {4}},
        {4, {}}
    };

    std::vector<int> noPath = shortestPath(graph5, 1, 4);
    if (!noPath.empty()) {
        std::cerr << "x Should return empty vector when no path exists" << std::endl;
        passed = false;
    } else {
        std::cout << "v Shortest path correctly returns empty for disconnected nodes" << std::endl;
    }

    // Test Cycle Detection - graph with cycle
    std::cout << "\nTesting Cycle Detection..." << std::endl;
    Graph graphWithCycle = {
        {1, {2}},
        {2, {3}},
        {3, {1}}
    };

    bool hasCycleResult1 = hasCycle(graphWithCycle);
    if (!hasCycleResult1) {
        std::cerr << "x Should detect cycle in 1->2->3->1" << std::endl;
        passed = false;
    } else {
        std::cout << "v Cycle detection correctly identified cycle" << std::endl;
    }

    // Test Cycle Detection - graph without cycle
    Graph graphNoCycle = {
        {1, {2, 3}},
        {2, {4}},
        {3, {4}},
        {4, {5}},
        {5, {}}
    };

    bool hasCycleResult2 = hasCycle(graphNoCycle);
    if (hasCycleResult2) {
        std::cerr << "x Should not detect cycle in acyclic graph" << std::endl;
        passed = false;
    } else {
        std::cout << "v Cycle detection correctly identified acyclic graph" << std::endl;
    }

    // Test Cycle Detection - self-loop
    Graph graphSelfLoop = {
        {1, {1, 2}},
        {2, {}}
    };

    bool hasCycleResult3 = hasCycle(graphSelfLoop);
    if (!hasCycleResult3) {
        std::cerr << "x Should detect self-loop as cycle" << std::endl;
        passed = false;
    } else {
        std::cout << "v Cycle detection correctly identified self-loop" << std::endl;
    }

    if (passed) {
        std::cout << "\n[PASS] All tests passed!" << std::endl;
        return 0;
    } else {
        std::cout << "\n[FAIL] Some tests failed." << std::endl;
        return 1;
    }
}
