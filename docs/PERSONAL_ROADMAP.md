# Code Realm - Personal Learning Roadmap

**Last Updated:** 2025-11-17
**Focus:** Personal single-user learning tool with multi-language support

---

## Current Status

### ✅ What's Working
- Core game engine (Story/Manual/Reset modes)
- XP and rank system
- Chapter 1: Forest of Recursion (TypeScript)
- Docker development environment
- File-based progress tracking

### ⚠️ Immediate Needs
- `npm install` (node_modules empty)
- `.gitignore` file
- 14 more core chapters
- Multi-language support

### 📊 Progress
- **Chapters:** 1/15 core chapters (6.7%)
- **Languages:** 1 (TypeScript only)
- **Code Quality:** Solid OOP foundation

---

## Priority 1: Essential Setup (Week 1)

### Critical Fixes

#### 1. Install Dependencies
```bash
npm install
```

#### 2. Create .gitignore
```gitignore
node_modules/
dist/
*.log
.DS_Store
.env
```

#### 3. Create ESLint Config (.eslintrc.json)
```json
{
  "parser": "@typescript-eslint/parser",
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended"
  ],
  "rules": {
    "@typescript-eslint/no-explicit-any": "warn",
    "@typescript-eslint/explicit-function-return-type": "off"
  }
}
```

#### 4. Create LICENSE (MIT)
Standard MIT license text

---

## Priority 2: Multi-Language Support (Week 2)

### Architecture Design

**Goal:** Allow solving challenges in TypeScript, JavaScript, Python, Go, Rust, Java, etc.

### Implementation Strategy

#### Option A: Language-Specific Quest Folders (Recommended)
```
quests/
├── chapter1/
│   ├── typescript/
│   │   ├── recursion.ts
│   │   └── check.ts
│   ├── python/
│   │   ├── recursion.py
│   │   └── check.py
│   ├── javascript/
│   │   ├── recursion.js
│   │   └── check.js
│   ├── go/
│   │   ├── recursion.go
│   │   └── check.go
│   └── rust/
│       ├── recursion.rs
│       └── check.rs
```

**Pros:**
- Clean separation
- Language-specific test runners
- Easy to add new languages
- Native tooling for each language

**Implementation Steps:**

1. **Update Chapter Interface**
```typescript
// src/engine/Chapter.ts
abstract class Chapter {
  abstract id: number;
  abstract title: string;
  abstract xpReward: number;
  abstract lore: string;
  abstract supportedLanguages: string[]; // ['typescript', 'python', 'javascript']

  abstract run(language: string): boolean | Promise<boolean>;
}
```

2. **Add Language Selection to Game Menu**
```typescript
// In Game.ts
private async selectLanguage(chapter: Chapter): Promise<string> {
  console.log("\nAvailable languages:");
  chapter.supportedLanguages.forEach((lang, i) => {
    console.log(`  ${i + 1}. ${lang}`);
  });
  // Get user choice
  return selectedLanguage;
}
```

3. **Language-Specific Test Runners**
```typescript
// src/utils/LanguageRunner.ts
class LanguageRunner {
  static async run(language: string, chapterId: number): Promise<boolean> {
    switch(language) {
      case 'typescript':
        return execSync(`ts-node quests/chapter${chapterId}/typescript/check.ts`);
      case 'python':
        return execSync(`python quests/chapter${chapterId}/python/check.py`);
      case 'javascript':
        return execSync(`node quests/chapter${chapterId}/javascript/check.js`);
      case 'go':
        return execSync(`go run quests/chapter${chapterId}/go/check.go`);
      case 'rust':
        return execSync(`cargo test --manifest-path quests/chapter${chapterId}/rust/Cargo.toml`);
      default:
        throw new Error(`Unsupported language: ${language}`);
    }
  }
}
```

4. **Docker Updates for Multi-Language**
```dockerfile
# Dockerfile
FROM node:20-alpine

# Install Python
RUN apk add --no-cache python3 py3-pip

# Install Go
RUN apk add --no-cache go

# Install Rust (optional - large image)
# RUN apk add --no-cache rust cargo

# Install Java (optional)
# RUN apk add --no-cache openjdk17-jre

WORKDIR /app
# ... rest of Dockerfile
```

### Phase 1: Core Languages (Week 2)
- [ ] TypeScript (already working)
- [ ] JavaScript (easy - Node.js already installed)
- [ ] Python (very popular for algorithms)

### Phase 2: Systems Languages (Week 3-4)
- [ ] Go (great for system design chapters)
- [ ] Rust (performance-focused chapters)

### Phase 3: Additional Languages (Optional)
- [ ] Java
- [ ] C++
- [ ] C#

---

## Priority 3: Content Creation (Months 1-3)

### Algorithm Arc (Chapters 2-5) - Month 1

#### Chapter 2: Mountains of Order (150 XP)
**Concepts:** MergeSort, QuickSort, Comparison-based sorting
**Challenges:**
- Implement MergeSort
- Implement QuickSort
- Sort with constraints (stability, in-place)

**Languages:** TypeScript, JavaScript, Python

#### Chapter 3: Mirror Maze (150 XP)
**Concepts:** Sliding Window, Two Pointers
**Challenges:**
- Maximum sum subarray (sliding window)
- Two sum, three sum (two pointers)
- Container with most water

**Languages:** TypeScript, JavaScript, Python

#### Chapter 4: Stream of Thoughts (150 XP)
**Concepts:** Async/Await, Promises, Event Loop
**Challenges:**
- Promise chain handling
- Async error handling
- Rate limiting with promises
- Event loop understanding

**Languages:** TypeScript, JavaScript (Node-specific)

#### Chapter 5: Labyrinth of Nodes (200 XP) ⭐ Master Project
**Concepts:** DFS, BFS, Graph representations
**Challenges:**
- Implement DFS and BFS
- Find shortest path
- Detect cycles
- Topological sort

**Languages:** TypeScript, JavaScript, Python, Go

### Data Structures & DP (Chapters 6-7) - Month 2

#### Chapter 6: Caves of Shadows (200 XP)
**Concepts:** Dynamic Programming (Knapsack, Paths, LCS)
**Challenges:**
- 0/1 Knapsack
- Longest common subsequence
- Coin change problem
- Grid path counting

**Languages:** TypeScript, JavaScript, Python

#### Chapter 7: Vault of Data (200 XP)
**Concepts:** Database design, SQL, Indexing
**Challenges:**
- Design schema for social network
- Write complex SQL queries
- Optimize with indexes
- Transaction handling

**Languages:** SQL (with any language driver)
**Tools:** SQLite (lightweight, file-based)

### Backend & APIs (Chapter 8) - Month 2

#### Chapter 8: Realm of APIs (200 XP)
**Concepts:** REST APIs, GraphQL, HTTP
**Challenges:**
- Build REST API endpoints
- Implement GraphQL resolver
- API versioning
- Error handling and status codes

**Languages:** TypeScript (Express), JavaScript (Express), Python (Flask), Go (net/http)

### DevOps & Containers (Chapter 9) - Month 3

#### Chapter 9: Dungeon of DevOps (200 XP)
**Concepts:** Docker, docker-compose, CI concepts
**Challenges:**
- Write Dockerfile for multi-stage build
- Create docker-compose setup
- Implement health checks
- Container networking

**Languages:** Dockerfile, YAML, shell scripts

### Security (Chapter 10) - Month 3

#### Chapter 10: Citadel of Firewalls (300 XP) ⭐ Master Project
**Concepts:** JWT, OAuth2, Password hashing, Rate limiting
**Challenges:**
- Implement JWT authentication
- Password hashing (bcrypt)
- Rate limiting middleware
- CORS and security headers

**Languages:** TypeScript, JavaScript, Python, Go

### System Design (Chapters 11-14) - Months 3-4

#### Chapter 11: Tower of Constructs (150 XP)
**Concepts:** URL shortener design
**Challenges:**
- Hash function design
- Collision handling
- Database schema
- API implementation

**Languages:** Any (design + implementation)

#### Chapter 12: Hall of Echoes (200 XP)
**Concepts:** Message queues, pub/sub patterns
**Challenges:**
- Implement simple message queue
- Producer-consumer pattern
- Message persistence
- Dead letter queue

**Languages:** TypeScript, Python, Go

#### Chapter 13: Crystal Socket Chamber (200 XP)
**Concepts:** Caching, LRU cache, Redis patterns
**Challenges:**
- Implement LRU cache
- Cache invalidation strategies
- Write-through vs write-back
- Redis integration

**Languages:** TypeScript, JavaScript, Python, Go

#### Chapter 14: Gate of Trials (250 XP)
**Concepts:** Load balancing, rate limiting, circuit breakers
**Challenges:**
- Implement round-robin load balancer
- Sliding window rate limiter
- Circuit breaker pattern
- Health check monitoring

**Languages:** TypeScript, Go

### Final Boss (Chapter 15) - Month 4

#### Chapter 15: Core of the Architect (500 XP) ⭐ FINAL BOSS
**Concepts:** Everything combined
**Challenges:**
- Design and implement complete system
- Combines: algorithms + data structures + API + caching + auth
- Real-world application (e.g., Twitter-like service)
- Multi-language implementation challenge

**Languages:** All learned languages

---

## Priority 4: Quality of Life Improvements

### Enhanced UX (Optional)

#### Add Colors (Simple)
```bash
npm install chalk
```

```typescript
import chalk from 'chalk';

console.log(chalk.green('✅ Chapter Complete!'));
console.log(chalk.yellow('⚠️ Challenge Failed'));
console.log(chalk.blue(`🎮 Current Rank: ${rank}`));
```

#### Better World Map (Optional)
Add ASCII art for zones:
```markdown
# Code Realm — World Map

🏰 Current Zone: Forest of Recursion
Rank: Apprentice ⭐⭐
XP: 150 / 300 to Adept

[===========>---------] 50%

📍 Completed Zones:
  ✅ Forest of Recursion (100 XP)

🔒 Locked Zones:
  🔒 Mountains of Order (Requires: Apprentice)
  🔒 Mirror Maze (Requires: Adept)
```

#### Progress Bars (Optional)
```bash
npm install cli-progress
```

Show XP progress toward next rank.

---

## Simplified Feature Priorities

### ✅ Keep Simple (Personal Use)
- File-based storage (JSON) - perfectly fine
- Single user - no auth needed
- Manual Docker commands - no CI/CD needed
- Console logging - no structured logging needed
- No backup - git history is enough
- No monitoring - you'll know if it breaks

### ❌ Skip Enterprise Features
- ~~Multi-user support~~
- ~~Database (PostgreSQL)~~
- ~~CI/CD pipelines~~
- ~~Monitoring/alerting~~
- ~~Logging systems~~
- ~~Backup strategies~~
- ~~Scalability concerns~~
- ~~Web UI~~ (CLI is better for learning)

### ⭐ Focus On
1. **Content:** More chapters (15 core chapters)
2. **Languages:** Multi-language support (TypeScript, JS, Python, Go, Rust)
3. **Learning:** Quality challenges and explanations
4. **UX:** Simple, clean, fast feedback loop

---

## Implementation Timeline

### Week 1: Foundation
- [x] Core engine working
- [ ] Fix critical issues (npm install, .gitignore, etc.)
- [ ] Test Chapter 1 thoroughly

### Week 2: Multi-Language Foundation
- [ ] Design language-agnostic architecture
- [ ] Add JavaScript support (easiest first)
- [ ] Add Python support
- [ ] Update Chapter 1 for all 3 languages

### Month 1: Algorithm Chapters
- [ ] Chapter 2: Sorting (TypeScript, JS, Python)
- [ ] Chapter 3: Two Pointers (TypeScript, JS, Python)
- [ ] Chapter 4: Async (TypeScript, JS only)
- [ ] Chapter 5: Graphs (TypeScript, JS, Python, Go)

### Month 2: Data & APIs
- [ ] Chapter 6: Dynamic Programming (TypeScript, JS, Python)
- [ ] Chapter 7: Database Design (SQL + any language)
- [ ] Chapter 8: REST APIs (TypeScript, JS, Python, Go)
- [ ] Add Go language support

### Month 3: DevOps & Security
- [ ] Chapter 9: Docker & DevOps (Dockerfile, YAML)
- [ ] Chapter 10: Security (TypeScript, JS, Python, Go)
- [ ] Add Rust language support (optional)

### Month 4: System Design & Boss
- [ ] Chapter 11-14: System Design (language-agnostic + implementations)
- [ ] Chapter 15: Final Boss (all languages)
- [ ] Polish and refinement

---

## Language Support Matrix

| Chapter | TypeScript | JavaScript | Python | Go | Rust | SQL |
|---------|-----------|------------|--------|----|----- |-----|
| 1 - Recursion | ✅ | ⏳ | ⏳ | - | - | - |
| 2 - Sorting | ⏳ | ⏳ | ⏳ | - | - | - |
| 3 - Two Pointers | ⏳ | ⏳ | ⏳ | - | - | - |
| 4 - Async | ⏳ | ⏳ | - | - | - | - |
| 5 - Graphs | ⏳ | ⏳ | ⏳ | ⏳ | - | - |
| 6 - DP | ⏳ | ⏳ | ⏳ | - | - | - |
| 7 - Database | ⏳ | ⏳ | ⏳ | ⏳ | - | ✅ |
| 8 - APIs | ⏳ | ⏳ | ⏳ | ⏳ | - | - |
| 9 - DevOps | N/A | N/A | N/A | N/A | N/A | - |
| 10 - Security | ⏳ | ⏳ | ⏳ | ⏳ | - | - |
| 11 - URL Shortener | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ | - |
| 12 - Queues | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ | - |
| 13 - Caching | ⏳ | ⏳ | ⏳ | ⏳ | - | - |
| 14 - Load Balancing | ⏳ | ⏳ | - | ⏳ | ⏳ | - |
| 15 - Final Boss | ⏳ | ⏳ | ⏳ | ⏳ | ⏳ | - |

**Legend:**
- ✅ Implemented
- ⏳ Planned
- `-` Not applicable/suitable

---

## Development Philosophy

### Keep It Simple
- **Storage:** JSON files work perfectly for personal use
- **Testing:** Manual testing is fine, add Jest tests only for complex logic
- **Documentation:** Code comments + README is enough
- **Setup:** Docker handles all dependencies, no complex CI needed

### Focus on Learning
- Quality over quantity
- Clear explanations in lore
- Real-world applicable challenges
- Gradual difficulty curve

### Multi-Language Benefits
- **Polyglot Skills:** Learn syntax across languages
- **Compare Approaches:** See how different languages solve same problem
- **Career Relevant:** Practice languages you use at work
- **Versatility:** Choose language based on problem type
  - Algorithms: Python (concise), TypeScript (typed)
  - System Design: Go (performance), TypeScript (familiarity)
  - Low-level: Rust (memory safety)

---

## Quick Reference: Adding a New Chapter

### 1. Create Chapter Class
```typescript
// src/chapters/Chapter2.ts
export class Chapter2 extends Chapter {
  id = 2;
  title = "Mountains of Order";
  xpReward = 150;
  supportedLanguages = ['typescript', 'javascript', 'python'];
  lore = `🏔️ The Mountains of Order...`;

  async run(language: string): Promise<boolean> {
    return LanguageRunner.run(language, this.id);
  }
}
```

### 2. Create Quest Folders
```bash
mkdir -p quests/chapter2/{typescript,javascript,python}
```

### 3. Create Solution Templates
```typescript
// quests/chapter2/typescript/sorting.ts
export function mergeSort(arr: number[]): number[] {
  // TODO: Implement MergeSort
  return arr;
}
```

### 4. Create Test Files
```typescript
// quests/chapter2/typescript/check.ts
import { mergeSort } from './sorting';

const testCases = [
  { input: [3,1,4,1,5], expected: [1,1,3,4,5] },
  // ... more tests
];

// Run tests and return boolean
```

### 5. Test Locally
```bash
./run_docker.sh play
# Select Manual Mode
# Choose Chapter 2
# Choose language
```

---

## Next Immediate Steps

1. **Fix critical setup** (5 minutes)
   ```bash
   npm install
   # Create .gitignore
   # Create .eslintrc.json
   ```

2. **Design multi-language architecture** (2 hours)
   - Update Chapter interface
   - Create LanguageRunner utility
   - Update Game.ts for language selection

3. **Add JavaScript support to Chapter 1** (30 minutes)
   - Copy TypeScript solutions to JavaScript
   - Create JavaScript test runner
   - Test both languages work

4. **Add Python support to Chapter 1** (1 hour)
   - Translate solutions to Python
   - Create Python test runner
   - Update Dockerfile for Python

5. **Create Chapter 2** (4 hours)
   - Implement in TypeScript
   - Port to JavaScript and Python
   - Write tests for all 3 languages

---

## Success Metrics (Personal Goals)

- [ ] Complete all 15 core chapters
- [ ] Support at least 3 languages (TypeScript, JavaScript, Python)
- [ ] Add 2 systems languages (Go, Rust)
- [ ] Solve each challenge in multiple languages
- [ ] Reach Grandmaster rank (1000+ XP)
- [ ] Build at least one real project from Chapter 15

**Timeline:** 4 months to complete all chapters

**Learning Outcome:** Master algorithms, system design, and multiple programming languages through hands-on practice in a fun, gamified environment.

---

**Ready to start!** Focus on content and learning, not infrastructure. Keep it simple, keep it personal, keep it effective.
