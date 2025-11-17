# Code Realm Documentation

Welcome to the Code Realm documentation! This directory contains all planning, analysis, and development guides for your personal learning journey.

---

## 📚 Documentation Index

### Planning & Roadmaps
- **[PERSONAL_ROADMAP.md](./PERSONAL_ROADMAP.md)** - Complete development roadmap focused on personal learning with multi-language support
- **[TASKS.md](./TASKS.md)** - Actionable task list organized by priority and timeline

### Project Information
- **[../README.md](../README.md)** - Project overview and quick start guide
- **[../PRD.md](../PRD.md)** - Product requirements and specification

---

## 🎯 Quick Overview

### What is Code Realm?
A gamified CLI RPG for mastering algorithms and system design through hands-on coding challenges in multiple programming languages.

### Current Status
- **Phase:** MVP (Core Engine Complete)
- **Chapters:** 1/15 implemented (6.7%)
- **Languages:** TypeScript only (multi-language support planned)
- **XP System:** ✅ Working
- **Docker Setup:** ✅ Working

### Key Features
- 🎮 Story Mode - Progress through chapters linearly
- 🔁 Manual Mode - Replay completed chapters
- 📈 XP & Rank System - Earn XP, level up (Initiate → Grandmaster)
- 🗺️ World Map - Track your journey
- 🐳 Docker Support - Containerized development

---

## 🚀 Getting Started

### Prerequisites
- Docker (recommended) OR Node.js 20+

### Quick Start
```bash
# Install dependencies
npm install

# Run the game (Docker)
./run_docker.sh play

# Or run locally
npm run dev
```

### Your First Steps
1. Read [PERSONAL_ROADMAP.md](./PERSONAL_ROADMAP.md) for the complete vision
2. Check [TASKS.md](./TASKS.md) for immediate action items
3. Fix critical setup (npm install, .gitignore)
4. Start learning with Chapter 1: Forest of Recursion

---

## 🎓 Learning Path

### Phase 1: Algorithm Fundamentals (Month 1)
**Focus:** Core algorithms in TypeScript, JavaScript, Python

- Chapter 1: Recursion & Backtracking (100 XP) ✅
- Chapter 2: Sorting Algorithms (150 XP)
- Chapter 3: Sliding Window & Two Pointers (150 XP)
- Chapter 4: Async Programming (150 XP)
- Chapter 5: Graph Algorithms (200 XP) ⭐

**Total XP:** 750

### Phase 2: Data Structures & Backend (Month 2)
**Focus:** Advanced algorithms, databases, APIs

- Chapter 6: Dynamic Programming (200 XP)
- Chapter 7: Database Design (200 XP)
- Chapter 8: REST & GraphQL APIs (200 XP)

**Total XP:** 600 (cumulative: 1350)

### Phase 3: DevOps & Security (Month 3)
**Focus:** Infrastructure and security

- Chapter 9: Docker & DevOps (200 XP)
- Chapter 10: Authentication & Security (300 XP) ⭐

**Total XP:** 500 (cumulative: 1850)

### Phase 4: System Design (Month 4)
**Focus:** Real-world system architecture

- Chapter 11: URL Shortener Design (150 XP)
- Chapter 12: Message Queues (200 XP)
- Chapter 13: Caching Systems (200 XP)
- Chapter 14: Load Balancing (250 XP)
- Chapter 15: Final Boss - Complete System (500 XP) ⭐⭐⭐

**Total XP:** 1300 (cumulative: 2950)

### Final Status
- **Rank:** Grandmaster (1000 XP required)
- **Total XP:** 2950
- **Languages Mastered:** 3-5 (TypeScript, JavaScript, Python, Go, Rust)
- **Skills:** Algorithms, Data Structures, APIs, Security, System Design

---

## 🌍 Multi-Language Support

### Supported Languages (Planned)
- ✅ **TypeScript** - Type-safe, modern JavaScript
- ⏳ **JavaScript** - Dynamic, ubiquitous
- ⏳ **Python** - Concise, readable, great for algorithms
- ⏳ **Go** - Fast, concurrent, systems programming
- ⏳ **Rust** - Memory-safe, high-performance (optional)

### Why Multi-Language?
- **Polyglot Skills:** Learn syntax and idioms across languages
- **Compare Approaches:** See different solutions to same problem
- **Career Relevant:** Practice languages used in real jobs
- **Versatility:** Choose best language for each problem type

### Language Usage Guide
- **Algorithms (Ch 1-6):** Python (concise) or TypeScript (typed)
- **Async (Ch 4):** JavaScript/TypeScript (Node.js native)
- **System Design (Ch 7-15):** Go (performance) or TypeScript
- **Performance (Ch 13-14):** Go or Rust

---

## 📂 Project Structure

```
code-realm-ts/
├── src/
│   ├── engine/          # Core game engine
│   │   ├── Game.ts      # Game loop, menus
│   │   ├── Player.ts    # XP, progress tracking
│   │   ├── Chapter.ts   # Abstract chapter interface
│   │   └── ChapterLoader.ts  # Dynamic chapter loading
│   ├── chapters/        # Chapter implementations
│   │   └── Chapter1.ts  # Forest of Recursion
│   └── utils/           # Utility modules
│       ├── XPSystem.ts  # Rank calculation
│       ├── WorldMap.ts  # Progress visualization
│       └── IO.ts        # Input/output helpers
├── quests/              # Your solution area
│   └── chapter1/
│       ├── typescript/
│       │   ├── recursion.ts    # Your solutions
│       │   └── check.ts        # Autograder
│       ├── javascript/  # (planned)
│       └── python/      # (planned)
├── realm/               # Game state (persistent)
│   ├── xp-tracker.json  # Your progress
│   └── world-map.md     # Visual map
├── docs/                # This directory
│   ├── README.md        # Documentation index (this file)
│   ├── PERSONAL_ROADMAP.md  # Development roadmap
│   └── TASKS.md         # Action items
├── Dockerfile           # Container definition
├── docker-compose.yml   # Service orchestration
├── run_docker.sh        # Interactive helper script
└── package.json         # Dependencies
```

---

## 🛠️ Development Workflow

### Adding Multi-Language Support
1. Update `Chapter` interface to support languages
2. Create `LanguageRunner` utility
3. Add language selection to game menu
4. Port Chapter 1 to JavaScript and Python
5. Update Docker for multi-language support

**See:** [PERSONAL_ROADMAP.md#priority-2-multi-language-support](./PERSONAL_ROADMAP.md#priority-2-multi-language-support)

### Creating a New Chapter
1. Create `src/chapters/ChapterN.ts`
2. Extend `Chapter` class with lore, XP, challenges
3. Create quest folders for each language
4. Write solution templates
5. Create test/autograder files
6. Test in Story Mode and Manual Mode

**See:** [PERSONAL_ROADMAP.md#quick-reference-adding-a-new-chapter](./PERSONAL_ROADMAP.md#quick-reference-adding-a-new-chapter)

### Testing Your Solutions
```bash
# Play the game
./run_docker.sh play

# Run specific chapter grader
npm run test:grader

# Run Jest unit tests
npm test
```

---

## 🎯 Design Philosophy

### Personal Tool Principles
✅ **Keep It Simple**
- File-based storage (JSON) - perfectly fine for one user
- No database, no auth, no complex infrastructure
- Manual Docker commands - no CI/CD needed
- Console output - no logging framework

✅ **Focus on Learning**
- Quality challenges over quantity
- Clear explanations and lore
- Gradual difficulty progression
- Real-world applicable skills

✅ **Multi-Language Mastery**
- Solve same problem in different languages
- Compare approaches and idioms
- Build polyglot programming skills
- Choose right tool for the job

❌ **Skip Enterprise Features**
- No multi-user support
- No PostgreSQL/complex databases
- No CI/CD pipelines
- No monitoring/alerting
- No web UI (CLI is better for learning)

---

## 📊 Rank System

| Rank | XP Required | Chapters to Complete |
|------|-------------|---------------------|
| Initiate | 0 | Start here |
| Apprentice | 100 | Chapter 1 |
| Adept | 300 | Chapters 1-2 |
| Architect | 600 | Chapters 1-4 |
| Grandmaster | 1000 | Chapters 1-6+ |

**Max XP Available:** 2950 (all 15 chapters)

---

## 🎮 Gameplay Mechanics

### Story Mode
- Play chapters in sequence
- First completion awards XP
- Unlocks next chapter
- Linear progression

### Manual Mode
- Replay completed chapters
- Practice in different languages
- No XP awarded (already earned)
- Validate chapter unlocked first

### Reset Mode
- Wipe all progress
- Start fresh from Initiate
- Useful for new language playthroughs

---

## 🔧 Configuration

### TypeScript
- Strict mode enabled
- ES2022 target
- CommonJS modules
- Source: `src/`, Output: `dist/`

### Docker
- Base: Node 20 Alpine (minimal)
- Multi-language support (TypeScript, Python, Go)
- Interactive TTY for gameplay
- Volume mounts for live development

### NPM Scripts
```json
{
  "dev": "ts-node src/main.ts",      // Development mode
  "start": "node dist/main.js",      // Production mode
  "build": "tsc",                     // Compile TypeScript
  "test": "jest",                     // Run tests
  "test:grader": "ts-node quests/chapter1/check.ts",  // Manual grader
  "lint": "eslint . --ext .ts"       // Code quality
}
```

---

## 📈 Progress Tracking

### Your Journey

**Starting Point:**
```
Rank: Initiate
XP: 0
Chapters: 0/15
Languages: TypeScript only
```

**Milestone Checkpoints:**
- **Apprentice (100 XP):** Recursion mastered
- **Adept (300 XP):** Sorting algorithms solid
- **Architect (600 XP):** Async programming comfortable
- **Grandmaster (1000 XP):** DP and graphs conquered

**Final Goal:**
```
Rank: Grandmaster
XP: 2950
Chapters: 15/15 (100%)
Languages: TypeScript, JavaScript, Python, Go, Rust
Skills: Full-stack developer with system design expertise
```

---

## 💡 Learning Tips

### Strategy
1. **Complete in TypeScript first** (familiar territory)
2. **Solve in Python** (learn concise syntax)
3. **Try JavaScript** (understand subtle differences)
4. **Challenge with Go/Rust** (systems thinking)

### Time Commitment
- **Per Chapter:** 4-8 hours (including multi-language)
- **Per Week:** 1 chapter
- **Total Timeline:** 15 weeks (~4 months)

### Quality Focus
- Understand concepts deeply
- Compare language approaches
- Refactor and optimize
- Build portfolio-worthy solutions

---

## 🤝 Contributing (To Your Own Learning)

### Best Practices
- Solve problems yourself before checking solutions
- Implement in multiple languages
- Optimize after getting it working
- Document your thought process
- Review and refactor regularly

### Git Workflow
```bash
# After completing a chapter
git add .
git commit -m "feat(chapter2): complete sorting algorithms in TypeScript, Python"

# After adding language support
git commit -m "feat(multi-lang): add Python support to Chapter 1"

# After optimizations
git commit -m "refactor(chapter1): optimize fibonacci with memoization"
```

---

## 📚 Additional Resources

### External Learning
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
- [Python Algorithms](https://docs.python.org/3/library/collections.html)
- [Go by Example](https://gobyexample.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [System Design Primer](https://github.com/donnemartin/system-design-primer)

### Related Projects
- LeetCode - Algorithm practice
- Exercism - Multi-language learning
- CodeSignal - Interview prep
- HackerRank - Competitive programming

---

## 🎉 Milestones to Celebrate

- ✅ Complete first chapter in any language
- ✅ Reach Apprentice rank (100 XP)
- ✅ Solve a chapter in 3 different languages
- ✅ Complete algorithm arc (Chapters 1-5)
- ✅ Reach Grandmaster rank (1000 XP)
- ✅ Complete all 15 core chapters
- ✅ Build a real project from Chapter 15

---

## 🚀 Next Steps

1. **Read** [PERSONAL_ROADMAP.md](./PERSONAL_ROADMAP.md) - Understand the vision
2. **Review** [TASKS.md](./TASKS.md) - See what's next
3. **Run** `npm install` - Set up dependencies
4. **Play** Chapter 1 - Start your journey
5. **Add** Multi-language support - Expand capabilities
6. **Create** Chapter 2 - Build momentum
7. **Master** All 15 chapters - Become a Grandmaster

---

**Ready to begin your journey through the Code Realm? 🗡️**

Start with the critical setup tasks, then embark on your multi-language learning adventure!
