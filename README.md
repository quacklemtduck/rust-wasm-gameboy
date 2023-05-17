# WebBoy

An online GameBoy emulator.

## How to play

The emulator is hosted at https://quacklemtduck.github.io/.
If you need a game to run you can download the test rom from the /test-rom/ folder. The file is called `test_rom.gb` https://github.com/quacklemtduck/rust-wasm-gameboy/blob/main/test-rom/test_rom.gb

You can then pick your game by clicking `Pick game` which will open popup with a library of the available games. To upload your own click on `Select local file` and pick a file ending in `.gb`. After uploading a game you can now select it from the library. After selecting a game, just click `Run` to start the emulator.

### Controls

| GameBoy | KeyBoard   |
|---------|------------|
| A       | S          |
| B       | A          |
| Start   | Enter      |
| Select  | Period "." |
| D-pad   | Arrow keys |

## How to compile and run
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
