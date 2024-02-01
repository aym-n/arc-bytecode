use crate::chunk;
use crate::scanner::*;
use crate::chunk::*;
use std::cell::RefCell;

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a  mut Chunk,
}
#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

impl <'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            parser: Parser::default(),
            scanner: Scanner::new("".to_string()),
            chunk,
        }
    }
    pub fn compile(&mut self, source: String) -> bool {
        self.scanner = Scanner::new(source);
        self.advance();
        // self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.");
        self.end_compiler();
        *self.parser.had_error.borrow()
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(&self.scanner.source[self.parser.current.start..self.parser.current.length]);
        }
    }

    fn error_at_current(&self, message: &str) {
        self.error_at(&self.parser.current, message);
    }

    fn error(&self, message: &str) {
        self.error_at(&self.parser.previous, message);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }
    fn error_at(&self, token: &Token, message: &str){

        if *self.parser.panic_mode.borrow() {
            return;
        }
        self.parser.panic_mode.replace(true);

        eprintln!("[line {}] Error", token.line);
        if token.token_type == TokenType::EOF {
            eprint!(" at end");
        } else if token.token_type == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at {}", self.scanner.source[token.start..token.length].to_string());
        }

        eprintln!(": {}", message);
        self.parser.had_error.replace(true);
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: u8) {
        self.emit_byte(byte1 as u8);
        self.emit_byte(byte2);
    }
}