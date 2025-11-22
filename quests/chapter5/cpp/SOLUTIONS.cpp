// Reference solutions for Chapter 5: Labyrinth of Nodes - Graph Algorithms

#include <vector>
#include <map>
#include <set>
#include <queue>

using Graph = std::map<int, std::vector<int>>;

// Helper function for DFS recursion
void dfsHelper(Graph& graph, int node, std::set<int>& visited, std::vector<int>& result) {
    if (visited.count(node)) {
        return;
    }

    visited.insert(node);
    result.push_back(node);

    if (graph.count(node)) {
        for (int neighbor : graph[node]) {
            dfsHelper(graph, neighbor, visited, result);
        }
    }
}

/**
 * Depth-First Search (DFS) - SOLUTION
 * Uses recursion with a visited set to track explored nodes.
 */
std::vector<int> dfs(Graph& graph, int start) {
    std::set<int> visited;
    std::vector<int> result;

    dfsHelper(graph, start, visited, result);
    return result;
}

/**
 * Breadth-First Search (BFS) - SOLUTION
 * Uses a queue to process nodes level by level.
 */
std::vector<int> bfs(Graph& graph, int start) {
    std::set<int> visited;
    std::queue<int> queue;
    std::vector<int> result;

    visited.insert(start);
    queue.push(start);

    while (!queue.empty()) {
        int node = queue.front();
        queue.pop();
        result.push_back(node);

        if (graph.count(node)) {
            for (int neighbor : graph[node]) {
                if (!visited.count(neighbor)) {
                    visited.insert(neighbor);
                    queue.push(neighbor);
                }
            }
        }
    }

    return result;
}

/**
 * Shortest Path (Unweighted Graph) - SOLUTION
 * Uses BFS with path tracking to find the shortest path.
 */
std::vector<int> shortestPath(Graph& graph, int start, int end) {
    if (start == end) {
        return {start};
    }

    std::set<int> visited;
    std::queue<std::pair<int, std::vector<int>>> queue;

    visited.insert(start);
    queue.push({start, {start}});

    while (!queue.empty()) {
        auto [node, path] = queue.front();
        queue.pop();

        if (graph.count(node)) {
            for (int neighbor : graph[node]) {
                if (neighbor == end) {
                    std::vector<int> newPath = path;
                    newPath.push_back(end);
                    return newPath;
                }

                if (!visited.count(neighbor)) {
                    visited.insert(neighbor);
                    std::vector<int> newPath = path;
                    newPath.push_back(neighbor);
                    queue.push({neighbor, newPath});
                }
            }
        }
    }

    return {};
}

// Helper function for cycle detection DFS
bool hasCycleHelper(Graph& graph, int node, std::set<int>& visited, std::set<int>& recursionStack) {
    visited.insert(node);
    recursionStack.insert(node);

    if (graph.count(node)) {
        for (int neighbor : graph[node]) {
            if (!visited.count(neighbor)) {
                if (hasCycleHelper(graph, neighbor, visited, recursionStack)) {
                    return true;
                }
            } else if (recursionStack.count(neighbor)) {
                return true;
            }
        }
    }

    recursionStack.erase(node);
    return false;
}

/**
 * Cycle Detection - SOLUTION
 * Uses DFS with a recursion stack to detect back edges.
 */
bool hasCycle(Graph& graph) {
    std::set<int> visited;
    std::set<int> recursionStack;

    for (const auto& [node, _] : graph) {
        if (!visited.count(node)) {
            if (hasCycleHelper(graph, node, visited, recursionStack)) {
                return true;
            }
        }
    }

    return false;
}
