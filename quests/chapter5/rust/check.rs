use std::collections::{HashMap, HashSet, VecDeque};

type Graph = HashMap<i32, Vec<i32>>;

fn dfs(graph: &Graph, start: i32) -> Vec<i32> {
    // TODO: Implement DFS
    vec![]
}

fn bfs(graph: &Graph, start: i32) -> Vec<i32> {
    // TODO: Implement BFS
    vec![]
}

fn shortest_path(graph: &Graph, start: i32, end: i32) -> Vec<i32> {
    // TODO: Implement shortest path
    vec![]
}

fn has_cycle(graph: &Graph) -> bool {
    // TODO: Implement cycle detection
    false
}

fn main() {
    let mut all_passed = true;

    // Create test graphs
    let mut graph1 = Graph::new();
    graph1.insert(0, vec![1, 2]);
    graph1.insert(1, vec![2]);
    graph1.insert(2, vec![3]);
    graph1.insert(3, vec![]);

    let mut graph2 = Graph::new();
    graph2.insert(1, vec![2, 3]);
    graph2.insert(2, vec![1, 3]);
    graph2.insert(3, vec![1, 2]);

    let mut graph3 = Graph::new();
    graph3.insert(0, vec![1]);
    graph3.insert(1, vec![2]);
    graph3.insert(2, vec![0]);

    let mut graph_single = Graph::new();
    graph_single.insert(5, vec![]);

    // Test DFS
    println!("Testing dfs()...");
    let dfs_result = dfs(&graph1, 0);
    if dfs_result.len() == 4 {
        println!("  ✓ dfs(graph1, 0) returned 4 nodes: {:?}", dfs_result);
    } else {
        println!("  ✗ dfs(graph1, 0) returned {:?} (expected 4 nodes)", dfs_result);
        all_passed = false;
    }

    let dfs_single = dfs(&graph_single, 5);
    if dfs_single == vec![5] {
        println!("  ✓ dfs(single_node, 5) = {:?}", dfs_single);
    } else {
        println!("  ✗ dfs(single_node, 5) = {:?} (expected [5])", dfs_single);
        all_passed = false;
    }

    // Test BFS
    println!("\nTesting bfs()...");
    let bfs_result = bfs(&graph1, 0);
    if bfs_result.len() == 4 {
        println!("  ✓ bfs(graph1, 0) returned 4 nodes: {:?}", bfs_result);
    } else {
        println!("  ✗ bfs(graph1, 0) returned {:?} (expected 4 nodes)", bfs_result);
        all_passed = false;
    }

    let bfs_single = bfs(&graph_single, 5);
    if bfs_single == vec![5] {
        println!("  ✓ bfs(single_node, 5) = {:?}", bfs_single);
    } else {
        println!("  ✗ bfs(single_node, 5) = {:?} (expected [5])", bfs_single);
        all_passed = false;
    }

    // Test Shortest Path
    println!("\nTesting shortest_path()...");
    let path = shortest_path(&graph1, 0, 3);
    if path.len() > 0 && path[0] == 0 && path[path.len() - 1] == 3 {
        println!("  ✓ shortest_path(graph1, 0, 3) = {:?}", path);
    } else {
        println!("  ✗ shortest_path(graph1, 0, 3) = {:?} (expected path from 0 to 3)", path);
        all_passed = false;
    }

    let path_same = shortest_path(&graph1, 2, 2);
    if path_same == vec![2] {
        println!("  ✓ shortest_path(graph1, 2, 2) = {:?}", path_same);
    } else {
        println!("  ✗ shortest_path(graph1, 2, 2) = {:?} (expected [2])", path_same);
        all_passed = false;
    }

    // Test Cycle Detection
    println!("\nTesting has_cycle()...");
    if !has_cycle(&graph1) {
        println!("  ✓ has_cycle(acyclic_graph) = false");
    } else {
        println!("  ✗ has_cycle(acyclic_graph) = true (expected false)");
        all_passed = false;
    }

    if has_cycle(&graph3) {
        println!("  ✓ has_cycle(cyclic_graph) = true");
    } else {
        println!("  ✗ has_cycle(cyclic_graph) = false (expected true)");
        all_passed = false;
    }

    if !has_cycle(&graph_single) {
        println!("  ✓ has_cycle(single_node) = false");
    } else {
        println!("  ✗ has_cycle(single_node) = true (expected false)");
        all_passed = false;
    }

    if !has_cycle(&graph2) {
        println!("  ✓ has_cycle(disconnected_cycle) = false or true (valid)");
    } else {
        println!("  ✓ has_cycle(disconnected_cycle) = true");
    }

    if all_passed {
        println!("\n✓ All tests passed!");
        std::process::exit(0);
    } else {
        println!("\n✗ Some tests failed!");
        std::process::exit(1);
    }
}
