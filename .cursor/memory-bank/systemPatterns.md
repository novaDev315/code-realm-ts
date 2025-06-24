🏛️ System Patterns — Code Realm TS

Architecture Overview
----------------------
```
src/
  engine/
    Game.ts         // orchestrates game loop
    Player.ts       // player data & XP tracking
    Chapter.ts      // abstract base for all chapters
    ChapterLoader.ts// dynamic import of chapter classes
  chapters/         // concrete ChapterN.ts implementations
  utils/            // XPSystem, WorldMap, IO helpers
quests/             // user-submitted solutions per chapter
realm/              // world-map.md & xp-tracker.json
```

Key Design Principles
---------------------
• **Encapsulation** — each chapter, player, and quest exists as a discrete class/module.
• **Polymorphism** — chapters extend `Chapter` and override `run()` for their unique logic.
• **Separation of Concerns** — engine handles flow, graders validate, utils manage IO/XPlogic.
• **Extensibility** — new chapters simply add a class + tests; engine auto-loads via `ChapterLoader`.

Important Components & Relationships
------------------------------------
1. **Game** interacts with **Player** and current **Chapter** via methods:
   • `start()` → main menu (Story, Manual, Reset)
   • `playChapter(chapterId)` → invokes `Chapter.run()`
   • awards XP then updates `WorldMap` and saves progress.
2. **ChapterLoader** scans `src/chapters` and returns instances based on player progress.
3. **XPSystem** converts accumulated XP to rank using threshold table.
4. **WorldMap** updates `realm/world-map.md` after chapter completion.
5. **Quests Folder** hosts the player's TypeScript solutions; each chapter's grader imports from here.

Patterns & Techniques
---------------------
• **Factory Method** — `ChapterLoader` acts as a factory for chapter instances.
• **Strategy** — graders implement a strategy for validating user functions per chapter.
• **Observer/Listener** — future enhancement: emit events (level-up, chapter-completed).

Runtime & Deployment
--------------------
• Executed via `ts-node` for rapid dev iterations.
• Dockerfile bundles Node 20-alpine, installs deps, and runs `src/main.ts`.
• Docker Compose provides an interactive TTY service named `code-realm`.
