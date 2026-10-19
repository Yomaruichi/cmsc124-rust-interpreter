use crate::ast::Expr;
use crate::token::{Token, TokenType, Value};
 

 
// Keeps the operator token so the error can report a line number.
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

// create runtime error constructor
impl RuntimeError {
    fn new(token: &Token, message: &str) -> Self {
        RuntimeError { token: token.clone(), message: message.to_string() }
    }
}

// just prints out where in the code was the error; line and message included
pub fn report_runtime_error(err: &RuntimeError) {
    eprintln!("[line {}] Runtime error: {}", err.token.line, err.message);
}
 
 

// main entry point: evaluate each expression and print its result
pub fn interpret(exprs: &[Expr]) -> Result<(), RuntimeError> {
    for expr in exprs {
        let value = evaluate(expr)?;
        println!("{}", stringify(&value));
    }
    Ok(())
}
 


// runtime value -> output text. Unlike Value's Display (which keeps "5.0"
// for the tokenizer), whole numbers print as "5" here.
pub fn stringify(value: &Value) -> String {
    match value {
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}


//main evalauator for expressions
pub fn evaluate(expr: &Expr) -> Result<Value, RuntimeError> {

    // evaluate each enum type
    // start the backbone with literals first
    match expr {
        Expr::Literal(value)=> {
            return Ok(value.clone());
        }

        Expr::Unary{operator, right} => {
            todo!();
        }

        Expr::Binary{left, operator, right} => {
            todo!();
        }

        Expr::Grouping (inner) => {
            todo!();
        }
    }
}
 
