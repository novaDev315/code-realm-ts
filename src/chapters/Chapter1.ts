import { Chapter } from "../engine/Chapter";

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
    // MVP placeholder – automatically succeed
    return true;
  }
}

export default Chapter1; 