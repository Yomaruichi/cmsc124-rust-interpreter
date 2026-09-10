use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub had_error: bool,
}

impl Scanner {
    pub fn new_string(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token { 
            token_type: TokenType::EOF, 
            lexeme: String::new(), 
            literal: None, 
            line: self.line });

        &self.tokens
    }

    pub fn scan_token(&mut self) {
        // must advance input (pagawa advance function :>)
        let input;

        //also pagawa ng add_token function
        match input {
            '(' => self.add_token(TokenType::L_PAREN),
            ')' => self.add_token(TokenType::R_PAREN),
            '{' => self.add_token(TokenType::L_CURLY),
            '}' => self.add_token(TokenType::R_CURLY),
            ',' => self.add_token(TokenType::COMMA),
            ';' => self.add_token(TokenType::SEMICOLON),
            '=' => self.add_token(TokenType::EQUAL),
            '!' => self.add_token(TokenType::NOT),
            '<' => self.add_token(TokenType::LESS),
            '>' => self.add_token(TokenType::GREATER),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            '*' => self.add_token(TokenType::STAR),
            '/' => self.add_token(TokenType::SLASH),
            ' ' => {}
            //also make error function :>
            _ => self.error
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}