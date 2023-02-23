use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

#[derive(Debug, PartialEq)]
pub struct AST(pub Vec<Declaration>);

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub int: i32,
}

impl Into<Vec<u8>> for AST {
    fn into(self) -> Vec<u8> {
        let declarations = self.0;
        let mut module = Module::new();

        let mut types = TypeSection::new();
        types.function(vec![], vec![ValType::I32]);

        let mut functions = FunctionSection::new();
        let type_index = 0;
        for _ in declarations.iter() {
            functions.function(type_index);
        }

        let mut exports = ExportSection::new();
        for (index, decl) in declarations.iter().enumerate() {
            exports.export(&decl.name, ExportKind::Func, index as u32);
        }

        let mut codes = CodeSection::new();
        for decl in declarations.iter() {
            let locals = vec![];
            let mut f = Function::new(locals);
            f.instruction(&Instruction::I32Const(decl.int));
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
