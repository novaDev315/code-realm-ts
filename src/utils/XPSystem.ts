const RANKS = [
  { name: "Initiate", xp: 0 },
  { name: "Apprentice", xp: 100 },
  { name: "Adept", xp: 300 },
  { name: "Architect", xp: 600 },
  { name: "Grandmaster", xp: 1000 }
] as const;

export type Rank = typeof RANKS[number]["name"];

export class XPSystem {
  static getRank(xp: number): Rank {
    let current: Rank = "Initiate";
    for (const r of RANKS) {
      if (xp >= r.xp) {
        current = r.name;
      } else {
        break;
      }
    }
    return current;
  }
} 