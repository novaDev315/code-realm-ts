import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter15 extends Chapter {
  id = 15;
  title = "Core of the Architect";
  xpReward = 300;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
🏰 Welcome, Architect - To the FINAL BOSS Challenge ⭐⭐

You stand at the summit of the Code Realm, facing the greatest challenge of all:
the Core of the Architect. This is where legends are forged.

The Grand Architect has watched your journey from Chapter 1 through Chapter 14.
They have tested your understanding of fundamental data structures, algorithms, and system design.
Now comes the ultimate test: can you unite all the knowledge into one coherent, scalable system?

In the preceding chambers, you mastered:
- Chapter 13: Crystal Socket Chamber - The LRU Cache for blazing-fast data retrieval
- Chapter 12: Hall of Echoes - The Message Queue for asynchronous processing
- Chapter 14: Gate of Trials - Load Balancing and Fault Tolerance

Now, you must synthesize these three pillars into a DISTRIBUTED SYSTEM that can:

🎯 DISTRIBUTE REQUESTS across multiple servers using intelligent load balancing
🚀 CACHE RESULTS to avoid redundant processing and serve responses in milliseconds
📬 QUEUE MESSAGES asynchronously for scalable batch processing
🔍 DESIGN SCALABLE APIS that grow with demand without degrading performance
💾 OPTIMIZE DATABASES by identifying slow queries and suggesting improvements

The Challenge: 300 XP (HIGHEST REWARD - THE FINAL BOSS)

You must implement:

1. DistributedSystem Class
   - Initialize cache, queue, and load balancer subsystems
   - Process requests using all three components
   - Track comprehensive metrics (cache hits, queue size, total requests)

2. Request Processing Pipeline
   - Check LRU cache first for cached results (instant response)
   - Add uncached requests to message queue for processing
   - Use round-robin load balancer to distribute across servers
   - Cache results for future requests
   - Return response with metrics

3. System Design Function
   - Analyze requirements (RPS, data size, caching, auth)
   - Return appropriate architecture components and server estimates
   - Scale from single-server to distributed n-tier systems

4. Database Optimization
   - Identify slow SQL queries (SELECT *, missing indexes, etc.)
   - Provide specific optimization suggestions
   - Help developers write scalable database queries

SKILLS TESTED:
✓ Combining multiple data structures (cache + queue)
✓ Load balancing and request distribution
✓ System architecture and scalability
✓ Performance optimization (caching, indexing)
✓ Code organization and integration
✓ Real-world problem solving

This is not just another challenge. This is the culmination of your journey.
The systems you build here power trillion-dollar companies:
- Netflix uses caching to serve millions of streams
- Amazon uses load balancing across millions of servers
- Google uses distributed systems to process petabytes of data
- Twitter uses message queues to handle millions of tweets per second

You have been prepared for this moment.
The Code Realm has taught you everything you need to know.
Now, show the Grand Architect that you are ready to join their ranks.

Complete all tests. Optimize the database queries. Design the scalable APIs.
Prove that you are not just a developer, but an ARCHITECT.

The final test awaits... Are you ready? ⭐⭐
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

export default Chapter15;
