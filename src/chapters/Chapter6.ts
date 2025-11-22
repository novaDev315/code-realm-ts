import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter6 extends Chapter {
  id = 6;
  title = "Caves of Shadows";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
In the depths of the Caves of Shadows, you discover ancient tablets inscribed with the secrets of memoization and dynamic programming.

Master the art of optimal substructure:
- 0/1 Knapsack Problem - Pack your belongings to maximize value within a weight limit
- Longest Common Subsequence - Find patterns hidden within two ancient texts
- Coin Change Problem - Exchange your treasures efficiently for the minimum number of coins

The shadows hold no mysteries for those who understand how to break complex problems into overlapping subproblems. Each cave chamber contains a puzzle that yields to the power of dynamic programming.

Use memoization to remember what you've already computed. Build tables of solutions from the ground up. The path through the shadows becomes clear when you see the optimal structure within each problem.

This is a journey of discovery (150 XP) - learn to transform impossible-seeming challenges into elegant solutions through the art of dynamic programming.
`;

  run(language: string = "typescript"): boolean {
    try {
      const checkFilePath = LanguageRunner.getCheckFilePath(this.id, language);
      return LanguageRunner.runCheck(language, checkFilePath);
    } catch (error) {
      console.error(`Error running chapter in ${language}:`, error);
      return false;
    }
  }
}

export default Chapter6;
