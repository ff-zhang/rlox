#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,
    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals
    Identifier, String, Number,
    // Keywords
    And, Class, Else, False,
    For, Fun, If, Nil, Or,
    Print, Return, Super, This,
    True, Var, While,

    Error, Eof
}

impl TokenType {
    pub fn identifier(scanner: &Scanner) -> TokenType {
        match scanner.source[0] as char {
            'a' => return TokenType::check_keyword(&scanner.source[1..3], "nd", TokenType::And),
            'c' => return TokenType::check_keyword(&scanner.source[1..5], "lass", TokenType::Class),
            'e' => return TokenType::check_keyword(&scanner.source[1..4], "lse", TokenType::Else),
            'f' => if scanner.current > 1 {
                match scanner.source[1] as char {
                    'a' => return TokenType::check_keyword(&scanner.source[2..5], "lse", TokenType::False),
                    'o' => return TokenType::check_keyword(&scanner.source[2..3], "r", TokenType::For),
                    'u' => return TokenType::check_keyword(&scanner.source[2..3], "n", TokenType::Fun),
                    _ => {},
                }
            },
            'i' => return TokenType::check_keyword(&scanner.source[1..2], "f", TokenType::If),
            'n' => return TokenType::check_keyword(&scanner.source[1..3], "il", TokenType::Nil),
            'o' => return TokenType::check_keyword(&scanner.source[1..2], "r", TokenType::Or),
            'p' => return TokenType::check_keyword(&scanner.source[1..5], "rint", TokenType::Print),
            'r' => return TokenType::check_keyword(&scanner.source[1..6], "eturn", TokenType::Return),
            's' => return TokenType::check_keyword(&scanner.source[1..5], "uper", TokenType::Super),
            't' => if scanner.current > 1 {
                match scanner.source[1] as char {
                    'h' => return TokenType::check_keyword(&scanner.source[2..4], "is", TokenType::This),
                    'r' => return TokenType::check_keyword(&scanner.source[2..4], "ue", TokenType::True),
                    _ => {},
                }
            },
            'v' => return TokenType::check_keyword(&scanner.source[1..3], "ar", TokenType::Var),
            'w' => return TokenType::check_keyword(&scanner.source[1..5], "hile", TokenType::While),
            _ => {},
        }

        TokenType::Identifier
    }

    fn check_keyword(slice: &[u8], rest: &str, kind: TokenType) -> TokenType {
        if slice == rest.as_bytes() { kind } else { TokenType::Identifier }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub slice: &'a [u8],
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(scanner: &Scanner<'a>, kind: TokenType) -> Token<'a> {
        Token {
            kind,
            slice: &scanner.source[..scanner.current],
            line: scanner.line,
        }
    }

    pub fn error(scanner: &Scanner, message: &'a [u8]) -> Token<'a> {
        Token {
            kind: TokenType::Error,
            slice: message,
            line: scanner.line,
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source: &'a [u8],
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Scanner {
            source,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        self.source = &self.source[self.current..];
        self.current = 0;
        if self.is_at_end() {
            return Token::new(&self, TokenType::Eof);
        }

        let c = self.advance();
        if c.is_alphabetic() || c == '_' { return self.identifier(); }
        if c.is_digit(10) { return self.number(); }

        match c  {
            '(' => return Token::new(&self, TokenType::LeftParen),
            ')' => return Token::new(&self, TokenType::RightParen),
            '{' => return Token::new(&self, TokenType::LeftBrace),
            '}' => return Token::new(&self, TokenType::RightBrace),
            ';' => return Token::new(&self, TokenType::Semicolon),
            ',' => return Token::new(&self, TokenType::Comma),
            '.' => return Token::new(&self, TokenType::Dot),
            '-' => return Token::new(&self, TokenType::Minus),
            '+' => return Token::new(&self, TokenType::Plus),
            '/' => return Token::new(&self, TokenType::Slash),
            '*' => return Token::new(&self, TokenType::Star),
            '!' => {
                let kind = if self.compare('=') { TokenType::BangEqual } else { TokenType::Bang };
                return Token::new(&self, kind)
            },
            '=' => {
                let kind = if self.compare('=') { TokenType::EqualEqual } else { TokenType::Equal };
                return Token::new(&self, kind)
            },
            '<' => {
                let kind = if self.compare('=') { TokenType::LessEqual } else { TokenType::Less };
                return Token::new(&self, kind)
            },
            '>' => {
                let kind = if self.compare('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                return Token::new(&self, kind)
            },
            '"' => return self.string(),
            _ => {},
        }

        Token::error(&self, "Unexpected character".as_bytes())
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                ' ' | '\r' | '\t' => { let _ = self.advance(); },
                '\n' => {
                    self.line += 1;
                    let _ = self.advance();
                },
                '/' => if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() { let _ = self.advance(); }
                } else { return; }
                _ => return,
            }
        }
    }

    fn identifier(&mut self) -> Token<'a> {
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
        Token::new(self, TokenType::identifier(&self))
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().is_digit(10) { self.advance(); }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) { self.advance(); }
        }

        Token::new(self, TokenType::Number)
    }

    fn string(&mut self) -> Token<'a> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() { return Token::error(self, "Unterminated string".as_bytes()); }

        // The closing quote
        self.advance();
        Token::new(&self, TokenType::String)
    }

    fn is_at_end(&self) -> bool {
        self.source.is_empty() || self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1] as char
    }

    fn compare(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source[self.current] as char != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.current < self.source.len() { self.source[self.current] as char } else { '\0' }
    }

    fn peek_next(&self) -> char {
        if self.current < self.source.len() - 1 { self.source[self.current + 1] as char } else { '\0' }
    }
}
