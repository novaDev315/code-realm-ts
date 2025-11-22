import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter3 extends Chapter {
  id = 3;
  title = "Mirror Maze";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
🪞 Welcome to the Mirror Maze, where patterns reflect endlessly.
Master the techniques of optimization:
- Sliding Window - See beyond the immediate view
- Two Pointers - Approach problems from both ends

These patterns will help you navigate the maze efficiently.
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

export default Chapter3;
