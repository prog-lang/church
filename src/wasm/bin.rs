pub type WASM = Vec<u8>;

// https://webassembly.github.io/spec/core/binary/modules.html#binary-module
pub const MODULE_HEADER: [u8; 4] = [0, b'a', b's', b'm'];
pub const VERSION_HEADER: [u8; 4] = [1, 0, 0, 0];

// https://webassembly.github.io/spec/core/binary/modules.html#sections
enum Section {
    Custom,
    Type,
    Import,
    Func,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

// https://webassembly.github.io/spec/core/binary/types.html
enum Type {
    I32 = 0x7f,
    F32 = 0x7d,
    // http://webassembly.github.io/spec/core/binary/types.html#function-types
    Func = 0x60,
}

// https://webassembly.github.io/spec/core/binary/instructions.html
enum Opcodes {
    End = 0x0b,
    Return = 0x0f,
    I32_const = 0x41,
}

impl Section {
    fn encode(self, bytes: Vec<u8>) -> Vec<u8> {
        vec![vec![self as u8], bytes].concat()
    }
}
