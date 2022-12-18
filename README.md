# calc-s2-rust

[ssimulacra2](https://github.com/cloudinary/ssimulacra2) 计算工具，使用 Rust 编写并构建为 Wasm，使其可以运行在 Web 端。

## Usage

npm package: [https://www.npmjs.com/package/calc-s2-rust](https://www.npmjs.com/package/calc-s2-rust)

Demo: [https://github.com/iola1999/calc-s2-vitefe](https://github.com/iola1999/calc-s2-vitefe)

## Build

`wasm-pack build  --target web`

## Credits

ssimulacra2 的计算部分依赖 [https://github.com/rust-av/ssimulacra2](https://github.com/rust-av/ssimulacra2)，但因没能关闭的 rayon feature（会导致无法直接跑在浏览器端），故暂时直接复制了代码过来修改。

## TODO

- [ ] https://github.com/GoogleChromeLabs/wasm-bindgen-rayon rayon polyfill
