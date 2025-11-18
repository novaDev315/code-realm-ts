import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter11 extends Chapter {
  id = 11;
  title = "Tower of Constructs";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python"];
  lore = `
🏗️ High above the clouds stands the Tower of Constructs, where system architects design solutions at scale.

The Tower teaches the principles of system design and scalability:
- Base62 Encoding - Efficiently encode large numbers into compact strings (0-9a-zA-Z)
- URL Shortening - Design a distributed system that maps long URLs to short codes
- Collision Resolution - Handle duplicate URLs with idempotent operations
- Data Structure Patterns - Use hash maps for O(1) lookups and O(1) storage
- Scalability Principles - Design systems that grow efficiently without performance degradation

Master the art of building URL shorteners, understanding database design patterns, and implementing efficient encoding schemes. This chapter (150 XP) challenges you to think about system architecture, data distribution, and the algorithms that power real-world services like bit.ly and TinyURL.

Learn how trillion-scale systems handle mapping requests, and how clever encoding can reduce storage requirements while maintaining uniqueness. Prove your mastery of system design fundamentals.
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

export default Chapter11;
