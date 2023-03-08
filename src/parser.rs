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
