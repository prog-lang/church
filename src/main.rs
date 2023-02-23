extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use std::fs;

use parser::ChurchParser;

fn main() {
    let path = std::env::args().nth(1).expect("source path missing");
    let src = fs::read_to_string(path).expect("failed to read source file");
    let ast = ChurchParser::parse_string(src).expect("failed to parse source file");
    println!("{:#?}", ast);
}
