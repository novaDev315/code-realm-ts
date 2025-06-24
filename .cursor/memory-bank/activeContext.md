🚧 Active Context — Code Realm TS (after Milestone 1)

Current Focus
-------------
Milestone Phases:
- Phase 1: Game engine & Player class — **Completed**
- Phase 2: Chapter 1 module + recursion checker — **Active**
- Phase 3: XP system, map, reset logic — **Completed**

Immediate Next Steps
--------------------
1. Implement real autograder for Chapter 1 (recursion functions) in `quests/chapter1/check.ts`.
2. Write Jest-based test harness and integrate with grader.
3. Expand `WorldMap` visuals and ensure updates after chapter completion.
4. Add Chapter 2 and Chapter 3 skeletons once autograder is proven.

Open Considerations
-------------------
• Choose assertion library for graders (Jest vs. custom).
• Finalise JSON vs. TS test case format.
• Persist settings/config per player.

Active Decisions
----------------
✓ OOP architecture remains core.
✓ Docker Compose workflow validated.
✓ Non-interactive exit handling added to support automated testing.
