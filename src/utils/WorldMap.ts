import * as fs from "fs/promises";

import { Player } from "../engine/Player";

export class WorldMap {
  static async update(player: Player): Promise<void> {
    const content = `# Code Realm — World Map

Rank: ${player.rank}\nXP: ${player.xp}\nCompleted Chapters: ${player.completedChapters.join(", ") || "None"}\n`;
    await fs.mkdir("realm", { recursive: true });
    await fs.writeFile("realm/world-map.md", content);
  }
} 