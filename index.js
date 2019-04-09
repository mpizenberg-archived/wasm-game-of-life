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
    universe.draw(ctx);
    requestAnimationFrame(renderLoop);
  };

  // Start the drawing loop.
  universe.draw(ctx);
  requestAnimationFrame(renderLoop);
}

run();
