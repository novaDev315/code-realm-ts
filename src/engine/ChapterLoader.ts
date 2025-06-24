import { Chapter } from "./Chapter";

export class ChapterLoader {
  static load(id: number): Chapter | null {
    try {
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      const mod = require(`../chapters/Chapter${id}`);
      const ChapterClass = mod.default ?? mod[`Chapter${id}`];
      if (ChapterClass) {
        return new ChapterClass();
      }
    } catch {
      /* ignore */
    }
    return null;
  }
} 