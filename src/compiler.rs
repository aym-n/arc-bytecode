use crate::scanner::*;
use std::cell::RefCell;

pub struct Compiler {
    parser: Parser,
    scanner: Scanner,
}
#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            parser: Parser::default(),
            scanner: Scanner::new("".to_string()),
        }
    }
    pub fn compile(&mut self, source: String) -> bool {
        self.scanner = Scanner::new(source);
        self.advance();
        // self.expression();
        // self.consume(TokenType::EOF, "Expect end of expression.");
        *self.parser.had_error.borrow()
    }

    pub fn advance(&mut self) {
        self.parser.previous = self.parser.current.clone();
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(self.parser.current.start);
        }
    }

    fn error_at_current(&self, message: usize) {
        self.error_at(&self.parser.current, message);
    }

    fn error(&self, message: usize) {
        self.error_at(&self.parser.previous, message);
    }

    fn error_at(&self, token: &Token, message: usize){

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

    fn consume(&mut self, token_type: TokenType, message: usize) {
        if self.parser.current.token_type == token_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
}