import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter13 extends Chapter {
  id = 13;
  title = "Crystal Socket Chamber";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
Welcome to the Crystal Socket Chamber, a mystical vault where memory itself is crystallized into sockets of finite capacity.

Master the art of Cache Management and Memory Optimization:
- LRU Cache Strategy - Evict the least recently used data when memory reaches capacity
- Get Operations - Retrieve cached values while marking them as recently used
- Put Operations - Store new data and manage eviction policies efficiently
- Cache Hit Rate - Measure the effectiveness of your caching strategy
- Memory Constraints - Understand how to maximize performance under limited resources

This Chapter (150 XP) tests your understanding of cache data structures, eviction policies, and memory management. Implement an efficient LRU cache that balances speed and space complexity. Master these concepts and your applications will achieve lightning-fast performance even under memory constraints!
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

export default Chapter13;
