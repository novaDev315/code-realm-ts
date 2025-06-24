📜 Code Realm TS — Project Brief

Purpose
-------
Code Realm TS is a gamified CLI role-playing game that teaches algorithms and system-design concepts through interactive challenges. The game blends fantasy storytelling with a chapter-based progression system to motivate continual learning.

Vision & Goals
--------------
1. Deliver an engaging learning experience that makes complex computer-science topics approachable and fun.
2. Provide an extensible OOP codebase so new chapters, quests, and mechanics can be added with minimal effort.
3. Offer automated grading and clear feedback so players can validate their solutions immediately.
4. Track player progress via XP, ranks, and a markdown world map to visualise achievements.

Scope (MVP)
------------
• Fully OOP TypeScript codebase runnable via `ts-node` on Node 20.
• Core game engine, player profile, and abstract chapter interface.
• At least one algorithmic chapter (Forest of Recursion) with autograding.
• Comprehensive system-design chapters beyond the MVP.
• CLI story mode, manual replay mode, and reset functionality.
• Containerised distribution with Docker & Docker Compose.

Success Criteria
----------------
• Players can complete Chapter and see XP/rank updates.
• Adding a new chapter requires only creating a subclass of `Chapter` plus tests.
• The project builds and runs with a single command (`npm start` or `docker compose up`).

Non-Goals
---------
• Graphical UI (reserved for future web UI phase).
• Networked multiplayer features.
