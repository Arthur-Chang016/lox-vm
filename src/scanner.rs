
#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen, TokenRightParen,
    TokenLeftBrace, TokenRightBrace,
    TokenComma, TokenDot, TokenMinus, TokenPlus,
    TokenSemicolon, TokenSlash, TokenStar,
    // One or two character tokens.
    TokenBang, TokenBangEqual,
    TokenEqual, TokenEqualEqual,
    TokenGreater, TokenGreaterEqual,
    TokenLess, TokenLessEqual,
    // Literals.
    TokenIdentifier, TokenString, TokenNumber,
    // Keywords.
    TokenAnd, TokenClass, TokenElse, TokenFalse,
    TokenFor, TokenFun, TokenIf, TokenNil, TokenOr,
    TokenPrint, TokenReturn, TokenSuper, TokenThis,
    TokenTrue, TokenVar, TokenWhile,
    // Others
    TokenError, TokenEof
}

pub struct Scanner<'a> {
    pub start: usize,
    pub current: usize,
    pub line: i32,
    pub source: &'a str,
}

#[derive(PartialEq)]
pub struct Token<'a> {
    pub type_: TokenType,
    // pub start: usize,
    // pub length: i32,
    pub content: &'a str,
    pub line: i32,
}

impl Scanner<'_> {
    pub fn new(s: &str) -> Scanner {
        return Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: s,
        }
    }
    
    pub fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
    
    fn make_content(&self) -> &str {
        return &self.source[self.start .. self.current];
    }
    
    pub fn advance(&self) -> char {
        '1'
    }
    
    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            return Token::make_token(TokenType::TokenEof, self.line, self.make_content());
        }
        
        let c = self.advance();
            
        match c {
            '(' => return Token::make_token(TokenType::TokenLeftParen, self.line, self.make_content()),
            ')' => return Token::make_token(TokenType::TokenRightParen, self.line, self.make_content()),
            '{' => return Token::make_token(TokenType::TokenLeftBrace, self.line, self.make_content()),
            '}' => return Token::make_token(TokenType::TokenRightBrace, self.line, self.make_content()),
            ';' => return Token::make_token(TokenType::TokenSemicolon, self.line, self.make_content()),
            ',' => return Token::make_token(TokenType::TokenComma, self.line, self.make_content()),
            '.' => return Token::make_token(TokenType::TokenDot, self.line, self.make_content()),
            '-' => return Token::make_token(TokenType::TokenMinus, self.line, self.make_content()),
            '+' => return Token::make_token(TokenType::TokenPlus, self.line, self.make_content()),
            '/' => return Token::make_token(TokenType::TokenSlash, self.line, self.make_content()),
            '*' => return Token::make_token(TokenType::TokenStar, self.line, self.make_content()),
            _ => {}
        }
        
        return Token::error_token("Unexpected character.", self.line);
    }
}

impl Token<'_> {
    pub fn error_token(message: &str, l: i32) -> Token {
        return Token {
            type_: TokenType::TokenError,
            content: message,
            line: l,
        }
    }
    
    pub fn make_token(t: TokenType, line: i32, content: &str) -> Token {
        return Token {
            type_: t,
            content: content,
            line: line,
        }
    }
    
    
}

