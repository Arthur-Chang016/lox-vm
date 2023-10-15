use crate::{scanner::{Scanner, TokenType, Token}, chunk::{Chunk, OpCode}, vm::VM};

pub struct Parser<'a> {
    pub current: Token<'a>,
    pub previous: Token<'a>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser<'_> {
    
    pub fn new() -> Parser<'static> {
        return Parser {
            current: Token::empty_token(),
            previous: Token::empty_token(),
            had_error: false,
            panic_mode: false,
        }
    }
    
    pub fn error_at_current(&mut self, message: &str) {
        self.error_at(&self.current, message);
        self.had_error = true;
    }
    
    pub fn error(&mut self,  message: &str) {
        self.error_at(&self.previous, message);
        self.had_error = true;
    }
    
    pub fn error_at(&self, token: &Token, message: &str) {
        eprint!("[line {}] Error", token.line);
        if token.type_ == TokenType::TokenEof {
            eprint!(" at end");
        } else if token.type_ == TokenType::TokenError {
            // Nothing
        } else {
            eprint!(" at '{}'", token.content);
        }
        eprintln!(": {message}");
        if self.panic_mode {
            return
        }
    }
    
}


pub fn advance<'a, 'b>(parser: &'a mut Parser<'b>, scanner: &'a mut Scanner<'b>) {
    parser.previous = parser.current;
    
    let scanner_ptr = scanner as *mut Scanner;
    let parser_ptr = parser as *mut Parser;
    
    loop {
        unsafe {
            let token = (*scanner_ptr).scan_token();
            (*parser_ptr).current = token;
        }
        
        if parser.current.type_ != TokenType::TokenError {
            break;
        }
        parser.error_at_current(parser.current.content);
    }
}

pub fn consume<'a, 'b>(parser: &'a mut Parser<'b>, type_: TokenType, message: &str, scanner: &'a mut Scanner<'b>) {
    if parser.current.type_ == type_ {
        advance(parser, scanner);
        return
    }
    parser.error_at_current(message);
}

pub fn compile(source: &str, chunk: &mut Chunk) -> bool {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new();
    advance(&mut parser, &mut scanner);
    expression();
    consume(&mut parser, TokenType::TokenEof, "Expect end of expression.", &mut scanner);
    end_compiler(chunk, parser.previous.line);
    return !parser.had_error;
}

pub fn expression() {
    
}

pub fn end_compiler(chunk: &mut Chunk, line: i32) {
    chunk.emit_return(line);
}

impl Chunk {
    pub fn emit_byte(&mut self, byte: u8, line: i32) {
        self.write_chunk(byte, line);
    }
    
    pub fn emit_bytes(&mut self, byte1: u8, line1: i32, byte2: u8, line2: i32) {
        self.write_chunk(byte1, line1);
        self.write_chunk(byte2, line2);
    }
    
    pub fn emit_return(&mut self, line: i32) {
        self.emit_byte(OpCode::OpReturn as u8, line);
    }
}


