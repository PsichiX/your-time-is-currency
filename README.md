# Your Time Is Currency
### Simple multiplayer game made with Rust and Oxygen game engine

![demo](https://github.com/PsichiX/your-time-is-currency/blob/master/media/your-precious-time.gif?raw=true)

## Oxygengine
https://github.com/PsichiX/oxygengine

## Installation
- Make sure that Rust toolchain is installed ( https://rustup.rs/ );
- Make sure that Node.js is installed ( https://nodejs.org/en/ );
- Make sure that wasm-pack package is installed ( https://rustwasm.github.io/wasm-pack/ );

## Build
Launch client live development with hot reloading:
```bash
cd client/
npm start
```

Build client for production:
```bash
cd client/
npm run build
```
after that your package is ready at: `client/dist/`

## Run
Run server:
```bash
cd server/
npm start
```

Run client:
```bash
cd client/dist/
http-server
```
then go to your browser at: `http://localhost:8080`.
