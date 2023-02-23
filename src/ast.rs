pub type AST = Vec<Declaration>;

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: i32,
}
