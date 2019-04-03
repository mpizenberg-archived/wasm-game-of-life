import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Construct the universe, and get its width and height.
const universe = Universe.new(512, 512);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext("2d");
// ctx.imageSmoothingEnabled = false;
canvas.height = height;
canvas.width = width;

const renderLoop = () => {
  universe.tick();
  drawCells();

  requestAnimationFrame(renderLoop);
};

const drawCells = () => {
  const canvas_data = new Uint8ClampedArray(
    memory.buffer,
    universe.canvas_data(),
    4 * width * height
  );
  const image_data = new ImageData(canvas_data, width, height);
  ctx.putImageData(image_data, 0, 0);
};

drawCells();
requestAnimationFrame(renderLoop);
