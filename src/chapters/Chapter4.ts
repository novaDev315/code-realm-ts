import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter4 extends Chapter {
  id = 4;
  title = "Stream of Thoughts";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript"];
  lore = `
In the realm of asynchronous operations, time flows differently.
Master the Stream of Thoughts to command the event loop:

- Async/Await: The spell of sequential time
- Promises: The threads of future results
- Parallelism: Execute multiple streams at once
- Retry Logic: When the first attempt fails, try again

The event loop is your ally. Learn to yield control and wait for results.
Some tasks take time - but they need not block your progress.
Use parallel operations to accomplish more in less sequential time.
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

export default Chapter4;
