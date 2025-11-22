import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter12 extends Chapter {
  id = 12;
  title = "Hall of Echoes";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
🔊 Welcome to the Hall of Echoes, where messages flow like whispers through an infinite corridor.

Master the art of Message Queues and Asynchronous Processing:
- FIFO Principle - First In, First Out ordering ensures fair message handling
- Enqueue Operation - Add messages to the queue without blocking
- Dequeue Operation - Remove and process messages in order
- Peek Operation - Inspect the next message without consuming it
- Batch Processing - Handle multiple messages efficiently in groups
- Queue Management - Track queue size and determine when it's empty

This Chapter (150 XP) tests your understanding of queue data structures, asynchronous communication patterns, and batch processing. Implement an efficient message queue that handles concurrent message flows, ensures FIFO ordering, and supports batch operations. Master these concepts and you'll understand the backbone of modern messaging systems, from email queues to task processors!

Learn how distributed systems use message queues to decouple components, handle backpressure, and scale horizontally. The Hall of Echoes awaits those who understand that sometimes, the order of things matters most.
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

export default Chapter12;
