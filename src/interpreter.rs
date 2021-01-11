use crate::lex::lex;
use crate::parse::parse;
use crate::eval::{eval, Environment, EvalResult};

/// Lexes, parses, and evaluates the given program.
pub fn run_interpreter(program: &str) -> EvalResult {
    // TODO
    //unimplemented!()
    match lex (&program){
        Err(err) => EvalResult::Err("Lex error: {:?}".into()),
        Ok(tokens) => match parse(&tokens){
            Err(err) => EvalResult::Err("parse error: {:?}".into()),
            Ok(expr) => {
                let mut env = Environment::default();
                    return eval(expr.clone(), &mut env)
            },
        }
    }  
}
