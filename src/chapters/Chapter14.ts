import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter14 extends Chapter {
  id = 14;
  title = "Gate of Trials";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
Welcome to the Gate of Trials, where you must master the art of distributed systems and resilience.

In this realm, you face the ultimate challenge of load balancing and fault tolerance - the guardians of system reliability:

- Round-Robin Load Balancing: Distribute requests evenly across multiple servers to prevent any single server from becoming a bottleneck. Master the algorithm that ensures fair distribution of traffic.

- Circuit Breaker Pattern: Implement the critical safeguard that prevents cascading failures. When a service starts failing, the circuit breaker trips open to protect the entire system from damage.

- Sliding Window Rate Limiting: Implement sophisticated rate limiting to protect your services from overwhelming traffic spikes and malicious attacks. Use time-based windows to enforce request limits.

This Chapter (150 XP) tests your understanding of distributed system design patterns. Implement load balancers to distribute traffic intelligently, circuit breakers to protect against failures, and rate limiters to control request flow. These patterns are essential for building scalable, resilient systems that can withstand real-world challenges.

The journey through the Gate of Trials will teach you that true mastery comes from understanding not just individual systems, but how to orchestrate many systems working together in harmony.
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

export default Chapter14;
