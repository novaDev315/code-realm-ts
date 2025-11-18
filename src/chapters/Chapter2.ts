import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter2 extends Chapter {
  id = 2;
  title = "Mountains of Order";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python"];
  lore = `
⛰️  You ascend the Mountains of Order, where chaos must be tamed.
The ancient monks teach the sacred arts of sorting:
- MergeSort - Divide and Conquer the disorder
- QuickSort - Swift as the mountain wind

Master both techniques to bring order to chaos.
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

export default Chapter2;
