# Dice

A Rust WebAssembly experiment that evalutes RPG-style dice specifications like `2d6+4`.

## Example

https://yozora.eu/dice/

## Deploy

```shell
$ cargo install wasm-pack
$ wasm-pack build --release --no-default-features
$ cd www
$ npm install
$ npm run build
$ npm start
```
