use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Type {
    Unknown,
    I32,
    Func(Box<Type>, Box<Type>),
}

enum Expr {
    I32(i32),
    Name(String),
    Func(String, Box<Expr>), //* name -> Expr
}

type Env = HashMap<String, Type>;

trait Match {
    fn check_type(&self, env: &Env, t: Type) -> Result<(), Type>;
}

impl Match for Expr {
    fn check_type(&self, env: &Env, want: Type) -> Result<(), Type> {
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
                    expr.check_type(&env_, expr_type.as_ref().clone())
                }
                _ => Err(want),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(Expr::I32(0).check_type(&HashMap::new(), Type::I32).is_ok());

        assert!(Expr::Name("a".to_string())
            .check_type(&HashMap::from([("a".to_string(), Type::I32)]), Type::I32)
            .is_ok());

        assert!(
            Expr::Func("a".to_string(), Box::new(Expr::I32(0))) //* a -> 0
                .check_type(
                    &HashMap::new(),
                    Type::Func(Box::new(Type::Unknown), Box::new(Type::I32))
                )
                .is_ok()
        );

        assert!(
            Expr::Func("a".to_string(), Box::new(Expr::Name("a".to_string()))) //* a -> a
                .check_type(
                    &HashMap::new(),
                    Type::Func(Box::new(Type::I32), Box::new(Type::I32))
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
    }
}
