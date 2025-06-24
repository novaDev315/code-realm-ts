📜 Chapter Catalog — Code Realm TS

Core Chapters (1–15)
--------------------
| #  | Zone                    | Concepts/Challenges                                  | Type            | XP  |
|----|-------------------------|------------------------------------------------------|-----------------|-----|
| 1  | Forest of Recursion     | Recursion, Backtracking                              | Algorithm       | 100 |
| 2  | Mountains of Order      | MergeSort, QuickSort                                 | Algorithm       | 150 |
| 3  | Mirror Maze             | Sliding Window, Two Pointers                         | Algorithm       | 150 |
| 4  | Stream of Thoughts      | Promises, Async/Await, Event Loop                    | Async           | 150 |
| 5  | Labyrinth of Nodes      | DFS, BFS, Adjacency Lists                            | Master Project  | 200 |
| 6  | Caves of Shadows        | Dynamic Programming (Knapsack, Paths)                | Algorithm       | 200 |
| 7  | Vault of Data           | Schema Design, Indexing, Joins                       | Database        | 200 |
| 8  | Realm of APIs           | REST, GraphQL, Caching Layers                        | Backend         | 200 |
| 9  | Dungeon of DevOps       | Dockerfiles, CI/CD, K8s Manifests                    | DevOps          | 200 |
| 10 | Citadel of Firewalls    | JWT, OAuth2, Password Hashing, Rate Limiting         | Master Project  | 300 |
| 11 | Tower of Constructs     | URL Shortener Design                                 | System Design   | 150 |
| 12 | Hall of Echoes          | Notification Queue Design (Kafka)                    | System Design   | 200 |
| 13 | Crystal Socket Chamber  | Caching Systems (LRU, Redis)                         | System Design   | 200 |
| 14 | Gate of Trials          | Load Balancing, Rate Limiting                        | System Design   | 250 |
| 15 | Core of the Architect   | Final Boss: Design + Algo Fusion                     | Master Project  | 500 |


Optional Bonus Chapters (16–30)
--------------------------------
| #  | Zone                     | Concepts/Challenges                                      | Type             | XP  |
|----|--------------------------|----------------------------------------------------------|------------------|-----|
| 16 | Web of Connections       | DFS, BFS, Topological Sort, Shortest Path (Dijkstra, A*) | Algorithm        | 200 |
| 17 | The Mind Forge           | A* Search, Minimax, Decision Trees, Heuristics           | Algorithm        | 250 |
| 18 | Arcade of Logic          | Collision Detection, Game Loops, Tile Maps, Pathfinding  | Master Project   | 300 |
| 19 | Chain of Trust           | Hashing, Merkle Trees, Signature Validation, Consensus   | Security         | 250 |
| 20 | Sky Citadel Nexus        | Chat App Architecture                                    | Master Project   | 300 |
| 21 | Clocktower of Threads    | Mutex, Deadlock, Thread Pooling, Async Queues            | Concurrency      | 250 |
| 22 | Arena of Algorithms      | Binary Search, DP, Trie, Sliding Window (rapid fire)     | Algorithm        | 300 |
| 23 | Shadow of the Compiler   | Lexing, Parsing, AST, Token Streams                      | Compilers        | 300 |
| 24 | Chamber of Language      | Tokenization, Word Embeddings, Text Classification       | NLP              | 200 |
| 25 | Oracle of Prediction     | Moving Average, Exponential Smoothing, Seasonal Models   | Master Project   | 300 |
| 26 | Mirrors of Optimization  | Bit Manipulation, Memory Trade-offs, Profiling           | Optimization     | 250 |
| 27 | Tome of File Systems     | FAT vs NTFS, Paging, Disk Caching, File Trees            | OS               | 200 |
| 28 | Nexus of Intelligence    | KNN, Linear Regression, Decision Trees                   | Machine Learning | 250 |
| 29 | Realm of APIs (Advanced) | GraphQL Federation, gRPC, API Gateways                   | Backend          | 250 |
| 30 | Echoes of Enlightenment  | Fusion of graph, DP, design, async, and AI elements      | Master Project   | 500 |


Chapter Design Notes
--------------------
• Every chapter contains:
  - An immersive lore description tied to its zone.
  - One to three coding challenges with associated test cases in JSON.
  - A dedicated autograder (`check.ts`) that validates user solutions.
  - XP reward that feeds into the rank system.
  - Replay support via Manual Mode once unlocked.

• Completing a chapter updates `xp-tracker.json`, the markdown world map, and may unlock subsequent zones.

• Master Project chapters (e.g., #12, #30) integrate multiple disciplines for a capstone experience. 