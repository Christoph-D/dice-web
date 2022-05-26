# Dice

A Rust WebAssembly experiment that evaluates RPG-style dice specifications like `2d6+4`.

## Example

https://yozora.eu/dice/

Try these:

* **1d6**: One 6-sided die
* **2d6+4**: Two 6-sided dice plus 4
* **1d6+1d8+1d10**: One 6-sided, one 8-sided, and one 10-sided die
* **100000d6**: One hundred thousand 6-sided dice

## Deploy

```shell
$ cargo install wasm-pack
$ wasm-pack build --release --no-default-features
$ cd www
$ npm install
$ npm run build
$ npm start
```
