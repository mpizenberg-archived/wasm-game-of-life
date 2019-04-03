# Game of life in Rust compiled to wasm

This repository is the result of following the [Rust Wasm book][book].

[book]: https://rustwasm.github.io/docs.html

## Cheatsheet

Recap of steps:

### Load wasm-pack template

cargo generate --git https://github.com/rustwasm/wasm-pack-template
-> prompt: wasm-game-of-life

### Build the project (creates pkg/)

wasm-pack build

### the web part of the project in www/

npm init wasm-app www
cd www
npm install

### my module into node modules without needing to publish to npm

cd ../pkg
npm link

### the newly created module

cd ../www
npm link wasm-game-of-life

### www/index.js to import "wasm-game-of-life" and then, Start the server (watch)

npm run start

### edit your code, and just re-run wasm-pack build in root dir

cd ..
wasm-pack build
