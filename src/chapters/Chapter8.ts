import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter8 extends Chapter {
  id = 8;
  title = "Realm of APIs";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python"];
  lore = `
Welcome to the Realm of APIs, where data flows through carefully designed channels and endpoints govern the exchange of information.

Master the foundations of modern API design:
- REST Architecture - Build scalable endpoints following resource-oriented principles
- GraphQL Queries - Parse and validate declarative data requests with precise field selection
- API Security - Validate and authenticate requests with secure key validation
- Response Handling - Structure consistent API responses with proper status codes and error handling

This Chapter (150 XP) tests your understanding of API design patterns and implementation concepts. Design your endpoints wisely, validate your queries carefully, and secure your APIs with proper authentication mechanisms.
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

export default Chapter8;
