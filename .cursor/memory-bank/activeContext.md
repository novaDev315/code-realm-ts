🚧 Active Context — Code Realm TS (as of initial import)

Current Focus
-------------
Milestone Phases outlined in PRD:
- Phase 1: Game engine & Player class — **Planned**
- Phase 2: Chapter 1 module + recursion checker — **Planned**
- Phase 3: XP system, map, reset logic — **Planned**

Immediate Next Steps
--------------------
1. Scaffold `src/engine` with `Game.ts`, `Player.ts`, `Chapter.ts`, and `ChapterLoader.ts`.
2. Implement Chapter 1 (Forest of Recursion) along with sample grader in `quests/chapter1/`.
3. Build `XPSystem.ts` and basic `WorldMap` generator.
4. Set up Jest test runner for autograding.

Open Considerations
-------------------
• Decide on CLI library (e.g., Inquirer vs. custom IO wrapper).
• Define file format for test cases (JSON vs. TS exports).
• Determine how to persist progress (`xp-tracker.json`) across sessions.

Active Decisions
----------------
✓ Use fully OOP architecture per spec.
✓ Containerised runtime via Docker Compose.
✓ World map stored as markdown for easy view in any environment.
