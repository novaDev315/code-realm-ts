import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter1 extends Chapter {
  id = 1;
  title = "Forest of Recursion";
  xpReward = 100;
  supportedLanguages = ["typescript", "javascript", "python"];
  lore = `
🌲 Deep in the Forest of Recursion lies the elder tree Yggloop.
It speaks only in repeating patterns. Solve its trials:
- fibonacci(n)
- factorial(n)
- stringPermutations(str)
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

export default Chapter1; 