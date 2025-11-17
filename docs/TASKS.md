# Code Realm - Task List

**Last Updated:** 2025-11-17
**Focus:** Personal learning tool with multi-language support

---

## 🚨 Critical (Do First)

### Setup & Configuration
- [ ] Run `npm install` to install dependencies
- [ ] Create `.gitignore` file
  ```gitignore
  node_modules/
  dist/
  *.log
  .DS_Store
  .env
  ```
- [ ] Create `.eslintrc.json` configuration
- [ ] Create `LICENSE` file (MIT)
- [ ] Test Chapter 1 works: `./run_docker.sh play`

**Estimated Time:** 30 minutes

---

## 🎯 Priority 1: Multi-Language Support

### Architecture Updates (Week 2)

#### 1. Update Chapter Interface
- [ ] Modify `src/engine/Chapter.ts` to include:
  - `supportedLanguages: string[]` property
  - `run(language: string)` method signature

#### 2. Create Language Runner Utility
- [ ] Create `src/utils/LanguageRunner.ts`
- [ ] Implement language detection and test execution
- [ ] Support TypeScript, JavaScript, Python initially
- [ ] Add exec wrapper for running external commands

#### 3. Update Game Engine
- [ ] Add language selection menu in `src/engine/Game.ts`
- [ ] Show supported languages for each chapter
- [ ] Pass selected language to chapter.run()
- [ ] Store language preference in player data (optional)

#### 4. Update Chapter 1 for Multi-Language
- [ ] Create folder structure:
  ```
  quests/chapter1/
  ├── typescript/
  │   ├── recursion.ts
  │   └── check.ts
  ├── javascript/
  │   ├── recursion.js
  │   └── check.js
  └── python/
      ├── recursion.py
      └── check.py
  ```
- [ ] Port TypeScript solutions to JavaScript
- [ ] Port TypeScript solutions to Python
- [ ] Create test runners for each language
- [ ] Update `src/chapters/Chapter1.ts` to support all languages

#### 5. Update Docker for Multi-Language
- [ ] Add Python to Dockerfile
- [ ] Add Go to Dockerfile (optional for now)
- [ ] Test all languages work in container

**Estimated Time:** 6-8 hours

---

## 🎓 Priority 2: Content Creation

### Algorithm Arc (Month 1)

#### Chapter 2: Mountains of Order (Sorting)
- [ ] Create `src/chapters/Chapter2.ts`
- [ ] Write lore and challenge description
- [ ] Create quest folders for TypeScript/JavaScript/Python
- [ ] Implement MergeSort challenge
- [ ] Implement QuickSort challenge
- [ ] Create test cases for all languages
- [ ] Test chapter completion awards correct XP (150)

**Concepts to cover:**
- MergeSort implementation
- QuickSort implementation
- Time complexity comparison
- Stability in sorting

**Estimated Time:** 4-6 hours

#### Chapter 3: Mirror Maze (Sliding Window, Two Pointers)
- [ ] Create `src/chapters/Chapter3.ts`
- [ ] Write lore and challenge description
- [ ] Create quest folders for TypeScript/JavaScript/Python
- [ ] Implement sliding window challenge (max sum subarray)
- [ ] Implement two pointers challenge (two sum, three sum)
- [ ] Create test cases for all languages
- [ ] Test chapter completion

**Concepts to cover:**
- Sliding window technique
- Two pointers pattern
- Array manipulation
- Optimization from O(n²) to O(n)

**Estimated Time:** 4-6 hours

#### Chapter 4: Stream of Thoughts (Async/Await)
- [ ] Create `src/chapters/Chapter4.ts`
- [ ] Write lore and challenge description
- [ ] Create quest folders for TypeScript/JavaScript (Node-specific)
- [ ] Implement Promise chain challenge
- [ ] Implement async error handling
- [ ] Implement rate limiting with promises
- [ ] Create test cases
- [ ] Test chapter completion

**Concepts to cover:**
- Promises and async/await
- Error handling in async code
- Event loop understanding
- Concurrent operations

**Estimated Time:** 4-6 hours

#### Chapter 5: Labyrinth of Nodes (Graphs) ⭐ Master Project
- [ ] Create `src/chapters/Chapter5.ts`
- [ ] Write lore and challenge description
- [ ] Create quest folders for TypeScript/JavaScript/Python/Go
- [ ] Implement DFS challenge
- [ ] Implement BFS challenge
- [ ] Implement shortest path challenge
- [ ] Implement cycle detection
- [ ] Create test cases for all languages
- [ ] Test chapter completion awards 200 XP

**Concepts to cover:**
- Graph representations (adjacency list, matrix)
- DFS and BFS traversal
- Shortest path algorithms
- Cycle detection
- Topological sort

**Estimated Time:** 6-8 hours

---

### Data Structures & Backend (Month 2)

#### Chapter 6: Caves of Shadows (Dynamic Programming)
- [ ] Create `src/chapters/Chapter6.ts`
- [ ] Implement 0/1 Knapsack challenge
- [ ] Implement Longest Common Subsequence
- [ ] Implement Coin Change problem
- [ ] Create test cases for TypeScript/JavaScript/Python
- [ ] Test chapter completion

**Estimated Time:** 6-8 hours

#### Chapter 7: Vault of Data (Database Design)
- [ ] Create `src/chapters/Chapter7.ts`
- [ ] Set up SQLite in Docker
- [ ] Create schema design challenge
- [ ] Create SQL query challenges
- [ ] Create indexing optimization challenge
- [ ] Test with multiple languages (SQL + app code)

**Estimated Time:** 6-8 hours

#### Chapter 8: Realm of APIs (REST & GraphQL)
- [ ] Create `src/chapters/Chapter8.ts`
- [ ] Create REST API endpoint challenge
- [ ] Create GraphQL resolver challenge
- [ ] Support TypeScript (Express), JavaScript, Python (Flask), Go
- [ ] Test API implementations

**Estimated Time:** 6-8 hours

---

### DevOps & Security (Month 3)

#### Chapter 9: Dungeon of DevOps
- [ ] Create `src/chapters/Chapter9.ts`
- [ ] Create Dockerfile writing challenge
- [ ] Create docker-compose challenge
- [ ] Create health check implementation
- [ ] Test container orchestration

**Estimated Time:** 4-6 hours

#### Chapter 10: Citadel of Firewalls (Security) ⭐ Master Project
- [ ] Create `src/chapters/Chapter10.ts`
- [ ] Implement JWT authentication challenge
- [ ] Implement password hashing (bcrypt)
- [ ] Implement rate limiting middleware
- [ ] Support TypeScript, JavaScript, Python, Go
- [ ] Test security implementations

**Estimated Time:** 6-8 hours

---

### System Design (Month 4)

#### Chapter 11: Tower of Constructs (URL Shortener)
- [ ] Create `src/chapters/Chapter11.ts`
- [ ] Design system architecture challenge
- [ ] Implement hash function
- [ ] Create API endpoints
- [ ] Test full implementation

**Estimated Time:** 6-8 hours

#### Chapter 12: Hall of Echoes (Message Queues)
- [ ] Create `src/chapters/Chapter12.ts`
- [ ] Implement simple message queue
- [ ] Create producer-consumer pattern
- [ ] Test with TypeScript, Python, Go

**Estimated Time:** 6-8 hours

#### Chapter 13: Crystal Socket Chamber (Caching)
- [ ] Create `src/chapters/Chapter13.ts`
- [ ] Implement LRU cache
- [ ] Create cache invalidation strategies
- [ ] Test implementations

**Estimated Time:** 4-6 hours

#### Chapter 14: Gate of Trials (Load Balancing)
- [ ] Create `src/chapters/Chapter14.ts`
- [ ] Implement round-robin load balancer
- [ ] Create rate limiter with sliding window
- [ ] Implement circuit breaker pattern
- [ ] Test with TypeScript and Go

**Estimated Time:** 6-8 hours

#### Chapter 15: Core of the Architect (Final Boss) ⭐
- [ ] Create `src/chapters/Chapter15.ts`
- [ ] Design comprehensive system challenge
- [ ] Combine: algorithms + APIs + caching + auth
- [ ] Support all learned languages
- [ ] Create epic final test

**Estimated Time:** 8-12 hours

---

## 🎨 Quality of Life (Optional)

### UX Improvements
- [ ] Add `chalk` for colored output
  ```bash
  npm install chalk
  ```
- [ ] Colorize rank display (green for Grandmaster, etc.)
- [ ] Colorize success/failure messages
- [ ] Add ASCII art chapter banners (optional)
- [ ] Add progress bar for XP (optional)
  ```bash
  npm install cli-progress
  ```
- [ ] Enhance world map with ASCII art zones

### Better World Map
- [ ] Add emoji icons for each zone
- [ ] Show XP progress bar
- [ ] Show locked vs unlocked zones
- [ ] Add visual map of the realm

**Estimated Time:** 2-4 hours (if desired)

---

## 📚 Documentation (As You Go)

### Update Existing Docs
- [ ] Update README.md with multi-language support info
- [ ] Add language selection instructions
- [ ] Update chapter catalog with completion status
- [ ] Add examples for each language

### Create New Docs
- [ ] Create `docs/LANGUAGE_GUIDE.md` - how to add new language
- [ ] Create `docs/CHAPTER_GUIDE.md` - how to create new chapter
- [ ] Add code examples for solution templates

**Estimated Time:** 1-2 hours total

---

## 📦 Language Support Progression

### Phase 1: Core Languages (Week 2)
- [x] TypeScript (already working)
- [ ] JavaScript (easy - same runtime)
- [ ] Python (popular for algorithms)

### Phase 2: Systems Languages (Month 2)
- [ ] Go (great for system design, concurrency)
- [ ] Rust (optional - for performance challenges)

### Phase 3: Additional (Optional)
- [ ] Java (if needed for specific challenges)
- [ ] C++ (for low-level optimization chapters)

---

## 🎯 Milestones

### Milestone 1: Multi-Language Foundation ✅ Ready
- [x] Core engine working
- [ ] JavaScript support added
- [ ] Python support added
- [ ] Chapter 1 works in all 3 languages

### Milestone 2: Algorithm Mastery
- [ ] Chapters 2-5 complete
- [ ] All algorithm fundamentals covered
- [ ] Multiple language implementations

### Milestone 3: Full Stack Development
- [ ] Chapters 6-8 complete
- [ ] Database, APIs covered
- [ ] Backend skills solid

### Milestone 4: DevOps & Security
- [ ] Chapters 9-10 complete
- [ ] Container orchestration understood
- [ ] Security best practices learned

### Milestone 5: System Design Mastery
- [ ] Chapters 11-14 complete
- [ ] Real-world system patterns understood
- [ ] Scalability concepts mastered

### Milestone 6: Grandmaster Achieved
- [ ] Chapter 15 complete
- [ ] All 15 chapters finished
- [ ] 1000+ XP earned
- [ ] Multiple languages mastered

---

## 📊 Progress Tracking

### Current Status
```
Total XP: 0 / 2950
Rank: Initiate
Chapters Complete: 0 / 15 (0%)
Languages Supported: 1 / 5 (TypeScript only)
```

### Target Status (4 months)
```
Total XP: 2950
Rank: Grandmaster
Chapters Complete: 15 / 15 (100%)
Languages Supported: 5 (TypeScript, JavaScript, Python, Go, Rust)
```

---

## 🚀 Quick Start Guide

### This Week
1. Fix critical setup (npm install, configs)
2. Design multi-language architecture
3. Add JavaScript to Chapter 1
4. Add Python to Chapter 1

### This Month
1. Complete Chapters 2-5 (algorithms)
2. Add Go language support
3. Practice solving in multiple languages

### Next 3 Months
1. Complete Chapters 6-15
2. Add Rust support (optional)
3. Build final project

---

## 💡 Tips for Success

### Learning Strategy
- **Complete each chapter in TypeScript first** (most familiar)
- **Then solve in Python** (learn concise syntax)
- **Then try JavaScript** (understand subtle differences)
- **Finally Go or Rust** (challenge yourself)

### Time Management
- **1 chapter per week** = 15 weeks (~4 months)
- **Block 4-8 hours per chapter**
- **Review solutions in all languages**

### Quality Over Speed
- **Understand concepts deeply**
- **Compare language approaches**
- **Refactor and optimize**
- **Build real understanding, not just XP**

---

**Next Step:** Run `npm install` and let's add multi-language support! 🚀
