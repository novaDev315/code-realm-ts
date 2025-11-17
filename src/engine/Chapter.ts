export abstract class Chapter {
  abstract id: number;
  abstract title: string;
  abstract xpReward: number;
  abstract lore: string;

  /**
   * Languages supported by this chapter.
   * Default: ['typescript']
   */
  abstract supportedLanguages: string[];

  /**
   * Execute the challenge for this chapter.
   * Should return true if user passes, false otherwise.
   * @param language - The programming language to use (default: 'typescript')
   */
  abstract run(language?: string): boolean | Promise<boolean>;
} 