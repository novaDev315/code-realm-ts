use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<i32, Vec<i32>>;

// Depth-First Search traversal
// Time Complexity: O(V + E) where V is vertices and E is edges
// Space Complexity: O(V) for the visited set and recursion stack
fn dfs(graph: &Graph, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    dfs_helper(graph, start, &mut visited, &mut result);
    result
}

fn dfs_helper(graph: &Graph, node: i32, visited: &mut HashSet<i32>, result: &mut Vec<i32>) {
    if visited.contains(&node) {
        return;
    }

    visited.insert(node);
    result.push(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            dfs_helper(graph, neighbor, visited, result);
        }
    }
}

// Breadth-First Search traversal
// Time Complexity: O(V + E)
// Space Complexity: O(V) for the queue and visited set
fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut result = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    result
}

// Find shortest path between two nodes using BFS
// Time Complexity: O(V + E)
// Space Complexity: O(V) for the queue, visited set, and parent map
fn shortest_path(graph: &Graph, start: i32, end: i32) -> Vec<i32> {
    if start == end {
        return vec![start];
    }

    let mut visited = HashSet::new();
    let mut parent: HashMap<i32, i32> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(node) = queue.pop_front() {
        if node == end {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = end;
            while current != start {
                path.push(current);
                current = parent[&current];
            }
            path.push(start);
            path.reverse();
            return path;
        }

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    parent.insert(neighbor, node);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    // No path found
    vec![]
}

// Detect if graph has a cycle using DFS with color marking
// Time Complexity: O(V + E)
// Space Complexity: O(V) for the color map and recursion stack
fn has_cycle(graph: &Graph) -> bool {
    let mut color: HashMap<i32, u8> = HashMap::new();

    // Initialize all nodes as unvisited (0)
    for &node in graph.keys() {
        color.insert(node, 0);
    }

    // Check each node
    for &node in graph.keys() {
        if color[&node] == 0 {
            if has_cycle_helper(graph, node, &mut color) {
                return true;
            }
        }
    }

    false
}

// Helper function for cycle detection using DFS
// Color codes: 0 = white (unvisited), 1 = gray (visiting), 2 = black (visited)
fn has_cycle_helper(graph: &Graph, node: i32, color: &mut HashMap<i32, u8>) -> bool {
    color.insert(node, 1); // Mark as visiting

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if color.get(&neighbor).copied().unwrap_or(0) == 0 {
                // Unvisited
                if has_cycle_helper(graph, neighbor, color) {
                    return true;
                }
            } else if color[&neighbor] == 1 {
                // Back edge found (currently visiting) - cycle detected
                return true;
            }
        }
    }

    color.insert(node, 2); // Mark as visited
    false
}

fn main() {
    println!("=== Graph Solutions ===\n");

    // Create a test graph
    let mut graph = Graph::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![3]);
    graph.insert(3, vec![]);

    println!("Graph: 0 -> [1, 2], 1 -> [2], 2 -> [3], 3 -> []");

    // DFS
    println!("\nDFS Traversal from node 0:");
    let dfs_result = dfs(&graph, 0);
    println!("  Result: {:?}", dfs_result);

    // BFS
    println!("\nBFS Traversal from node 0:");
    let bfs_result = bfs(&graph, 0);
    println!("  Result: {:?}", bfs_result);

    // Shortest Path
    println!("\nShortest Path from 0 to 3:");
    let path = shortest_path(&graph, 0, 3);
    println!("  Path: {:?}", path);

    // Cycle Detection
    println!("\nCycle Detection:");
    println!("  Has cycle: {}", has_cycle(&graph));

    // Test with cyclic graph
    let mut cyclic_graph = Graph::new();
    cyclic_graph.insert(0, vec![1]);
    cyclic_graph.insert(1, vec![2]);
    cyclic_graph.insert(2, vec![0]);

    println!("\nCyclic Graph: 0 -> [1], 1 -> [2], 2 -> [0]");
    println!("  Has cycle: {}", has_cycle(&cyclic_graph));
}
