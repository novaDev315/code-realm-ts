🛠️ Tech Context — Code Realm TS

Languages & Frameworks
----------------------
• TypeScript (strict mode) running on Node.js 20
• `ts-node` for direct TS execution during development

Key Dependencies
----------------
| Package | Purpose |
|---------|---------|
| typescript | Type definitions & compiler |
| ts-node | Run TS files without pre-compilation |
| jest (planned) | Unit testing & autograding harness |

Tooling & Infrastructure
------------------------
• **Docker** — `Dockerfile` builds minimal alpine image including dependencies.
• **Docker Compose** — defines service `code-realm` with TTY interactive session.
• **Git** — project source control; follow commit guidelines.
• **VS Code + Remote Containers** — recommended IDE config for consistent env.

Directory Structure Snapshot
----------------------------
```
code-realm-ts/
  src/engine/           # Game core classes
  src/chapters/         # Chapter implementations
  src/utils/            # XPSystem, WorldMap, IO
  quests/               # Player code submissions
  realm/                # Progress + map artifacts
  Dockerfile & docker-compose.yml
  tsconfig.json & package.json
```

Development Setup
-----------------
1. Clone repo and `cd code-realm-ts`.
2. Install dependencies: `npm install`.
3. Run in dev mode: `npm run dev` (alias for `ts-node src/main.ts`).
4. Alternatively: `docker compose up` to run in container.

Technical Constraints
---------------------
• Must maintain full OOP style; functional helpers allowed but core logic in classes.
• Keep chapter loading dynamic — no hard-coded import list.
• Ensure cross-platform compatibility (Windows/macOS/Linux via Node or Docker).
