pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    pub line: usize,
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

        self.skip_whitespace();

        self.start = self.current;
        
        if self.is_at_end() {
            return Token::new(TokenType::EOF, self);
        }

        let c = self.advance();
        match c {
            '(' => return Token::new(TokenType::LeftParen, self),
            ')' => return Token::new(TokenType::RightParen, self),
            '{' => return Token::new(TokenType::LeftBrace, self),
            '}' => return Token::new(TokenType::RightBrace, self),
            ';' => return Token::new(TokenType::Semicolon, self),
            ',' => return Token::new(TokenType::Comma, self),
            '.' => return Token::new(TokenType::Dot, self),
            '-' => return Token::new(TokenType::Minus, self),
            '+' => return Token::new(TokenType::Plus, self),
            '/' => return Token::new(TokenType::Slash, self),
            '*' => return Token::new(TokenType::Star, self),

            '!' => {
                if self.match_char('=') {
                    return Token::new(TokenType::BangEqual, self);
                }else{
                    return Token::new(TokenType::Bang, self);
                }
            }

            '=' => {
                if self.match_char('=') {
                    return Token::new(TokenType::EqualEqual, self);
                }else{
                    return Token::new(TokenType::Equal, self);
                }
            }

            '<' => {
                if self.match_char('=') {
                    return Token::new(TokenType::LessEqual, self);
                }else{
                    return Token::new(TokenType::Less, self);
                }
            }

            '>' => {
                if self.match_char('=') {
                    return Token::new(TokenType::GreaterEqual, self);
                }else{
                    return Token::new(TokenType::Greater, self);
                }
            }

            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    return Token::error_token("Unterminated string.", self);
                }

                self.advance();
                return Token::new(TokenType::String, self);
            }

            '0'..='9' => {
                while self.peek().is_digit(10) {
                    self.advance();
                    
                    if self.peek() == '.' && self.peek_next().is_digit(10) {
                        self.advance();

                        while self.peek().is_digit(10) {
                            self.advance();
                        }
                    }
                }

                return Token::new(TokenType::Number, self);
            }

            _ =>  return Token::error_token("Unexpected character.", self),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current) == Some('\0')
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self){
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    }else{
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }



}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

#[derive(PartialEq, Debug)]
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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} ", self)
    }
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
