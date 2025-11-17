import { Player } from "./Player";
import { IO } from "../utils/IO";
import { ChapterLoader } from "./ChapterLoader";
import { LanguageRunner } from "../utils/LanguageRunner";

export class Game {
  private player: Player;

  constructor() {
    this.player = new Player();
    // load later in start()
  }

  async start(): Promise<void> {
    await this.player.load();
    IO.println("🌟 Welcome to Code Realm TS!");
    let running = true;
    while (running) {
      IO.println(`\nRank: ${this.player.rank} | XP: ${this.player.xp}`);
      const choice = await IO.prompt("Choose: [S]tory, [M]anual, [R]eset, [E]xit: ");
      switch (choice.toLowerCase()) {
        case "s":
          await this.playNextChapter();
          break;
        case "m":
          await this.manualMode();
          break;
        case "r":
          await this.player.reset();
          IO.println("Progress reset.");
          break;
        case "e":
          running = false;
          break;
        default:
          IO.println("Invalid choice.");
      }
    }
    await IO.close();
    IO.println("Goodbye!");
  }

  private async playNextChapter() {
    const nextId = (this.player.completedChapters.at(-1) ?? 0) + 1;
    await this.playChapter(nextId);
  }

  private async manualMode() {
    const idStr = await IO.prompt("Enter chapter number to replay: ");
    const id = Number(idStr);
    if (!this.player.completedChapters.includes(id)) {
      IO.println("Chapter not unlocked yet.");
      return;
    }
    await this.playChapter(id);
  }

  private async playChapter(id: number) {
    const chapter = ChapterLoader.load(id);
    if (!chapter) {
      IO.println("Chapter not found or not implemented yet.");
      return;
    }
    IO.println(`\nEntering ${chapter.title}\n${chapter.lore}\n`);

    // Language selection
    const supportedLanguages = chapter.supportedLanguages;
    let selectedLanguage = "typescript"; // default

    if (supportedLanguages.length > 1) {
      IO.println("📚 Supported languages for this chapter:");
      supportedLanguages.forEach((lang, index) => {
        const config = LanguageRunner.getLanguageConfig(lang);
        IO.println(`  ${index + 1}. ${config?.name || lang}`);
      });

      const langChoice = await IO.prompt(
        `\nSelect language (1-${supportedLanguages.length}) or press Enter for TypeScript: `
      );

      if (langChoice.trim()) {
        const index = parseInt(langChoice) - 1;
        if (index >= 0 && index < supportedLanguages.length) {
          selectedLanguage = supportedLanguages[index];
        } else {
          IO.println("Invalid selection, using TypeScript.");
        }
      }
    }

    const config = LanguageRunner.getLanguageConfig(selectedLanguage);
    IO.println(`\n🔧 Running challenge in ${config?.name || selectedLanguage}...\n`);

    const success = await chapter.run(selectedLanguage);
    if (success) {
      const firstClear = !this.player.completedChapters.includes(id);
      if (firstClear) {
        IO.println("✅ Challenge passed! XP awarded.");
        await this.player.addXP(chapter.xpReward);
        await this.player.markChapterComplete(id);
      } else {
        IO.println("♻️  Chapter replayed — no additional XP (already cleared).");
      }
    } else {
      IO.println("❌ Challenge failed. Try again!");
    }
  }
} 