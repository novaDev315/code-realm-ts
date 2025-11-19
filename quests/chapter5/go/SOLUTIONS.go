package main

// DFSSolution performs depth-first search
func DFSSolution(graph Graph, start int) []int {
	var result []int
	visited := make(map[int]bool)

	var dfsHelper func(int)
	dfsHelper = func(node int) {
		if visited[node] {
			return
		}
		visited[node] = true
		result = append(result, node)

		for _, neighbor := range graph[node] {
			if !visited[neighbor] {
				dfsHelper(neighbor)
			}
		}
	}

	dfsHelper(start)
	return result
}

// BFSSolution performs breadth-first search
func BFSSolution(graph Graph, start int) []int {
	var result []int
	visited := make(map[int]bool)
	queue := []int{start}
	visited[start] = true

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		result = append(result, node)

		for _, neighbor := range graph[node] {
			if !visited[neighbor] {
				visited[neighbor] = true
				queue = append(queue, neighbor)
			}
		}
	}

	return result
}

// ShortestPathSolution finds shortest path between two nodes
func ShortestPathSolution(graph Graph, start int, end int) []int {
	if start == end {
		return []int{start}
	}

	visited := make(map[int]bool)
	parent := make(map[int]int)
	queue := []int{start}
	visited[start] = true
	parent[start] = -1

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]

		if node == end {
			// Reconstruct path
			var path []int
			current := end
			for current != -1 {
				path = append([]int{current}, path...)
				if current == start {
					break
				}
				current = parent[current]
			}
			return path
		}

		for _, neighbor := range graph[node] {
			if !visited[neighbor] {
				visited[neighbor] = true
				parent[neighbor] = node
				queue = append(queue, neighbor)
			}
		}
	}

	// No path found
	return []int{}
}

// HasCycleSolution detects if graph has a cycle
func HasCycleSolution(graph Graph) bool {
	visited := make(map[int]bool)
	recStack := make(map[int]bool)

	var hasCycleDFS func(int) bool
	hasCycleDFS = func(node int) bool {
		visited[node] = true
		recStack[node] = true

		for _, neighbor := range graph[node] {
			if !visited[neighbor] {
				if hasCycleDFS(neighbor) {
					return true
				}
			} else if recStack[neighbor] {
				return true
			}
		}

		recStack[node] = false
		return false
	}

	// Check each unvisited node
	for node := range graph {
		if !visited[node] {
			if hasCycleDFS(node) {
				return true
			}
		}
	}

	return false
}
