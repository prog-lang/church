extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod parser;

use std::{
    fs,
    io::{stdout, Write},
};

use ast::AST;
use parser::{ChurchParser, Rule};
use pest::Parser;

fn main() {
    let path = std::env::args().nth(1).expect("source path missing");
    let src = fs::read_to_string(path).expect("failed to read source file");
    let parsed = ChurchParser::parse(Rule::file, &src);
    if let Err(syntax_error) = parsed {
        println!("Syntax error:\n{}", syntax_error);
        return;
    }
    let ast = AST::try_from(parsed.unwrap());
    if let Err(semantic_error) = ast {
        println!("Semantic error:\n{}", semantic_error);
        return;
    }
    stdout()
        .write_all(Into::<Vec<u8>>::into(ast.unwrap()).as_slice())
        .expect("failed to write WebAssembly binary to stdout");
}
