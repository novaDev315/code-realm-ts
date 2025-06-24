export abstract class Chapter {
  abstract id: number;
  abstract title: string;
  abstract xpReward: number;
  abstract lore: string;

  /**
   * Execute the challenge for this chapter.
   * Should return true if user passes, false otherwise.
   */
  abstract run(): boolean | Promise<boolean>;
} 