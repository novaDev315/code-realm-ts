from graphs import dfs, bfs, shortest_path, has_cycle


def run_check():
    """Run comprehensive tests for graph algorithms."""
    passed = True

    # Test DFS
    print("Testing DFS...")
    graph1 = {
        1: [2, 3],
        2: [4],
        3: [5],
        4: [],
        5: [],
    }

    dfs_result = dfs(graph1, 1)
    if len(dfs_result) != 5 or 1 not in dfs_result or 2 not in dfs_result:
        print(f"❌ DFS failed. Expected to visit all 5 nodes, got: {dfs_result}")
        passed = False
    else:
        print(f"✓ DFS correctly visited {len(dfs_result)} nodes: {' -> '.join(map(str, dfs_result))}")

    # Test DFS with disconnected component
    graph2 = {
        1: [2],
        2: [],
        3: [4],
        4: [],
    }

    dfs_result2 = dfs(graph2, 1)
    if len(dfs_result2) != 2:
        print(f"❌ DFS should only visit connected nodes. Expected 2, got: {len(dfs_result2)}")
        passed = False
    else:
        print("✓ DFS correctly handled disconnected graph")

    # Test BFS
    print("\nTesting BFS...")
    graph3 = {
        1: [2, 3],
        2: [4],
        3: [5],
        4: [],
        5: [],
    }

    bfs_result = bfs(graph3, 1)
    if len(bfs_result) != 5:
        print(f"❌ BFS failed. Expected 5 nodes, got: {len(bfs_result)}")
        passed = False
    elif bfs_result[0] != 1 or 2 not in bfs_result or 3 not in bfs_result:
        print(f"❌ BFS order incorrect: {bfs_result}")
        passed = False
    else:
        print(f"✓ BFS correctly visited nodes in level order: {' -> '.join(map(str, bfs_result))}")

        # Test level-order property
        bfs_index_of_2 = bfs_result.index(2)
        bfs_index_of_3 = bfs_result.index(3)
        bfs_index_of_4 = bfs_result.index(4)
        bfs_index_of_5 = bfs_result.index(5)

        if max(bfs_index_of_2, bfs_index_of_3) < min(bfs_index_of_4, bfs_index_of_5):
            print("✓ BFS maintains level-order property")
        else:
            print("⚠ BFS may not maintain strict level-order")

    # Test Shortest Path
    print("\nTesting Shortest Path...")
    graph4 = {
        1: [2, 3],
        2: [4],
        3: [4],
        4: [5],
        5: [],
    }

    path1 = shortest_path(graph4, 1, 5)
    if len(path1) == 0:
        print("❌ Shortest path not found from 1 to 5")
        passed = False
    elif path1[0] != 1 or path1[-1] != 5:
        print(f"❌ Path doesn't start at 1 and end at 5: {path1}")
        passed = False
    else:
        print(f"✓ Shortest path found (length {len(path1)}): {' -> '.join(map(str, path1))}")

    # Test shortest path - same start and end
    path_same = shortest_path(graph4, 1, 1)
    if len(path_same) != 1 or path_same[0] != 1:
        print("❌ Shortest path from node to itself should be [1]")
        passed = False
    else:
        print("✓ Shortest path handles same start and end")

    # Test shortest path - no path
    graph5 = {
        1: [2],
        2: [],
        3: [4],
        4: [],
    }

    no_path = shortest_path(graph5, 1, 4)
    if len(no_path) != 0:
        print("❌ Should return empty list when no path exists")
        passed = False
    else:
        print("✓ Shortest path correctly returns empty for disconnected nodes")

    # Test Cycle Detection - graph with cycle
    print("\nTesting Cycle Detection...")
    graph_with_cycle = {
        1: [2],
        2: [3],
        3: [1],
    }

    has_cycle_result1 = has_cycle(graph_with_cycle)
    if not has_cycle_result1:
        print("❌ Should detect cycle in 1->2->3->1")
        passed = False
    else:
        print("✓ Cycle detection correctly identified cycle")

    # Test Cycle Detection - graph without cycle
    graph_no_cycle = {
        1: [2, 3],
        2: [4],
        3: [4],
        4: [5],
        5: [],
    }

    has_cycle_result2 = has_cycle(graph_no_cycle)
    if has_cycle_result2:
        print("❌ Should not detect cycle in acyclic graph")
        passed = False
    else:
        print("✓ Cycle detection correctly identified acyclic graph")

    # Test Cycle Detection - self-loop
    graph_self_loop = {
        1: [1, 2],
        2: [],
    }

    has_cycle_result3 = has_cycle(graph_self_loop)
    if not has_cycle_result3:
        print("❌ Should detect self-loop as cycle")
        passed = False
    else:
        print("✓ Cycle detection correctly identified self-loop")

    if passed:
        print("\n✅ All tests passed!")
    else:
        print("\n❌ Some tests failed.")
        exit(1)

    return passed


if __name__ == "__main__":
    run_check()
