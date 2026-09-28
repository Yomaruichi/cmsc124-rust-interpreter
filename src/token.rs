// define values to represent literal runtime values
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

// different display for different literal values
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{:?}", n), // {:?} keeps the ".0": 5.0, not 5
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

// token type list
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LPAREN,
    RPAREN,
    LCURLY,
    RCURLY,
    COMMA,
    SEMICOLON,
    EQUAL,
    NOT,
    LESS,
    GREATER,
    PLUS,
    MINUS,
    STAR,
    MODULO,
    SLASH,
    EOF,

    // double -character tokens
    EQUALEQUAL,
    NOTEQUAL,
    LESSEQUAL,
    GREATEREQUAL,
    

    // literals
    IDENTIFIER,
    NUMBER,
    STRING,

    // keywords
    ITAKDA,
    TIYAK,
    GAWA,
    URI,
    KUNG,
    KUNDI,
    TUWING,
    HABANG,
    GAWIN,
    AT,
    OKAYA,
    TIGIL,
    ITULOY,
    TOTOO,
    MALI,
    WALA,
    ISAMA,
    IPAKITA,
    IPASOK,
    PILIIN,
    KAPAG,
    EDI,
    IBALIK,
}

// Holds token type, raw lexeme, literal value, and line number
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Value>,
    pub line: usize,
}

// Output format for token printing
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = match &self.literal {
            Some(v) => v.to_string(),
            None => "null".to_string(),
        };
        write!(
            f,
            "Token(type= {:?}, lexeme= '{}', literal= {}, line= {})",
            self.token_type, self.lexeme, literal, self.line
        )
    }
}