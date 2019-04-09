import { Universe, default as init } from "./pkg/wasm_game_of_life.js";

// Retrieve the canvas in the DOM.
const canvas = document.getElementById("game-of-life-canvas");
const ctx = canvas.getContext("2d");

async function run() {
  // Initialize the wasm module.
  const wasm = await init("pkg/wasm_game_of_life_bg.wasm");

  // Construct the universe, and get its width and height.
  const universe = Universe.new(512, 512);
  const width = universe.width();
  const height = universe.height();

  // Update the canvas size.
  canvas.height = height;
  canvas.width = width;

  // Definition of the render loop.
  const renderLoop = () => {
    universe.tick();
    drawCells();
    requestAnimationFrame(renderLoop);
  };

  // Function calling the wasm code to draw cells.
  const drawCells = () => {
    const canvas_data = new Uint8ClampedArray(
      wasm.memory.buffer,
      universe.canvas_data(),
      4 * width * height
    );
    const image_data = new ImageData(canvas_data, width, height);
    ctx.putImageData(image_data, 0, 0);
  };

  // Start the drawing loop.
  drawCells();
  requestAnimationFrame(renderLoop);
}

run();
