const assert = require("assert");
const fs = require("fs/promises");

const wasm = fs.readFile("../wasm/i32.wasm").then((bytes) =>
  WebAssembly.instantiate(bytes, {
    /* NO IMPORTS */
  })
);

(async () => {
  let unexported;
  const vm = await wasm;
  assert(vm.instance.exports.magic() === 42);
  assert(vm.instance.exports.minus1() === -1);
  assert(vm.instance.exports.zero === unexported);
  console.info("test result: ok.");
})();

module.exports = {
  wasm,
};
