use crate::scanner::*;
use crate::vm::*;

pub struct Parser {
    current: Token,
    previous: Token,
    scanner: Scanner,
}

impl Parser{

    pub fn advance(&mut self){
        self.previous = self.current;
        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }
            self.error_at_current(self.current.start);
        }
    }

}

pub fn compile(source: String) -> bool{
    let mut scanner = Scanner::new(source);
    scanner.advance();
    scanner.expression();
    scanner.consume(TokenType::EOF, "Expect end of expression.");
    true
}
