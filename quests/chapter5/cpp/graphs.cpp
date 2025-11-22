// Chapter 5: Labyrinth of Nodes - Graph Algorithms
// Your task: Implement the four core graph traversal and analysis algorithms

#include <vector>
#include <map>
#include <set>
#include <queue>

using Graph = std::map<int, std::vector<int>>;

/**
 * Depth-First Search (DFS)
 * Traverse the graph starting from a given node, exploring as far as
 * possible along each branch before backtracking.
 *
 * TODO: Implement using recursion or a stack
 * - Keep track of visited nodes to avoid cycles
 * - Visit neighbors in order (as they appear in the adjacency list)
 * - Return nodes in the order they were first visited
 * - Time complexity: O(V + E) where V = vertices, E = edges
 * - Space complexity: O(V) for visited set and recursion stack
 *
 * @param graph Adjacency list representation of the graph
 * @param start Starting node for traversal
 * @return Vector of nodes in DFS visit order
 */
std::vector<int> dfs(Graph& graph, int start) {
    // TODO: Implement depth-first search
    // Hint: Use recursion with a visited set, or use an explicit stack
    return {};
}

/**
 * Breadth-First Search (BFS)
 * Traverse the graph starting from a given node, exploring all neighbors
 * at the current depth before moving to the next level.
 *
 * TODO: Implement using a queue
 * - Keep track of visited nodes to avoid cycles
 * - Visit neighbors in order (as they appear in the adjacency list)
 * - Return nodes in the order they were first visited (level by level)
 * - Time complexity: O(V + E) where V = vertices, E = edges
 * - Space complexity: O(V) for visited set and queue
 *
 * @param graph Adjacency list representation of the graph
 * @param start Starting node for traversal
 * @return Vector of nodes in BFS visit order
 */
std::vector<int> bfs(Graph& graph, int start) {
    // TODO: Implement breadth-first search
    // Hint: Use a queue to process nodes level by level
    return {};
}

/**
 * Shortest Path (Unweighted Graph)
 * Find the shortest path between two nodes using BFS.
 * In an unweighted graph, BFS guarantees the shortest path.
 *
 * TODO: Implement using BFS with path tracking
 * - Track the path to each node as you traverse
 * - Return the path as soon as you reach the end node
 * - If start equals end, return a vector containing just that node
 * - If no path exists, return an empty vector
 * - Time complexity: O(V + E)
 * - Space complexity: O(V) for visited set, queue, and path storage
 *
 * @param graph Adjacency list representation of the graph
 * @param start Starting node
 * @param end Target node
 * @return Vector of nodes representing the shortest path, or empty if no path exists
 */
std::vector<int> shortestPath(Graph& graph, int start, int end) {
    // TODO: Implement shortest path using BFS
    // Hint: Store the path along with each node in the queue
    return {};
}

/**
 * Cycle Detection
 * Detect if a directed graph contains a cycle using DFS.
 *
 * TODO: Implement using DFS with recursion stack tracking
 * - Use a visited set to track all visited nodes
 * - Use a recursion stack to track nodes in the current DFS path
 * - A cycle exists if we encounter a node that's in the current recursion stack
 * - Check all nodes to handle disconnected components
 * - Time complexity: O(V + E)
 * - Space complexity: O(V) for visited set and recursion stack
 *
 * @param graph Adjacency list representation of the graph
 * @return true if the graph contains a cycle, false otherwise
 */
bool hasCycle(Graph& graph) {
    // TODO: Implement cycle detection using DFS
    // Hint: Track both visited nodes and nodes in current recursion path
    return false;
}
