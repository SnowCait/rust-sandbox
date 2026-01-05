# Wasm

https://developer.mozilla.org/ja/docs/WebAssembly/Guides/Rust_to_Wasm

## --target web

```shell
wasm-pack build --target web
npx serve .
```

## --target bundler

```shell
wasm-pack build --target bundler
cd site/
npm run serve
```
