import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter7 extends Chapter {
  id = 7;
  title = "Vault of Data";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
🗄️ You discover the Vault of Data, an ancient repository where information is organized and preserved.
The Vault Keepers teach the sacred art of Database Design:
- CREATE TABLE - Structure your data foundations
- SELECT & JOIN - Query and combine data from multiple sources
- Indexing - Speed up your searches through vast archives
- Data Relationships - Connect information in meaningful ways

Master SQL and database design to unlock the secrets contained within this vault.
The knowledge here is the foundation of all great systems.
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

export default Chapter7;
