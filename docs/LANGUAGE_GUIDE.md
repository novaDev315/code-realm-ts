# Language Support Guide

This guide explains how to add new programming languages to Code Realm and how the multi-language system works.

## Overview

Code Realm supports multiple programming languages for solving challenges. Each chapter can specify which languages it supports, and players can choose their preferred language when playing.

## Currently Supported Languages

| Language | Extension | Runtime | Status |
|----------|-----------|---------|--------|
| TypeScript | `.ts` | ts-node | ✅ Full support |
| JavaScript | `.js` | node | ✅ Full support |
| Python | `.py` | python3 | ✅ Full support |
| Go | `.go` | go run | 🚧 Planned |

## Architecture

### Core Components

1. **LanguageRunner** (`src/utils/LanguageRunner.ts`)
   - Central utility for managing multi-language support
   - Handles language detection, configuration, and test execution
   - Provides helper methods for file paths and validation

2. **Chapter Interface** (`src/engine/Chapter.ts`)
   - `supportedLanguages: string[]` - Lists available languages
   - `run(language?: string)` - Executes challenge in specified language

3. **Game Engine** (`src/engine/Game.ts`)
   - Displays language selection menu
   - Passes selected language to chapter execution

### File Structure

Each chapter's quests are organized by language:

```
quests/chapter{N}/
├── typescript/
│   ├── solution.ts      # Player's implementation
│   └── check.ts         # Autograder/test file
├── javascript/
│   ├── solution.js
│   └── check.js
└── python/
    ├── solution.py
    └── check.py
```

## Adding a New Language

### Step 1: Update LanguageRunner Configuration

Edit `src/utils/LanguageRunner.ts` and add your language to the `SUPPORTED_LANGUAGES` object:

```typescript
export const SUPPORTED_LANGUAGES: Record<string, LanguageConfig> = {
  // ... existing languages ...

  rust: {
    name: "Rust",
    extensions: [".rs"],
    testCommand: (filePath: string) => `rustc ${filePath} && ./check`,
    checkCommand: (filePath: string) => `rustc ${filePath} && ./check`,
  },
};
```

### Step 2: Update Dockerfile

Add runtime dependencies to `Dockerfile`:

```dockerfile
# Install Rust
RUN apk add --no-cache rust cargo
```

### Step 3: Create Language Folder for Chapters

For each chapter that should support the new language:

```bash
mkdir -p quests/chapter1/rust
```

### Step 4: Create Solution and Check Files

**Solution file** (`quests/chapter1/rust/recursion.rs`):
```rust
// Player solutions will live here
pub fn fibonacci(n: u32) -> u32 {
    // placeholder
    0
}

pub fn factorial(n: u32) -> u32 {
    // placeholder
    1
}
```

**Check file** (`quests/chapter1/rust/check.rs`):
```rust
mod recursion;

fn main() {
    let mut passed = true;

    // Test fibonacci
    if recursion::fibonacci(5) != 5 {
        eprintln!("fibonacci(5) failed");
        passed = false;
    }

    if passed {
        println!("✅ All tests passed!");
    } else {
        println!("❌ Some tests failed.");
        std::process::exit(1);
    }
}
```

### Step 5: Update Chapter Class

Edit the chapter to include the new language:

```typescript
export class Chapter1 extends Chapter {
  // ... existing properties ...
  supportedLanguages = ["typescript", "javascript", "python", "rust"];

  // run() method uses LanguageRunner automatically
}
```

### Step 6: Add npm Scripts (Optional)

Update `package.json` for convenience:

```json
{
  "scripts": {
    "test:grader:rust": "cd quests/chapter1/rust && rustc check.rs && ./check"
  }
}
```

## Language Implementation Guidelines

### Test File Requirements

Every language's check file must:

1. **Import/require the solution file**
2. **Run all test cases** with consistent inputs/outputs
3. **Print clear error messages** for failures
4. **Exit with code 1 on failure** (for CI/CD)
5. **Print success message** when all tests pass

### Test Output Format

Use consistent output formatting across languages:

```
✅ All tests passed!        # Success
❌ Some tests failed.       # Failure
fibonacci(5) expected 5 but got 0  # Error details
```

### Solution File Template

Each solution file should:

1. Include placeholder implementations
2. Have clear function signatures
3. Include comments explaining the challenge
4. Use language-idiomatic naming (camelCase vs snake_case)

## Testing Your Language Support

### 1. Test the Check File Directly

```bash
# TypeScript
npm run test:grader:ts

# JavaScript
npm run test:grader:js

# Python
npm run test:grader:py
```

### 2. Test Through the Game Engine

```bash
npm run dev
# Select Story Mode -> Choose your new language
```

### 3. Test with Working Solutions

Create reference solutions to verify the autograder works:

```bash
# Edit the solution file with correct implementations
# Run the checker to verify it passes
```

## Common Patterns by Language

### TypeScript/JavaScript

```typescript
// Module exports
export function fibonacci(n: number): number { }

// Or CommonJS
module.exports = { fibonacci };
```

### Python

```python
# Function definition
def fibonacci(n: int) -> int:
    pass

# Main guard
if __name__ == "__main__":
    run_check()
```

### Go

```go
// Package and function
package main

func Fibonacci(n int) int {
    return 0
}

func main() {
    // Run tests
}
```

## Best Practices

1. **Keep test cases identical** across all languages
2. **Use language-specific conventions** (naming, style)
3. **Handle edge cases** consistently
4. **Provide helpful error messages**
5. **Test on multiple platforms** (especially with Docker)

## Troubleshooting

### Issue: Language not appearing in menu

**Solution:** Check that:
- Language is in `SUPPORTED_LANGUAGES`
- Chapter includes it in `supportedLanguages` array
- Language folder exists in `quests/chapter{N}/`

### Issue: Tests not running

**Solution:** Verify:
- Runtime is installed in Dockerfile
- File permissions are correct (especially for compiled languages)
- Command in `LanguageConfig` is correct

### Issue: Different results across languages

**Solution:** Ensure:
- Test cases are identical
- Edge cases are handled consistently
- Numeric types match expectations (int vs float)

## Example: Complete Language Implementation

See `quests/chapter1/` for complete examples of TypeScript, JavaScript, and Python implementations with:
- Solution templates
- Check files
- Test cases
- Error handling

## Next Steps

After adding a new language:

1. ✅ Test the implementation thoroughly
2. ✅ Update documentation (README.md, this guide)
3. ✅ Add to CI/CD pipeline if applicable
4. ✅ Create issues for future chapters to support it
5. ✅ Share examples with the community

## Contributing

When contributing new language support:

1. Follow this guide's structure
2. Include all necessary runtime dependencies
3. Test on both local and Docker environments
4. Update all relevant documentation
5. Provide at least one working example chapter

---

**Questions?** Check the main README.md or see existing implementations in `quests/chapter1/`.
