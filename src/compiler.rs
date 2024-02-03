use crate::chunk::*;
use crate::scanner::*;
use crate::token::*;
use crate::value::*;
use std::cell::RefCell;

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a mut Chunk,
}
#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None = 0,
    Assignment, // =
    Or,        // or
    And,      // and
    Equality, // == !=
    Comparison, // < > <= >=
    Term,     // + -
    Factor,  // * /
    Unary, // ! -
    Call, // . ()
    Primary, 
}

impl<'a> Compiler<'a> {
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
            let message = self.parser.current.lexeme.as_str();
            self.error_at_current(&message);
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

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type.clone();
        
        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into()),
            _ => {}
        }
    }
    fn parse_precedence(&mut self, precedence: Precedence) {
        unimplemented!()
    }
    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&mut self) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(value);
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        if let Some(constant) = self.chunk.add_constant(value) {
            constant
        } else {
            self.error("Too many constants in one chunk.");
            0
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }
    fn error_at(&self, token: &Token, message: &str) {
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
            eprint!(" at {}", token.lexeme);
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
