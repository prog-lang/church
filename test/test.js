const assert = require("assert");
const fs = require("fs/promises");

const wasm = fs.readFile("../wasm/i32.wasm").then((bytes) =>
  WebAssembly.instantiate(bytes, {
    /* NO IMPORTS */
  })
);

(async () => {
  const vm = await wasm;
  assert(vm.instance.exports.magic() === 42);
  console.info("test result: ok.");
})();

module.exports = {
  wasm,
};
