install:
    cargo install --path .

compile: install
    church-lang ./ch/i32.ch > ./wasm/i32.wasm

try:
    cd ./test && node test.js

test: compile try
