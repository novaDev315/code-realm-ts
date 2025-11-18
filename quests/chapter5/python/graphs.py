# Reference solutions for Chapter 5: Labyrinth of Nodes


def dfs(graph: dict[int, list[int]], start: int) -> list[int]:
    """Perform depth-first search and return visited nodes in order."""
    visited = set()
    result = []

    def traverse(node):
        if node in visited:
            return
        visited.add(node)
        result.append(node)

        for neighbor in graph.get(node, []):
            traverse(neighbor)

    traverse(start)
    return result


def bfs(graph: dict[int, list[int]], start: int) -> list[int]:
    """Perform breadth-first search and return visited nodes in order."""
    visited = set()
    queue = [start]
    result = []

    visited.add(start)

    while queue:
        node = queue.pop(0)
        result.append(node)

        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)

    return result


def shortest_path(graph: dict[int, list[int]], start: int, end: int) -> list[int]:
    """Find shortest path between start and end using BFS.

    Return path as list of nodes, or empty if no path exists.
    """
    if start == end:
        return [start]

    visited = set()
    queue = [{"node": start, "path": [start]}]

    visited.add(start)

    while queue:
        current = queue.pop(0)
        node = current["node"]
        path = current["path"]

        for neighbor in graph.get(node, []):
            if neighbor == end:
                return path + [end]

            if neighbor not in visited:
                visited.add(neighbor)
                queue.append({"node": neighbor, "path": path + [neighbor]})

    return []


def has_cycle(graph: dict[int, list[int]]) -> bool:
    """Detect if graph has a cycle using DFS."""
    visited = set()
    recursion_stack = set()

    def dfs_helper(node):
        visited.add(node)
        recursion_stack.add(node)

        for neighbor in graph.get(node, []):
            if neighbor not in visited:
                if dfs_helper(neighbor):
                    return True
            elif neighbor in recursion_stack:
                return True

        recursion_stack.discard(node)
        return False

    for node in graph.keys():
        if node not in visited:
            if dfs_helper(node):
                return True

    return False
