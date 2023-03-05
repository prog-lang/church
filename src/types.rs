use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Unknown,
    I32,
    Func(Box<Type>, Box<Type>),
}

pub type Env = HashMap<String, Type>;

pub trait Match {
    fn check_type(&self, env: &Env, t: Type) -> Result<(), Type>;
}
