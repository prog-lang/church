use std::collections::HashSet;

use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChurchParser;

pub fn is_not_eoi(pair: &Pair<Rule>) -> bool {
    if let Rule::EOI = pair.as_rule() {
        false
    } else {
        true
    }
}

pub fn uid(pair: Pair<Rule>) -> String {
    pair.as_str().to_string()
}

pub fn id(pair: Pair<Rule>) -> String {
    pair.as_str().to_string()
}

pub fn int(pair: Pair<Rule>) -> i32 {
    pair.as_str().parse().unwrap()
}

pub fn exports(pairs: Pairs<Rule>) -> HashSet<String> {
    HashSet::from_iter(pairs.into_iter().map(id))
}

// #[cfg(test)]
// mod tests {
//     use core::panic;

//     use crate::ast::ModuleHeader;

//     use super::*;

//     #[test]
//     fn parse_i32_const() {
//         let src = "
//         module Numbers (magic, zerO0);

//         minus5 = -5;
//         zerO0 = 0;
//         meaning = 42;
//         "
//         .to_string();

//         let parse_result = ChurchParser::parse_string(src);
//         if let Err(err) = parse_result {
//             panic!("{}", err);
//         }

//         let got_ast = parse_result.unwrap();
//         let want_ast = AST {
//             module: ModuleHeader {
//                 name: "Numbers".to_string(),
//                 exports: vec!["magic".to_string(), "zerO0".to_string()],
//             },
//             declarations: vec![
//                 Declaration {
//                     name: "minus5".to_string(),
//                     int: -5,
//                 },
//                 Declaration {
//                     name: "zerO0".to_string(),
//                     int: 0,
//                 },
//                 Declaration {
//                     name: "meaning".to_string(),
//                     int: 42,
//                 },
//             ],
//         };
//         assert_eq!(got_ast, want_ast);
//     }
// }
