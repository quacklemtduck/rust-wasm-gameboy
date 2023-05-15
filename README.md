# WebBoy

To be able to run this you need to have Node and NPM installed. You also need to have Rust and wasm-pack [https://rustwasm.github.io/wasm-pack/] installed.

To build the WASM package you can then run

```bash
npm run build:wasm
```

And then do

```bash
npm install ./gameboy/pkg
```

To install the WASM package.

Then to run the emulator you can do

```bash
npm start
```
