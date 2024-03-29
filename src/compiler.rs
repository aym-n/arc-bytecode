use crate::chunk::*;
use crate::scanner::*;
use crate::token;
use crate::token::*;
use crate::value::*;
use std::cell::RefCell;

pub struct Compiler<'a> {
    parser: Parser,
    scanner: Scanner,
    chunk: &'a mut Chunk,
    rules: Vec<ParseRule<'a>>,
}

#[derive(Copy, Clone)]
struct ParseRule<'a> {
    prefix: Option<fn(&mut Compiler<'a>, bool)>,
    infix: Option<fn(&mut Compiler<'a>, bool)>,
    precedence: Precedence,
}

#[derive(Default)]
pub struct Parser {
    current: Token,
    previous: Token,
    had_error: RefCell<bool>,
    panic_mode: RefCell<bool>,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum Precedence {
    None = 0,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl From<usize> for Precedence {
    fn from(v: usize) -> Self {
        match v {
            0 => Precedence::None,
            1 => Precedence::Assignment,
            2 => Precedence::Or,
            3 => Precedence::And,
            4 => Precedence::Equality,
            5 => Precedence::Comparison,
            6 => Precedence::Term,
            7 => Precedence::Factor,
            8 => Precedence::Unary,
            9 => Precedence::Call,
            10 => Precedence::Primary,
            v => panic!("cannot convert {v} into Precedence"),
        }
    }
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        let mut rules = vec![
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            };
            TokenType::Undefined as usize + 1
        ];

        rules[TokenType::LeftParen as usize].prefix = Some(Compiler::grouping);

        rules[TokenType::Minus as usize] = ParseRule {
            prefix: Some(Compiler::unary),
            infix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Term,
        };

        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Factor,
        };

        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(Compiler::number),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::True as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Nil as usize] = ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Bang as usize] = ParseRule {
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::BangEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Equality,
        };

        rules[TokenType::EqualEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Equality,
        };

        rules[TokenType::Greater as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::GreaterEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::Less as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::LessEqual as usize] = ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: Precedence::Comparison,
        };

        rules[TokenType::String as usize] = ParseRule {
            prefix: Some(Compiler::string),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::Identifier as usize] = ParseRule {
            prefix: Some(Compiler::variable),
            infix: None,
            precedence: Precedence::None,
        };

        Self {
            parser: Parser::default(),
            scanner: Scanner::new("".to_string()),
            chunk,
            rules,
        }
    }
    pub fn compile(&mut self, source: String) -> bool {
        self.scanner = Scanner::new(source);
        self.advance();

        while !self.matches(TokenType::EOF) {
            self.declaration();
        }

        self.end_compiler();
        *self.parser.had_error.borrow()
    }

    fn matches(&mut self, token_type: TokenType) -> bool {
        if !(self.parser.current.token_type == token_type) {
            return false;
        }
        self.advance();
        true
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

        if !*self.parser.had_error.borrow() {
            self.chunk.disassemble("code");
        }
    }

    fn grouping(&mut self, _: bool) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self, _: bool) {
        let operator_type = self.parser.previous.token_type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::OpNot.into()),
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into()),
            _ => {}
        }
    }
    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        if let Some(prefix_rule) = self.rules[self.parser.previous.token_type as usize].prefix {
            let can_assign = precedence <= Precedence::Assignment;
            prefix_rule(self, can_assign);

            while precedence <= self.rules[self.parser.current.token_type as usize].precedence {
                self.advance();
                if let Some(infix_rule) = self.rules[self.parser.previous.token_type as usize].infix
                {
                    infix_rule(self, can_assign);
                }
            }
        } else {
            self.error("Expect expression.");
            return;
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn declaration(&mut self) {
        if self.matches(TokenType::Var) {
            self.var_declaration();
        } else {
            self.statement();
        }
        if *self.parser.panic_mode.borrow() {
            self.synchronize();
        }
    }

    fn named_variable(&mut self, name: &Token, can_assign: bool) {
        let arg = self.identifier_constant(name);
        if can_assign && self.matches(TokenType::Equal) {
            self.expression();
            self.emit_bytes(OpCode::OpSetGlobal.into(), arg);
        } else {
            self.emit_bytes(OpCode::OpGetGlobal.into(), arg);
        }
    }

    fn variable(&mut self, can_assign: bool) {
        let name = self.parser.previous.clone();
        self.named_variable(&name, can_assign);
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.matches(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpNil.into());
        }

        self.consume(TokenType::Semicolon, "Expect ; after variable declaration");
        self.define_variable(global);
    }

    fn parse_variable(&mut self, message: &str) -> u8 {
        self.consume(TokenType::Identifier, message);
        let token = self.parser.previous.clone();
        return self.identifier_constant(&token);
    }

    fn identifier_constant(&mut self, token: &Token) -> u8 {
        self.make_constant(Value::Str(token.lexeme.clone()))
    }

    fn define_variable(&mut self, global: u8) {
        self.emit_bytes(OpCode::OpDefineGlobal.into(), global);
    }

    fn synchronize(&mut self) {
        self.parser.panic_mode.replace(false);

        while self.parser.current.token_type != TokenType::EOF {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.parser.current.token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }

    fn statement(&mut self) {
        if self.matches(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ; after Expression");
        self.emit_byte(OpCode::OpPop.into());
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::OpPrint.into());
    }

    fn number(&mut self, _: bool) {
        let value = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn literal(&mut self, _: bool) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OpFalse.into()),
            TokenType::True => self.emit_byte(OpCode::OpTrue.into()),
            TokenType::Nil => self.emit_byte(OpCode::OpNil.into()),
            _ => return,
        }
    }

    fn string(&mut self, _: bool) {
        let len = self.parser.previous.lexeme.len() - 1;
        let string = self.parser.previous.lexeme[1..len].to_string();
        self.emit_constant(Value::Str(string));
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant.into(), constant);
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

    fn binary(&mut self, _: bool) {
        let operator_type = self.parser.previous.token_type.clone();
        let rule = self.rules[operator_type as usize].clone();

        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::OpAdd.into()),
            TokenType::Minus => self.emit_byte(OpCode::OpSubtract.into()),
            TokenType::Star => self.emit_byte(OpCode::OpMultiply.into()),
            TokenType::Slash => self.emit_byte(OpCode::OpDivide.into()),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater.into()),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual.into()),
            TokenType::Less => self.emit_byte(OpCode::OpLess.into()),
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual.into(), OpCode::OpNot.into()),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess.into(), OpCode::OpNot.into()),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater.into(), OpCode::OpNot.into()),
            _ => todo!(),
        }
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

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }
}

impl Precedence {
    fn next(self) -> Self {
        if self == Precedence::Primary {
            panic!("no next after Primary");
        }
        let p = self as usize;
        (p + 1).into()
    }

    fn previous(self) -> Self {
        if self == Precedence::None {
            panic!("no previous before None");
        }
        let p = self as usize;
        (p - 1).into()
    }
}
