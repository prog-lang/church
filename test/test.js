const assert = require("assert");
const fs = require("fs/promises");

const init = fs.readFile("../wasm/i32.wasm").then((bytes) =>
  WebAssembly.instantiate(bytes, {
    /* NO IMPORTS */
  })
);

(async () => {
  let unexported;
  const vm = await init;
  assert(vm.instance.exports.magic() === 42);
  assert(vm.instance.exports.minus1() === -1);
  assert(vm.instance.exports.o() === 0);
  assert(vm.instance.exports.zero === unexported);
  console.info("test result: ok.");
})();

module.exports = {
  init,
};

/* 
 ? PLAY IN INTERACTIVE MODE:
 * $ cd test && node
 * > const test = require('./test')
   undefined
 * > const vm = await test.init
   undefined
 * > vm.instance.exports.magic()
   42
 */
