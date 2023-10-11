
use std::{borrow::BorrowMut, rc::Rc, cell::RefCell};

use crate::{scanner::{Scanner, TokenType, Token}, chunk::Chunk};

// struct Parser<'a> {
//     pub current: Token<'a>,
//     pub previous: Token<'a>,
//     pub had_error: bool,
//     pub panic_mode: bool,
//     pub scanner: &'a mut Scanner<'a>,
// }

pub struct Parser<'a> {
    pub current: Token<'a>,
    pub previous: Token<'a>,
    pub had_error: bool,
    pub panic_mode: bool,
    // pub scanner: Rc<RefCell<Scanner<'a>>>,
    // pub scanner: Scanner<'a>,
}

impl Parser<'_> {
    // pub fn new(scanner: Rc<RefCell<Scanner<'a>>>) -> Parser {
    //     Parser {
    //         current: Token::empty_token(),
    //         previous: Token::empty_token(),
    //         had_error: false,
    //         panic_mode: false,
    //         scanner,
    //     }
    // }
    
    
    // pub fn new(sc: Scanner) -> Parser {
    //     Parser {
    //         current: Token::empty_token(),
    //         previous: Token::empty_token(),
    //         had_error: false,
    //         panic_mode: false,
    //         scanner: Box::new(sc),
    //     }
    // }
    
    // pub fn new<'a>(scanner: &'a mut Scanner) -> Parser<'a> {
    //     return Parser {
    //         current: Token::empty_token(),
    //         previous: Token::empty_token(),
    //         had_error: false,
    //         panic_mode: false,
    //         scanner: scanner,
    //     }
    // }
    
    
    pub fn new() -> Parser<'static> {
        return Parser {
            current: Token::empty_token(),
            previous: Token::empty_token(),
            had_error: false,
            panic_mode: false,
            // scanner: Scanner::new(s),
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
    
    // pub fn advance<'a, 'b>(self: &'a mut Parser<'b>, scanner: &'a mut Scanner) {
    //     self.previous = self.current;
        
    //     let scanner_ptr = scanner as *mut Scanner;
    //     let parser_ptr = self as *mut Parser;
        
    //     loop {
    //         unsafe {
    //             let token = (*scanner_ptr).scan_token();
    //             (*parser_ptr).current = token;
    //         }
            
    //         if self.current.type_ != TokenType::TokenError {
    //             break;
    //         }
    //         self.error_at_current(self.current.content);
    //     }
    // }
    
    // pub fn consume(&mut self, type_: TokenType, message: &str, scanner: &mut Scanner) {
    //     if self.current.type_ == type_ {
    //         // self.advance(scanner);
    //         advance(self, scanner);
    //         return
    //     }
    //     self.error_at_current(message);
    // }
    
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



// static mut scanner: Scanner = Scanner::new("");
// static mut SCANNER: Option<Scanner> = None;
// static STR_REF: &str = "";

pub fn compile(source: &str, chunk: &Chunk) -> bool {
    let mut scanner = Scanner::new(source);
    // let scanner = Box::new(Scanner::new(source));
    // let mut scanner = Rc::new(RefCell::new(Scanner::new(source)));
    
    // let source_cpy = source.
    // STR_REF = source;
    // unsafe {  // init scanner
    //     SCANNER = Some(Scanner::new(source));
        
    //     // scanner = Scanner::new(source);
    // };
    
    // scanner.
    // let mut parser = Parser::new(scanner.clone());
    let mut parser = Parser::new();
    // parser.advance(&mut scanner);
    advance(&mut parser, &mut scanner);
    // expression();
    // parser.consume(TokenType::TokenEof, "Expect end of expression.", &mut scanner);
    consume(&mut parser, TokenType::TokenEof, "Expect end of expression.", &mut scanner);
    return !parser.had_error;
    
    // let mut line = -1;
    // loop {
    //     let token = scanner.scan_token();
    //     if token.line != line {
    //         print!("{:4}", token.line);
    //         line = token.line;
    //     } else {
    //         print!("   | ");
    //     }
    //     println!("{:2} '{}'\n", token.type_ as i32, token.content);
        
    //     if token.type_ == TokenType::TokenEof {
    //         break;
    //     }
    // }
}