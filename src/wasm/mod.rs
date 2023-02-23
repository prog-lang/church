mod bin;

use crate::ast::AST;

pub struct Module {}

impl Module {
    pub fn from(ast: &AST) -> Self {
        Self {}
    }

    pub fn to_wasm(&self) -> bin::WASM {
        vec![bin::MODULE_HEADER.to_vec(), bin::VERSION_HEADER.to_vec()].concat()
    }
}
