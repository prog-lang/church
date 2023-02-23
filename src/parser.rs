use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

use crate::ast::{Declaration, AST};

type ParseResult = Result<AST, String>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChurchParser;

impl ChurchParser {
    pub fn parse_string(src: String) -> ParseResult {
        ChurchParser::parse(Rule::file, &src)
            .map_err(|e| e.to_string())
            .map(ChurchParser::pairs_to_ast)
    }

    pub fn pairs_to_ast(pairs: Pairs<Rule>) -> AST {
        pairs
            .into_iter()
            .filter(ChurchParser::is_not_eoi)
            .map(|pair| ChurchParser::declaration(pair.into_inner()))
            .collect()
    }

    fn is_not_eoi(pair: &Pair<Rule>) -> bool {
        if let Rule::EOI = pair.as_rule() {
            false
        } else {
            true
        }
    }

    fn declaration(pairs: Pairs<Rule>) -> Declaration {
        let mut declaration = Declaration {
            name: String::new(),
            value: 0,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::ID => declaration.name = ChurchParser::id(pair),
                Rule::INT => declaration.value = ChurchParser::int(pair),
                _ => unreachable!(),
            }
        }

        declaration
    }

    fn id(pair: Pair<Rule>) -> String {
        pair.as_str().to_string()
    }

    fn int(pair: Pair<Rule>) -> i32 {
        pair.as_str().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn parse_i32_const() {
        let src = "
        minus5 = -5;
        zerO0 = 0;
        meaning = 42; 
        "
        .to_string();

        let parse_result = ChurchParser::parse_string(src);
        if let Err(err) = parse_result {
            panic!("{}", err);
        }

        let got_ast = parse_result.unwrap();
        let want_ast = vec![
            Declaration {
                name: "minus5".to_string(),
                value: -5,
            },
            Declaration {
                name: "zerO0".to_string(),
                value: 0,
            },
            Declaration {
                name: "meaning".to_string(),
                value: 42,
            },
        ];
        assert_eq!(got_ast, want_ast);
    }
}
