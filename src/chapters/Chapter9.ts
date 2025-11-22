import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter9 extends Chapter {
  id = 9;
  title = "Dungeon of DevOps";
  xpReward = 150;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
Welcome to the Dungeon of DevOps, where containerization and orchestration are the keys to mastery.

In this realm, you must harness the power of Docker and container orchestration:
- Docker Fundamentals - Craft Dockerfiles that define your application's environment and dependencies
- Container Configuration - Design multi-stage builds and optimize image sizes for production
- Orchestration with Docker Compose - Coordinate multiple services and define network topologies
- Health Checks & Monitoring - Implement health check strategies to ensure service resilience
- Environment Management - Configure environment variables and secrets for different deployment stages

This Chapter (150 XP) tests your understanding of containerization principles and DevOps practices. Build efficient container images, orchestrate complex service architectures, and implement production-ready health checks to achieve DevOps mastery.
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

export default Chapter9;
