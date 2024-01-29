pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token{
        self.start = self.current;
        
        if self.is_at_end() {
            return Token::new(TokenType::EOF, self);
        }

        return Token::error_token("Unexpected character.", self);
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current) == Some('\0')
    }
}

pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
}

pub enum TokenType {
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    Identifier, String, Number,

    And, Class, Else, False,
    Fn, For, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, EOF,
}

impl Token {
    pub fn new(token_type: TokenType, scanner:&mut Scanner) -> Self {
        Self {
            token_type,
            start: scanner.start,
            length: (scanner.current - scanner.start),
            line: scanner.line,
        }
    }

    pub fn error_token(message: &str, scanner: &mut Scanner) -> Self {
        Self {
            token_type: TokenType::Error,
            start: 0,
            length: message.len(),
            line: scanner.line,
        }
    }
}
