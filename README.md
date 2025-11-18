# Code Realm TS

A gamified command-line RPG for mastering algorithms and system-design in multiple programming languages.

---

## 📜 Overview
Code Realm TS turns your terminal into a fantasy world where each coding challenge is a chapter in an epic journey. Earn XP, level up ranks, and defeat boss encounters while sharpening real-world engineering skills.

* **Languages**: TypeScript, JavaScript, Python (with Go support planned)
* **Architecture**: Fully OOP, chapter-based engine
* **Runtime**: `ts-node` or Docker
* **Progression**: Story Mode (linear) + Manual Mode (replay)
* **Autograding**: Built-in test harness validates solutions instantly
* **Multi-Language**: Choose your preferred language for each chapter!

## 🎮 Gameplay at a Glance
| Rank | XP Required |
|------|-------------|
| Initiate | 0 |
| Apprentice | 100 |
| Adept | 300 |
| Architect | 600 |
| Grandmaster | 1000 |

1. **Story Mode** – Tackle chapters in order; unlock lore and new zones.
2. **Manual Mode** – Replay completed chapters or simulate failure for practice.
3. **Reset** – Start fresh, wiping XP and progress.

## 🧩 Complete Chapter Catalog ✅

### Algorithm Fundamentals (Chapters 1-5)
| # | Zone | Concepts | XP | Languages |
|---|----------------------|--------------------------------------|-----|-----------|
| 1 | Forest of Recursion | Recursion, Backtracking | 100 | TS, JS, PY |
| 2 | Mountains of Order | MergeSort, QuickSort | 150 | TS, JS, PY |
| 3 | Mirror Maze | Sliding Window, Two Pointers | 150 | TS, JS, PY |
| 4 | Stream of Thoughts | Async/Await, Promises | 150 | TS, JS |
| **5** | **Labyrinth of Nodes** ⭐ | **Graphs (DFS, BFS, Paths)** | **200** | **TS, JS, PY** |

**Arc Total:** 750 XP

### Data Structures & Backend (Chapters 6-8)
| # | Zone | Concepts | XP | Languages |
|---|----------------------|--------------------------------------|-----|-----------|
| 6 | Caves of Shadows | Dynamic Programming (Knapsack, LCS) | 150 | TS, JS, PY |
| 7 | Vault of Data | Database Design, SQL Queries | 150 | TS, JS, PY |
| 8 | Realm of APIs | REST Endpoints, GraphQL Parsing | 150 | TS, JS, PY |

**Arc Total:** 450 XP

### DevOps & Security (Chapters 9-10)
| # | Zone | Concepts | XP | Languages |
|---|----------------------|--------------------------------------|-----|-----------|
| 9 | Dungeon of DevOps | Docker, Compose, Health Checks | 150 | TS, JS, PY |
| **10** | **Citadel of Firewalls** ⭐ | **Security, JWT, Rate Limiting** | **200** | **TS, JS, PY** |

**Arc Total:** 350 XP

### System Design Mastery (Chapters 11-15)
| # | Zone | Concepts | XP | Languages |
|---|----------------------|--------------------------------------|-----|-----------|
| 11 | Tower of Constructs | URL Shortener, Base62 Encoding | 150 | TS, JS, PY |
| 12 | Hall of Echoes | Message Queues, Producer-Consumer | 150 | TS, JS, PY |
| 13 | Crystal Socket Chamber | LRU Cache, Eviction Policies | 150 | TS, JS, PY |
| 14 | Gate of Trials | Load Balancing, Circuit Breaker | 150 | TS, JS |
| **15** | **Core of the Architect** ⭐⭐ | **Distributed Systems (FINAL BOSS)** | **300** | **TS, JS, PY** |

**Arc Total:** 900 XP

---

**🎯 TOTAL: 2,450 XP • 15 Complete Chapters • 3 Master Projects**

⭐ = Master Project (extra XP) | ⭐⭐ = Final Boss (highest XP reward!)

## 🌍 Multi-Language Support

Code Realm supports solving challenges in multiple programming languages! When you start a chapter, you'll be prompted to select your preferred language.

### Currently Supported Languages:
- **TypeScript** - Full support with type checking
- **JavaScript** - ES6+ with Node.js runtime
- **Python** - Python 3 with standard library

### Planned Language Support:
- **Go** - Coming soon for system design chapters
- **Rust** - Optional for performance-focused challenges

### Quest Structure (Multi-Language):
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

Each language folder contains:
- **Solution file** - Where you write your implementations
- **Check file** - Autograder that tests your solutions

## 🗂 Directory Structure
```
code-realm-ts/
  src/
    engine/          # Game core (Game, Chapter, Player, Loader)
    chapters/        # Chapter implementations
    utils/           # XPSystem, WorldMap, IO helpers, LanguageRunner
  quests/            # Player-written solutions (multi-language)
  realm/             # xp-tracker.json & world-map.md
  .cursor/           # Memory Bank & rules (auto-generated)
  Dockerfile / docker-compose.yml
  tsconfig.json / package.json
```

## 🚀 Quick Start
### Prerequisites
* Node.js ≥ 20 **or** Docker

### Local (Node)
```bash
# Install dependencies
npm install

# Start game in dev mode
npm run dev           # alias: ts-node src/main.ts
```

### Containerised
```bash
# Build & run interactive container
docker compose up
```

## 🔧 Development
* Each chapter lives in `src/chapters/ChapterN.ts` and extends the abstract `Chapter` class.
* Autograding scripts reside beside player quests (e.g., `quests/chapter1/check.ts`).
* Add new chapters by creating a class, tests, and updating `chapterCatalog.md` – no engine changes required.

### Testing
Jest is planned for the test harness. Configure tests under `__tests__/` or within chapter folders.

Run lint & test:
```bash
npm run lint
npm test
```