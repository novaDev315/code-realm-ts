import { fibonacci, factorial, stringPermutations } from "./recursion";

interface Case<I, O> {
  input: I;
  expect: O;
}

export function runCheck(): boolean {
  let passed = true;

  const fibCases: Case<number, number>[] = [
    { input: 5, expect: 5 },
    { input: 8, expect: 21 }
  ];

  for (const c of fibCases) {
    const result = fibonacci(c.input);
    if (result !== c.expect) {
      console.error(`fibonacci(${c.input}) expected ${c.expect} but got ${result}`);
      passed = false;
    }
  }

  const factCases: Case<number, number>[] = [{ input: 5, expect: 120 }];
  for (const c of factCases) {
    const result = factorial(c.input);
    if (result !== c.expect) {
      console.error(`factorial(${c.input}) expected ${c.expect} but got ${result}`);
      passed = false;
    }
  }

  const permCases: Case<string, number>[] = [
    { input: "abc", expect: 6 },
    { input: "aab", expect: 3 }
  ];
  for (const c of permCases) {
    const result = stringPermutations(c.input);
    if (result.length !== c.expect) {
      console.error(
        `stringPermutations('${c.input}') expected ${c.expect} permutations but got ${result.length}`
      );
      passed = false;
    }
  }

  return passed;
} 