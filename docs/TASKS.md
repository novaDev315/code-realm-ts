# Code Realm - Task List

**Last Updated:** 2025-11-18
**Focus:** Personal learning tool with multi-language support

---

## 🚨 Critical (Do First) ✅ COMPLETED

### Setup & Configuration
- [x] Run `npm install` to install dependencies
- [x] Create `.gitignore` file (includes Python cache exclusions)
- [x] Create `.eslintrc.json` configuration
- [x] Create `LICENSE` file (MIT)
- [x] Test Chapter 1 works: All tests passing

**Estimated Time:** 30 minutes ✅ Done

---

## 🎯 Priority 1: Multi-Language Support ✅ COMPLETED

### Architecture Updates

#### 1. Update Chapter Interface ✅
- [x] Modify `src/engine/Chapter.ts` to include:
  - `supportedLanguages: string[]` property
  - `run(language: string)` method signature

#### 2. Create Language Runner Utility ✅
- [x] Create `src/utils/LanguageRunner.ts`
- [x] Implement language detection and test execution
- [x] Support TypeScript, JavaScript, Python initially
- [x] Add exec wrapper for running external commands

#### 3. Update Game Engine ✅
- [x] Add language selection menu in `src/engine/Game.ts`
- [x] Show supported languages for each chapter
- [x] Pass selected language to chapter.run()

#### 4. Update Chapter 1 for Multi-Language ✅
- [x] Create folder structure for all three languages
- [x] Port TypeScript solutions to JavaScript
- [x] Port TypeScript solutions to Python
- [x] Create test runners for each language
- [x] Update `src/chapters/Chapter1.ts` to support all languages
- [x] Add reference SOLUTIONS files for all languages

#### 5. Update Docker for Multi-Language ✅
- [x] Add Python to Dockerfile
- [x] Test all languages work

**Estimated Time:** 6-8 hours ✅ Done

---

## 🎓 Priority 2: Content Creation ✅ COMPLETED

### Algorithm Arc (Chapters 2-5)

#### Chapter 2: Mountains of Order (Sorting) ✅
- [x] Create `src/chapters/Chapter2.ts`
- [x] Write lore and challenge description
- [x] Create quest folders for TypeScript/JavaScript/Python
- [x] Implement MergeSort challenge
- [x] Implement QuickSort challenge
- [x] Create test cases for all languages
- [x] Add reference SOLUTIONS for all languages
- [x] Test chapter completion awards correct XP (150)

**Concepts covered:**
- MergeSort implementation (divide and conquer)
- QuickSort implementation (pivot partitioning)
- Time complexity O(n log n)
- Handling edge cases (empty, single element, duplicates)

**Estimated Time:** 4-6 hours ✅ Done

#### Chapter 3: Mirror Maze (Sliding Window, Two Pointers) ✅
- [x] Create `src/chapters/Chapter3.ts`
- [x] Write lore and challenge description
- [x] Create quest folders for TypeScript/JavaScript/Python
- [x] Implement sliding window challenge (max sum subarray)
- [x] Implement two pointers challenge (two sum, three sum)
- [x] Create test cases for all languages
- [x] Add reference SOLUTIONS for all languages
- [x] Test chapter completion awards XP (150)

**Concepts covered:**
- Sliding window technique (O(n) for subarray problems)
- Two pointers pattern (sorted array optimization)
- Array manipulation
- Deduplication in three sum

**Estimated Time:** 4-6 hours ✅ Done

#### Chapter 4: Stream of Thoughts (Async/Await) ✅
- [x] Create `src/chapters/Chapter4.ts`
- [x] Write lore and challenge description
- [x] Create quest folders for TypeScript/JavaScript (Node-specific)
- [x] Implement delayed sum challenge
- [x] Implement parallel fetch challenge
- [x] Implement retry operation with exponential backoff
- [x] Create test cases for both languages
- [x] Add reference SOLUTIONS
- [x] Test chapter completion awards XP (150)

**Concepts covered:**
- Promises and async/await
- Parallel execution with Promise.all()
- Error handling in async code
- Retry logic and exponential backoff

**Estimated Time:** 4-6 hours ✅ Done

#### Chapter 5: Labyrinth of Nodes (Graphs) ⭐ Master Project ✅
- [x] Create `src/chapters/Chapter5.ts`
- [x] Write epic lore and challenge description
- [x] Create quest folders for TypeScript/JavaScript/Python
- [x] Implement DFS challenge (depth-first search)
- [x] Implement BFS challenge (breadth-first search)
- [x] Implement shortest path challenge (BFS-based)
- [x] Implement cycle detection (DFS with recursion stack)
- [x] Create comprehensive test cases for all languages
- [x] Add reference SOLUTIONS for all languages
- [x] Test chapter completion awards 200 XP (Master Project!)

**Concepts covered:**
- Graph representations (Map/dict adjacency list)
- DFS and BFS traversal
- Shortest path using BFS
- Cycle detection with visited/recursion tracking
- Edge cases (disconnected graphs, self-loops)

**Estimated Time:** 6-8 hours ✅ Done

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

### Milestone 1: Multi-Language Foundation ✅ ACHIEVED
- [x] Core engine working
- [x] JavaScript support added
- [x] Python support added
- [x] Chapter 1 works in all 3 languages
- [x] Documentation complete (LANGUAGE_GUIDE.md)

### Milestone 2: Algorithm Mastery ✅ ACHIEVED
- [x] Chapters 2-5 complete
- [x] All algorithm fundamentals covered
- [x] Multiple language implementations
- [x] Reference solutions for all chapters

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
Total XP Available: 750 / 2950 (Chapters 1-5 implemented)
Rank: Up to Architect (600 XP) achievable
Chapters Complete: 5 / 15 (33%)
Languages Supported: 3 / 5 (TypeScript, JavaScript, Python)
  - Chapter 1: TS, JS, Python
  - Chapter 2: TS, JS, Python
  - Chapter 3: TS, JS, Python
  - Chapter 4: TS, JS (async-specific)
  - Chapter 5: TS, JS, Python (Master Project!)
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

### Ready to Play! ✅

The game is fully set up with 5 chapters ready to play:

```bash
# Install and run
npm install
npm run dev

# Or use Docker
docker compose up
```

### Test Individual Chapters

```bash
# Chapter 1 - Forest of Recursion
npm run test:ch1:ts   # TypeScript
npm run test:ch1:js   # JavaScript
npm run test:ch1:py   # Python

# Chapter 2 - Mountains of Order (Sorting)
npm run test:ch2:ts
npm run test:ch2:js
npm run test:ch2:py

# Chapter 3 - Mirror Maze (Sliding Window)
npm run test:ch3:ts
npm run test:ch3:js
npm run test:ch3:py

# Chapter 4 - Stream of Thoughts (Async)
npm run test:ch4:ts
npm run test:ch4:js

# Chapter 5 - Labyrinth of Nodes (Graphs) ⭐
npm run test:ch5:ts
npm run test:ch5:js
npm run test:ch5:py
```

### What's Next?

**For Players:**
1. Start with Chapter 1 and work through Chapter 5
2. Try solving in different languages
3. Study the SOLUTIONS files to learn optimal approaches
4. Aim for Architect rank (600 XP from first 5 chapters!)

**For Developers:**
1. Implement Chapters 6-15 (see Priority 3+ sections)
2. Add Go language support
3. Enhance testing and CI/CD
4. Build community features

---

## 💡 Tips for Success

### Learning Strategy
- **Complete each chapter in TypeScript first** (strongly typed)
- **Then solve in Python** (learn concise syntax)
- **Then try JavaScript** (understand subtle differences)
- **Study reference solutions** to learn best practices

### Time Management
- **1 chapter per session** = ~2-4 hours per chapter
- **Review solutions in all languages**
- **Take breaks between chapters**

### Quality Over Speed
- **Understand concepts deeply** before moving on
- **Compare language approaches** to see idioms
- **Refactor and optimize** your solutions
- **Build real understanding, not just XP**

---

**Current Achievement:** 🎉 Chapters 1-5 complete with full multi-language support!
