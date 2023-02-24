install:
    cargo install --path .

compile: install
    church-lang ./ch/i32.ch > ./wasm/i32.wasm

test: compile
    cd ./test && node test.js
