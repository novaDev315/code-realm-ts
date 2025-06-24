import * as fs from "fs/promises";
import * as path from "path";
import { XPSystem, Rank } from "../utils/XPSystem";
import { WorldMap } from "../utils/WorldMap";

export interface PlayerData {
  xp: number;
  rank: Rank;
  completedChapters: number[];
}

export class Player {
  private data: PlayerData = { xp: 0, rank: "Initiate", completedChapters: [] };
  private readonly file: string;

  constructor(file = "realm/xp-tracker.json") {
    this.file = file;
  }

  get xp() {
    return this.data.xp;
  }
  get rank() {
    return this.data.rank;
  }
  get completedChapters() {
    return this.data.completedChapters;
  }

  async addXP(amount: number) {
    this.data.xp += amount;
    this.data.rank = XPSystem.getRank(this.data.xp);
    await this.save();
    await WorldMap.update(this);
  }

  async markChapterComplete(id: number) {
    if (!this.data.completedChapters.includes(id)) {
      this.data.completedChapters.push(id);
      await this.save();
    }
  }

  async reset() {
    this.data = { xp: 0, rank: "Initiate", completedChapters: [] };
    await this.save();
    await WorldMap.update(this);
  }

  async load(): Promise<void> {
    try {
      const raw = await fs.readFile(this.file, "utf8");
      this.data = JSON.parse(raw);
    } catch {
      await this.save();
    }
  }

  private async save(): Promise<void> {
    await fs.mkdir(path.dirname(this.file), { recursive: true });
    await fs.writeFile(this.file, JSON.stringify(this.data, null, 2));
  }
} 