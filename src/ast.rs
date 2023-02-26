use std::collections::{HashMap, HashSet};

use pest::iterators::{Pair, Pairs};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::parser;
use crate::parser::Rule;

#[derive(Default, Debug, PartialEq)]
pub struct AST {
    pub module: ModuleHeader,
    pub declarations: HashMap<String, Declaration>,
}

#[derive(Default, Debug, PartialEq)]
pub struct ModuleHeader {
    pub name: String,
    pub exports: HashSet<String>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Declaration {
    pub index: usize,
    pub name: String,
    pub value: Value,
}

#[derive(Default, Debug, PartialEq)]
pub struct Value(i32);

impl TryFrom<Pairs<'_, Rule>> for AST {
    type Error = String;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let mut ast = Self::default();

        for (index, pair) in pairs.into_iter().filter(parser::is_not_eoi).enumerate() {
            ast.declaration(index, pair);
        }

        ast.validate()
    }
}

impl AST {
    fn declaration(&mut self, index: usize, pair: Pair<Rule>) {
        match pair.as_rule() {
            Rule::module => self.module = ModuleHeader::from(pair.into_inner()),
            Rule::declaration => {
                let mut decl = Declaration::from(pair.into_inner());
                decl.index = index - 1; // module header is the first declaration
                self.declarations.insert(decl.name.clone(), decl);
            }
            _ => unreachable!(),
        }
    }

    fn validate(self) -> Result<Self, String> {
        for export in self.module.exports.iter() {
            if !self.declarations.contains_key(export) {
                return Err(format!(
                    "module '{}' exports an unknown identifier '{}'.",
                    self.module.name, export
                ));
            }
        }
        Ok(self)
    }

    fn sorted_declarations(&self) -> Vec<&Declaration> {
        let mut declarations: Vec<&Declaration> = self.declarations.values().collect();
        declarations.sort_by_key(|decl| decl.index);
        declarations
    }
}

impl Into<Vec<u8>> for AST {
    fn into(self) -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.function(vec![], vec![ValType::I32]);

        let mut functions = FunctionSection::new();
        let type_index = 0;
        for _ in self.declarations.iter() {
            functions.function(type_index);
        }

        let mut exports = ExportSection::new();
        for decl in self.declarations.values() {
            if self.module.exports.contains(&decl.name) {
                exports.export(&decl.name, ExportKind::Func, decl.index as u32);
            }
        }

        let mut codes = CodeSection::new();
        for decl in self.sorted_declarations() {
            let locals = vec![];
            let mut f = Function::new(locals);
            f.instruction(&Instruction::I32Const(decl.value.0));
            f.instruction(&Instruction::Return);
            f.instruction(&Instruction::End);
            codes.function(&f);
        }

        module.section(&types);
        module.section(&functions);
        module.section(&exports);
        module.section(&codes);
        module.finish()
    }
}

impl From<Pairs<'_, Rule>> for ModuleHeader {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut module = ModuleHeader::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::Name => module.name = parser::uid(pair),
                Rule::tuple => module.exports = parser::exports(pair.into_inner()),
                _ => unreachable!(),
            }
        }

        module
    }
}

impl From<Pairs<'_, Rule>> for Declaration {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut declaration = Declaration::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::name => declaration.name = parser::id(pair),
                Rule::integer => declaration.value = Value(parser::int(pair)),
                _ => unreachable!(),
            }
        }

        declaration
    }
}
