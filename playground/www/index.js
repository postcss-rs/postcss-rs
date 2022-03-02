import init, { ast } from "postcss-rs-wasm";

(async () => {
  await init();
  console.log(ast(`.test{}`));
})();
