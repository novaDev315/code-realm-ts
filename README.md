# Code Realm TS

A gamified command-line RPG for mastering algorithms and system-design in TypeScript.

---

## 📜 Overview
Code Realm TS turns your terminal into a fantasy world where each coding challenge is a chapter in an epic journey. Earn XP, level up ranks, and defeat boss encounters while sharpening real-world engineering skills.

* **Language**: TypeScript (Node 20)
* **Architecture**: Fully OOP, chapter-based engine
* **Runtime**: `ts-node` or Docker
* **Progression**: Story Mode (linear) + Manual Mode (replay)
* **Autograding**: Built-in test harness validates solutions instantly

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

## 🧩 Chapter Catalog (Core)
| # | Zone | Concepts | XP |
|---|-----------------------|--------------------------------------|----|
| 1 | Forest of Recursion | Recursion, Backtracking | 100 |
| 2 | Mountains of Order | MergeSort, QuickSort | 150 |
| 3 | Mirror Maze | Sliding Window, Two Pointers | 150 |
| 4 | Stream of Thoughts | Async/Await, Event Loop | 150 |
| **5** | **Labyrinth of Nodes** | **Graphs (DFS, BFS)** | **200** |
| … | _See `memory-bank/chapterCatalog.md` for full list_ | | |

Boss encounters appear every 5 levels culminating in **Core of the Architect** (Chapter 15).

## 🗂 Directory Structure
```
code-realm-ts/
  src/
    engine/          # Game core (Game, Chapter, Player, Loader)
    chapters/        # Chapter implementations
    utils/           # XPSystem, WorldMap, IO helpers
  quests/            # Player-written solutions
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