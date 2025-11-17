import { execSync } from "child_process";
import * as path from "path";
import * as fs from "fs";

export interface LanguageConfig {
  name: string;
  extensions: string[];
  testCommand: (filePath: string) => string;
  checkCommand: (filePath: string) => string;
}

export const SUPPORTED_LANGUAGES: Record<string, LanguageConfig> = {
  typescript: {
    name: "TypeScript",
    extensions: [".ts"],
    testCommand: (filePath: string) => `ts-node ${filePath}`,
    checkCommand: (filePath: string) => `ts-node ${filePath}`,
  },
  javascript: {
    name: "JavaScript",
    extensions: [".js"],
    testCommand: (filePath: string) => `node ${filePath}`,
    checkCommand: (filePath: string) => `node ${filePath}`,
  },
  python: {
    name: "Python",
    extensions: [".py"],
    testCommand: (filePath: string) => `python3 ${filePath}`,
    checkCommand: (filePath: string) => `python3 ${filePath}`,
  },
  go: {
    name: "Go",
    extensions: [".go"],
    testCommand: (filePath: string) => `go run ${filePath}`,
    checkCommand: (filePath: string) => `go run ${filePath}`,
  },
};

export class LanguageRunner {
  /**
   * Detect which programming language a file is written in
   */
  static detectLanguage(filePath: string): string | null {
    const ext = path.extname(filePath);
    for (const [lang, config] of Object.entries(SUPPORTED_LANGUAGES)) {
      if (config.extensions.includes(ext)) {
        return lang;
      }
    }
    return null;
  }

  /**
   * Check if a language is supported
   */
  static isLanguageSupported(language: string): boolean {
    return language in SUPPORTED_LANGUAGES;
  }

  /**
   * Get the configuration for a specific language
   */
  static getLanguageConfig(language: string): LanguageConfig | null {
    return SUPPORTED_LANGUAGES[language] || null;
  }

  /**
   * Run a test file in the specified language
   * @param language - The programming language
   * @param filePath - Path to the file to execute
   * @returns true if execution succeeds, false otherwise
   */
  static runTest(language: string, filePath: string): boolean {
    const config = this.getLanguageConfig(language);
    if (!config) {
      console.error(`❌ Unsupported language: ${language}`);
      return false;
    }

    if (!fs.existsSync(filePath)) {
      console.error(`❌ File not found: ${filePath}`);
      return false;
    }

    try {
      const command = config.testCommand(filePath);
      console.log(`\n🔧 Running: ${command}\n`);
      const output = execSync(command, {
        encoding: "utf-8",
        stdio: "inherit",
        cwd: process.cwd(),
      });
      return true;
    } catch (error) {
      console.error(`\n❌ Test execution failed`);
      return false;
    }
  }

  /**
   * Run a check/grader file in the specified language
   * @param language - The programming language
   * @param checkFilePath - Path to the check/grader file
   * @returns true if all tests pass, false otherwise
   */
  static runCheck(language: string, checkFilePath: string): boolean {
    const config = this.getLanguageConfig(language);
    if (!config) {
      console.error(`❌ Unsupported language: ${language}`);
      return false;
    }

    if (!fs.existsSync(checkFilePath)) {
      console.error(`❌ Check file not found: ${checkFilePath}`);
      return false;
    }

    try {
      const command = config.checkCommand(checkFilePath);
      console.log(`\n🧪 Running tests for ${config.name}...\n`);
      execSync(command, {
        encoding: "utf-8",
        stdio: "inherit",
        cwd: process.cwd(),
      });
      return true;
    } catch (error) {
      console.error(`\n❌ Tests failed for ${config.name}`);
      return false;
    }
  }

  /**
   * Get the folder path for a specific language in a chapter
   */
  static getLanguageFolderPath(
    chapterId: number,
    language: string
  ): string {
    return path.join(
      process.cwd(),
      "quests",
      `chapter${chapterId}`,
      language
    );
  }

  /**
   * Get the check file path for a specific language in a chapter
   */
  static getCheckFilePath(chapterId: number, language: string): string {
    const config = this.getLanguageConfig(language);
    if (!config) {
      throw new Error(`Unsupported language: ${language}`);
    }

    const ext = config.extensions[0];
    const folderPath = this.getLanguageFolderPath(chapterId, language);
    return path.join(folderPath, `check${ext}`);
  }

  /**
   * List all available languages for a chapter
   */
  static getAvailableLanguages(chapterId: number): string[] {
    const chapterPath = path.join(process.cwd(), "quests", `chapter${chapterId}`);

    if (!fs.existsSync(chapterPath)) {
      return [];
    }

    const availableLanguages: string[] = [];
    const entries = fs.readdirSync(chapterPath, { withFileTypes: true });

    for (const entry of entries) {
      if (entry.isDirectory() && this.isLanguageSupported(entry.name)) {
        availableLanguages.push(entry.name);
      }
    }

    return availableLanguages;
  }

  /**
   * Display supported languages
   */
  static displaySupportedLanguages(): void {
    console.log("\n📚 Supported Programming Languages:");
    for (const [key, config] of Object.entries(SUPPORTED_LANGUAGES)) {
      console.log(`  - ${config.name} (${key})`);
    }
    console.log();
  }
}
