use std::collections::{HashMap, HashSet};

use pest::iterators::{Pair, Pairs};
use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::parser;
use crate::parser::Rule;
use crate::types::{self, Type};

#[derive(Default, Debug, PartialEq)]
pub struct AST {
    module: Header,
    decls: Vec<Decl>,
}

#[derive(Default, Debug, PartialEq)]
struct Header {
    name: String,
    exports: HashSet<String>,
}

impl TryFrom<Pairs<'_, Rule>> for AST {
    type Error = String;

    fn try_from(pairs: Pairs<Rule>) -> Result<Self, Self::Error> {
        let mut ast = Self::default();

        for pair in pairs.into_iter().filter(parser::is_not_eoi) {
            ast.declaration(pair);
        }

        Ok(ast)
    }
}

impl AST {
    fn declaration(&mut self, pair: Pair<Rule>) {
        match pair.as_rule() {
            Rule::module => self.module = Header::from(pair.into_inner()),
            Rule::declaration => self.decls.push(pair.into_inner().into()),
            _ => unreachable!(),
        }
    }
}

impl From<Pairs<'_, Rule>> for Header {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut module = Header::default();

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

impl From<Pairs<'_, Rule>> for Decl {
    fn from(pairs: Pairs<Rule>) -> Self {
        let mut name = String::new();
        let mut expr = Expr::I32(0);

        for pair in pairs {
            match pair.as_rule() {
                Rule::name => name = parser::id(pair),
                Rule::integer => expr = Expr::I32(parser::int(pair)),
                _ => unreachable!(),
            }
        }

        Decl { name, expr }
    }
}

#[derive(Debug, PartialEq)]
struct Decl {
    pub name: String,
    pub expr: Expr,
}

impl types::Match for Decl {
    fn check_type(&self, env: &types::Env, t: Type) -> Result<(), Type> {
        self.expr.check_type(env, t)
    }
}

#[derive(Debug, PartialEq)]
enum Expr {
    I32(i32),                //* -42
    Name(String),            //* x
    Func(String, Box<Expr>), //* x -> Expr
}

impl types::Match for Expr {
    fn check_type(&self, env: &types::Env, want: Type) -> Result<(), Type> {
        match self {
            Self::I32(_) => {
                if want == Type::I32 {
                    Ok(())
                } else {
                    Err(Type::I32)
                }
            }
            Self::Name(id) => {
                let got = env.get(id).map_or(Type::Unknown, |t| t.clone());
                if want == got {
                    Ok(())
                } else {
                    Err(got)
                }
            }
            Self::Func(param, expr) => match want {
                Type::Func(param_type, expr_type) => {
                    let mut env_ = env.clone();
                    env_.insert(param.clone(), param_type.as_ref().clone());
                    match expr.check_type(&env_, expr_type.as_ref().clone()) {
                        Err(got) => Err(Type::Func(param_type, got.into())),
                        ok => ok,
                    }
                }
                _ => Err(Type::Func(Type::Unknown.into(), Type::Unknown.into())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Match;

    #[test]
    fn it_works() {
        assert!(Expr::I32(0).check_type(&HashMap::new(), Type::I32).is_ok());

        assert!(Expr::Name("a".to_string())
            .check_type(&HashMap::from([("a".to_string(), Type::I32)]), Type::I32)
            .is_ok());

        assert!(Expr::Func("a".to_string(), Expr::I32(0).into()) //* a -> 0
            .check_type(
                &HashMap::new(),
                Type::Func(Type::Unknown.into(), Type::I32.into())
            )
            .is_ok());

        assert!(
            Expr::Func("a".to_string(), Expr::Name("a".to_string()).into()) //* a -> a
                .check_type(
                    &HashMap::new(),
                    Type::Func(Type::I32.into(), Type::I32.into())
                )
                .is_ok()
        );
    }

    #[test]
    fn it_catches_errors() {
        assert!(Expr::I32(0)
            .check_type(
                &HashMap::new(),
                Type::Func(Box::new(Type::I32), Box::new(Type::I32))
            )
            .is_err());

        assert!(Expr::Name("a".to_string())
            .check_type(
                &HashMap::from([("a".to_string(), Type::Unknown)]),
                Type::I32
            )
            .is_err());

        assert_eq!(
            Expr::Func("a".to_string(), Box::new(Expr::Name("b".to_string())))
                .check_type(
                    &HashMap::from([("a".to_string(), Type::I32)]),
                    Type::Func(Box::new(Type::I32), Box::new(Type::I32))
                )
                .unwrap_err(),
            Type::Func(Type::I32.into(), Type::Unknown.into()),
        );
    }
}

/* AST -> WASM */
impl Into<Vec<u8>> for AST {
    fn into(self) -> Vec<u8> {
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.function(vec![], vec![ValType::I32]);

        let mut functions = FunctionSection::new();
        let type_index = 0;
        for _ in self.decls.iter() {
            functions.function(type_index);
        }

        let mut exports = ExportSection::new();
        for (i, decl) in self.decls.iter().enumerate() {
            if self.module.exports.contains(&decl.name) {
                exports.export(&decl.name, ExportKind::Func, i as u32);
            }
        }

        let mut codes = CodeSection::new();
        for decl in self.decls.iter() {
            let locals = vec![];
            let mut f = Function::new(locals);
            match decl.expr {
                Expr::I32(i) => f.instruction(&Instruction::I32Const(i)),
                _ => unreachable!(),
            };
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
