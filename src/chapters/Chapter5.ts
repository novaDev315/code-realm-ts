import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter5 extends Chapter {
  id = 5;
  title = "Labyrinth of Nodes";
  xpReward = 200;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
Welcome to the Labyrinth of Nodes, a complex realm of interconnected passages and hidden paths.

Master the art of Graph Traversal:
- Depth-First Search (DFS) - Venture deep into one path before exploring others
- Breadth-First Search (BFS) - Explore all nearby passages before moving outward
- Shortest Path - Find the most efficient route through the labyrinth
- Cycle Detection - Identify loops that might trap you in endless corridors

This Master Project (200 XP) will test your understanding of graph algorithms and your ability to navigate complex data structures. Navigate the labyrinth wisely, and you shall emerge victorious!
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

export default Chapter5;
