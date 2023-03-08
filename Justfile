install:
    cargo install --path .

compile:
    church-lang ./ch/i32.ch > ./wasm/i32.wasm

try:
    cd ./test && node test.js

test: install compile try
