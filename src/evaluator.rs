mod coercion;

use crate::ast::{Expr, Program};
use crate::token::{Token, TokenType, Value};
use crate::evaluator::coercion::coerce_number;
 

 
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
        Value::Boolean(n) => n.to_string(),
        Value::Nil => "nil".to_string(),
        other => other.to_string(),
    }
}

// Literals and grouping preserve their values; operator expressions dispatch to their type rules.
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
 


// Converts a number or boolean operand to f64 for unary minus; strings and nil are rejected.
fn number_unary(operator: &Token, value: &Value) -> Result<f64, RuntimeError>{
    coerce_number(operator, value)
}


// Evaluates operands left to right, then applies arithmetic, numeric ordering, or strict equality.
// Ordering accepts numbers/booleans; equality accepts every Value variant without coercion.
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
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Division by zero."));
            }
            Ok(Value::Number(a / b))
        }

        TokenType::MODULO => {
            let (a, b) = number_operands(operator, &l, &r)?;
            if b == 0.0 {
                return Err(RuntimeError::new(operator, "Modulo by zero."));
            }
            Ok(Value::Number(a % b))
        }

        TokenType::PLUS => {
            if matches!(l, Value::Nil) || matches!(r, Value::Nil) {
                return Err(RuntimeError::new(operator, "Nil cannot be used in arithmetic."));
            }
            if matches!(l, Value::String(_)) || matches!(r, Value::String(_)) {
                return Ok(Value::String(stringify(&l) + &stringify(&r)));
            }
            let (a, b) = number_operands(operator, &l, &r)?;
            Ok(Value::Number(a + b))
        },

        TokenType::GREATER | TokenType::GREATEREQUAL | TokenType::LESS | TokenType::LESSEQUAL => {
            if matches!(l, Value::Nil) || matches!(r, Value::Nil) {
                return Err(RuntimeError::new(operator, "Nil cannot be used in comparisons."));
            }
            let (a, b) = number_operands(operator, &l, &r)?;
            let result = match operator.token_type {
                TokenType::GREATER => a > b,
                TokenType::GREATEREQUAL => a >= b,
                TokenType::LESS => a < b,
                _ => a <= b, // This arm can only be LESSEQUAL after the outer match.
            };
            Ok(Value::Boolean(result))
        }
        // Value's derived PartialEq compares both enum variant and contained value.
        TokenType::EQUALEQUAL => Ok(Value::Boolean(l == r)),
        TokenType::NOTEQUAL => Ok(Value::Boolean(l != r)),

        (_) => todo!()
    }
}

// Converts two number or boolean operands to f64; strings and nil produce a runtime error.
// Shared by arithmetic and ordering so boolean conversion stays consistent (true = 1, false = 0).
fn number_operands(operator: &Token, left: &Value, right: &Value) -> Result<(f64, f64), RuntimeError> {
    Ok((coerce_number(operator, left)?, coerce_number(operator, right)?))
}
