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
    SLASH,
    EOF,

    // double -character tokens
    EQUALEQUAL,
    NOTEQUAL,
    LESSEQUAL,
    GREATEREQUAL,
    AND,
    OR,
    

    // literals
    IDENTIFIER,
    NUMBER,
    STRING,

    // keywords
    LET,
    CONST,
    FUNC,
    IF,
    ELSE,
    FOR,
    WHILE,
    DO,
    BREAK,
    CONTINUE,
    TRUE,
    FALSE,
    NONE,
    IMPORT,
    PRINT,
    INPUT,
    SWITCH,
    CASE,
    DEFAULT,
    RETURN,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let literal = self.literal.as_deref().unwrap_or("null");
        write!(
            f,
            "Token(type= {:?}, lexeme= '{}', literal= {}, line= {})",
            self.token_type, self.lexeme, literal, self.line
        )
    }
}