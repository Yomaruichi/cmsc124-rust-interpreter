use crate::ast::{Expr, Program};
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
pub fn interpret(program: &Program) -> Result<(), RuntimeError> {
    for expr in &program.expressions {
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
            let right = evaluate(right)?;
            match operator.token_type {
                TokenType::MINUS => {
                    let n = number_unary(operator, &right)?;
                    Ok(Value::Number(-n))
                }

                // add an error here for a unsupported operator
                _ => todo!()
            }
        }

        Expr::Binary{left, operator, right} => {
            evaluate_binary(left, operator, right)
        }

        Expr::Grouping (inner) => {
            evaluate(inner)
        }
    }
}
 


fn number_unary(operator: &Token, value: &Value) -> Result<f64, RuntimeError>{
    match value {
        Value::Number(n) => Ok(*n),
        _ => Err(RuntimeError::new(operator, "Operand must be a number.")),
    }
}


fn evaluate_binary(left: &Expr, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
 
    let l = evaluate(left)?;
    let r = evaluate(right)?;
 
    match operator.token_type {
     
        TokenType::MINUS => {
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a - b))
        }
        TokenType::STAR => {
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a * b))
        }
        TokenType::SLASH => {
            let (a, b) = number_operands(operator, &l, &r)?;
            if (b == 0.0) {
                return Err(RuntimeError::new(operator, "Division by zero."));
            }
            Ok(Value::Number(a / b))
        }
        TokenType::MODULO => {
            let (a, b) = number_operands(operator, &l, &r)?;
            if (b == 0.0) {
                return Err(RuntimeError::new(operator, "Modulo by zero."));
            }
            Ok(Value::Number(a % b))
        }
        TokenType::PLUS => match (&l, &r) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
 
            _ => Err(RuntimeError::new(operator, "Operands must be two numbers.")),
        },
 
        (_) => todo!()
    }
}

//used for binary pattern
fn number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64), RuntimeError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => Ok((*a, *b)),
        _ => Err(RuntimeError::new(operator, "Operands must be numbers.")),
    }
}
