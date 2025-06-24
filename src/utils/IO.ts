import readline from "readline/promises";
import { stdin as input, stdout as output } from "process";

export class IO {
  private static rl = readline.createInterface({ input, output });

  static async prompt(question: string): Promise<string> {
    try {
      return (await IO.rl.question(question)).trim();
    } catch (err: any) {
      if (err?.code === "ERR_USE_AFTER_CLOSE") {
        return "e"; // default to exit when input closed
      }
      throw err;
    }
  }

  static println(message: string): void {
    console.log(message);
  }

  static async close(): Promise<void> {
    await IO.rl.close();
  }
} 