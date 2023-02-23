extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;
mod wasm;

use std::{
    fs,
    io::{stdout, Write},
};

use parser::ChurchParser;
use wasm::Module;

fn main() {
    let path = std::env::args().nth(1).expect("source path missing");
    let src = fs::read_to_string(path).expect("failed to read source file");
    let ast = ChurchParser::parse_string(src).expect("failed to parse source file");
    let wasm = Module::from(&ast).to_wasm();
    stdout().write_all(wasm.as_slice());
}
