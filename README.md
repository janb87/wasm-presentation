# wasm-presentation

## View presentation

```bash
npm install
npm start
```

Open `http://localhost:3000` in the browser, user the arrow keys to navigate.

## Build example

### Install / configure Rust

```bash
brew install rustup
rustup-init
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install wasm-gc
cargo install https
```

### Start example

```bash
cd example
npm install
npm start
```
