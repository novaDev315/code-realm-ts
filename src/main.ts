import { Game } from "./engine/Game";

(async () => {
  const game = new Game();
  await game.start();
})(); 