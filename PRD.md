📜 Code Realm TS — Project Specification
A gamified CLI coding RPG to master algorithms & system design using TypeScript, built with Object-Oriented Principles.

🧱 Overview
Title: Code Realm TS

Type: CLI Role-Playing Coding Game

Language: TypeScript

Architecture: Fully OOP (Object-Oriented Programming)

Runtime: Node.js (ts-node)

Containerized: Docker + Docker Compose

Progression: Chapter-based system

Features: XP system, story mode, manual mode, rank tracking, fantasy visuals (optional)

🧠 Design Philosophy
Encapsulation: All chapters, players, and quests are self-contained classes

Extensibility: Easily add new chapters or features

Reusability: Game engine and checker logic use polymorphic patterns

Separation of Concerns: Distinct responsibilities for game engine, chapter, grader, and player data

🧩 Features
Feature	Description
🎮 Story Mode	Solve challenges to progress through chapters
🔁 Manual Mode	Replay unlocked chapters, simulate failure
🧼 Reset Mode	Start over from the beginning
🧪 Autograding	Validate user code with test cases in each chapter
📈 XP & Rank System	Earn XP, level up ranks as chapters are completed
📜 Lore System	Each chapter has a backstory, hints, and location
🗺️ World Map	Markdown-based visual progress map

🗂 Directory Structure
bash
Copy
Edit
code-realm-ts/
├── src/
│   ├── engine/               # Core game engine
│   │   ├── Game.ts           # Game controller class
│   │   ├── Player.ts         # XP, rank, progress
│   │   ├── Chapter.ts        # Abstract class/interface
│   │   └── ChapterLoader.ts  # Dynamically loads chapters
│   ├── chapters/
│   │   ├── Chapter1.ts       # Forest of Recursion
│   │   ├── Chapter2.ts       # Mountains of Order
│   │   └── ... up to Chapter12
│   └── utils/
│       ├── XPSystem.ts       # XP, ranks, leveling
│       ├── WorldMap.ts       # Markdown map generator
│       └── IO.ts             # Input/output wrapper
├── quests/                   # Player-written solutions
│   └── chapter1/
│       └── recursion.ts
├── realm/
│   ├── xp-tracker.json
│   └── world-map.md
├── Dockerfile
├── docker-compose.yml
├── tsconfig.json
├── package.json
└── README.md
🎭 Game Loop Overview (OOP)
ts
Copy
Edit
// main.ts (simplified)
const game = new Game();
game.start();
Class Structure:
Game: Main entry point, handles game state & menu

Player: Tracks name, XP, rank, completed chapters

Chapter: Abstract class with check(), getLore(), getXP()

Chapter1, Chapter2, etc: Extend Chapter, implement unique logic

WorldMap: Updates world-map.md

XPSystem: Calculates level/rank based on XP

ChapterLoader: Dynamically loads current/next chapter

IO: Handles user interaction & CLI

📘 Chapter Breakdown
📚 Chapter Structure
Each chapter:

Has a fantasy theme

Teaches 1–3 algorithmic/system design concepts

Offers a challenge with test cases

Grants XP and unlocks new rank/zones

🔥 Chapters 1–12
#	Zone	Concepts	Type	XP
1	Forest of Recursion	Recursion, Backtracking	Algorithm	100
2	Mountains of Order	MergeSort, QuickSort	Algorithm	150
3	Caves of Shadows	Dynamic Programming (Knapsack, Paths)	Algorithm	200
4	Labyrinth of Nodes	Graphs: DFS, BFS, Adjacency Lists	Algorithm	200
5	Mirror Maze	Sliding Window, Two Pointers	Algorithm	150
6	Temple of Time	Big-O, Complexity Optimization	Theory	100
7	Tower of Constructs	URL Shortener Design	System Design	150
8	Hall of Echoes	Notification Queue Design (Kafka)	System Design	200
9	Crystal Socket Chamber	Caching Systems (LRU, Redis)	System Design	200
10	Sky Citadel Nexus	Chat App Architecture	System Design	250
11	Gate of Trials	Load Balancing, Rate Limiting	System Design	250
12	Core of the Architect	Final Boss: Design + Algo Fusion	Master Project	500

🧠 XP + Rank System
ts
Copy
Edit
const RANKS = [
  { name: 'Initiate', xp: 0 },
  { name: 'Apprentice', xp: 100 },
  { name: 'Adept', xp: 300 },
  { name: 'Architect', xp: 600 },
  { name: 'Grandmaster', xp: 1000 }
];
📜 Sample Chapter1.ts (OOP)
ts
Copy
Edit
import { Chapter } from "../engine/Chapter";
import { runCheck } from "../../quests/chapter1/check";

export class Chapter1 extends Chapter {
  id = 1;
  title = "Forest of Recursion";
  xpReward = 100;
  lore = `
  🌲 Deep in the Forest of Recursion lies the elder tree Yggloop.
  It speaks only in repeating patterns. Solve its trials:
  - fibonacci(n)
  - factorial(n)
  - stringPermutations(str)
  `;

  run(): boolean {
    return runCheck();
  }
}
🧼 Reset System
ts
Copy
Edit
class Player {
  reset() {
    this.xp = 0;
    this.rank = "Initiate";
    this.chaptersCompleted = [];
    saveProgress(this);
    WorldMap.update(this);
  }
}
🧪 Sample Test Case Format
json
Copy
Edit
{
  "fibonacci": [
    { "input": 5, "output": 5 },
    { "input": 8, "output": 21 }
  ],
  "factorial": [
    { "input": 5, "output": 120 }
  ]
}
🐳 Docker Support
Dockerfile
Dockerfile
Copy
Edit
FROM node:20-alpine
WORKDIR /app
COPY . .
RUN npm install -g ts-node typescript && npm install
CMD ["ts-node", "src/main.ts"]
docker-compose.yml
yaml
Copy
Edit
version: '3'
services:
  code-realm:
    build: .
    volumes:
      - .:/app
    stdin_open: true
    tty: true
🚀 Development Plan (Milestones)
Phase	Task	Status
1	Game engine + Player class	✅ Planned
2	Chapter1 module + recursion checker	✅ Planned
3	XP system, map, and reset logic	✅ Planned
4	Chapter loader + chapter2/3 content	⏳ Next
5	System Design chapters 7–12	🕐 After core gameplay
6	Optional: Web UI (React + WebSocket)	🔒 Future

✅ Deliverables
 src/engine/Game.ts – core engine class

 src/engine/Chapter.ts – base class interface

 src/chapters/Chapter1.ts – sample challenge

 quests/ folder – user solution area

 check.ts scripts – auto-evaluators

 xp-tracker.json + world-map.md

 Docker support

 README.md + LICENSE