# Chapter 1: Forest of Recursion

Welcome to the first challenge in Code Realm! In this chapter, you'll master the art of recursion by implementing three classic recursive algorithms.

## 🎯 Challenge Overview

You must implement three recursive functions to pass the trials of Yggloop, the elder tree:

1. **Fibonacci** - Calculate the nth Fibonacci number
2. **Factorial** - Calculate the factorial of n
3. **String Permutations** - Generate all unique permutations of a string

## 📚 Choose Your Language

This chapter supports multiple programming languages:
- **TypeScript** (`typescript/` folder)
- **JavaScript** (`javascript/` folder)
- **Python** (`python/` folder)

## 🚀 How to Complete the Challenge

### Option 1: Through the Game

1. Start the game: `npm run dev`
2. Select **Story Mode**
3. Choose your preferred language when prompted
4. The game will run your solution and show results

### Option 2: Direct Testing

Test your solution directly without running the game:

**TypeScript:**
```bash
npm run test:grader:ts
```

**JavaScript:**
```bash
npm run test:grader:js
```

**Python:**
```bash
npm run test:grader:py
```

## 📝 Implementation Guide

### TypeScript

Edit `typescript/recursion.ts`:

```typescript
export function fibonacci(n: number): number {
  // Your implementation here
}

export function factorial(n: number): number {
  // Your implementation here
}

export function stringPermutations(str: string): string[] {
  // Your implementation here
}
```

### JavaScript

Edit `javascript/recursion.js`:

```javascript
function fibonacci(n) {
  // Your implementation here
}

function factorial(n) {
  // Your implementation here
}

function stringPermutations(str) {
  // Your implementation here
}

module.exports = { fibonacci, factorial, stringPermutations };
```

### Python

Edit `python/recursion.py`:

```python
def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number."""
    # Your implementation here

def factorial(n: int) -> int:
    """Calculate the factorial of n."""
    # Your implementation here

def string_permutations(s: str) -> list[str]:
    """Generate all unique permutations of a string."""
    # Your implementation here
```

## ✅ Test Cases

Your implementations will be tested against these cases:

### Fibonacci
- `fibonacci(5)` should return `5`
- `fibonacci(8)` should return `21`

### Factorial
- `factorial(5)` should return `120`

### String Permutations
- `stringPermutations("abc")` should return 6 unique permutations
- `stringPermutations("aab")` should return 3 unique permutations

## 💡 Hints

<details>
<summary>Click to reveal hints</summary>

### Fibonacci
- Base cases: n = 0 returns 0, n = 1 returns 1
- Recursive case: fibonacci(n) = fibonacci(n-1) + fibonacci(n-2)

### Factorial
- Base case: n ≤ 1 returns 1
- Recursive case: factorial(n) = n × factorial(n-1)

### String Permutations
- Base cases: empty string returns [''], single char returns [char]
- For each character, find permutations of remaining characters
- Combine the character with each permutation
- Use a Set to track unique permutations (important for duplicates!)

</details>

## 🎓 Reference Solutions

If you get stuck, reference implementations are available in the `SOLUTIONS.*` files:
- `typescript/SOLUTIONS.ts`
- `javascript/SOLUTIONS.js`
- `python/SOLUTIONS.py`

**Note:** Try to solve it yourself first! The learning comes from the struggle.

## 🏆 Rewards

Upon successful completion:
- **XP Gained:** 100
- **Rank Progress:** Towards Apprentice (requires 100 XP total)
- **Unlock:** Chapter 2 - Mountains of Order

## 🐛 Troubleshooting

### Tests are failing
- Check that your function names match exactly (case-sensitive)
- Verify your return types are correct
- Make sure you're handling base cases
- For string permutations, ensure you return unique permutations only

### Import/Module errors
- **TypeScript:** Use `export function`
- **JavaScript:** Use `module.exports = { ... }`
- **Python:** Functions should be at module level (not indented)

### Still stuck?
- Review the hints above
- Check the reference solutions
- Ensure you understand how recursion works (a function calling itself)

## 📚 Learning Resources

- [Recursion Explained](https://en.wikipedia.org/wiki/Recursion_(computer_science))
- [Fibonacci Sequence](https://en.wikipedia.org/wiki/Fibonacci_number)
- [Factorial](https://en.wikipedia.org/wiki/Factorial)
- [Permutations](https://en.wikipedia.org/wiki/Permutation)

---

Good luck, adventurer! May your stack never overflow. 🌲✨
