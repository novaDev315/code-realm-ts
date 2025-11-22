import { Chapter } from "../engine/Chapter";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Chapter10 extends Chapter {
  id = 10;
  title = "Citadel of Firewalls";
  xpReward = 200;
  supportedLanguages = ["typescript", "javascript", "python", "go", "rust", "java", "cpp"];
  lore = `
🔐 Welcome to the Citadel of Firewalls - the MASTER PROJECT ⭐
The most crucial domain in all the Code Realm. Here you must master the ancient art of security.

The Great Sentinel stands at the gates, testing your knowledge of:
- Password Hashing: Protect secrets with cryptographic strength
- JWT Validation: Verify the identity of tokens and travelers
- Rate Limiting: Control the flow of requests to prevent abuse
- Input Sanitization: Guard against the XSS dragons and injection attacks

This is a MASTER PROJECT worth 200 XP. Those who succeed here become Guardians of Security.
Only the worthy can protect the realm from malicious forces.

Four trials await:

🔐 TRIAL 1: Hash Passwords with Salt
Transform passwords into unreadable hashes that protect user accounts.
Combine password + salt and return a cryptographic hash.

🎫 TRIAL 2: Validate JWT Tokens
Authenticate travelers by validating their JWT tokens.
Check structure (header.payload.signature) and decode payload safely.

⏱️ TRIAL 3: Implement Rate Limiting
Protect the realm from brute force attacks and DoS attempts.
Count requests within a time window and enforce limits.

🛡️ TRIAL 4: Sanitize User Input
Defend against XSS attacks by removing dangerous HTML and scripts.
Escape special characters to prevent code injection.

The safety of the entire Code Realm depends on your security skills.
Pass all four trials to earn the MASTER title!
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

export default Chapter10;
