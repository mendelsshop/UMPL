use crate::{
    error,
    keywords::Keyword,
    token::{Token, TokenType},
};

use unic_emoji_char as emoji;

pub struct Lexer {
    token_list: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: i32,
    keywords: Keyword,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
            keywords: Keyword::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.token_list
            .push(Token::new(TokenType::EOF, "", self.line));
        &self.token_list
    }

    fn is_at_end(&self) -> bool {
        (self.current) >= (self.source.chars().count())
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '[' => self.add_token(TokenType::LeftBracket),
            '}' => self.add_token(TokenType::RightBrace),
            ')' => self.add_token(TokenType::RightParen),
            ']' => self.add_token(TokenType::RightBracket),
            '!' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '.' => self.add_token(TokenType::Dot),
            '<' => self.add_token(TokenType::LessThan),
            '>' => self.add_token(TokenType::GreaterThan),
            ':' => self.add_token(TokenType::Colon),
            '\n' => self.line += 1,
            '`' => self.string(),
            c => {
                if c.is_alphabetic() {
                    self.identifier()
                } else if c.is_ascii_whitespace() {
                } else if c.is_ascii_digit() {
                    self.number();
                } else if emoji::is_emoji(c) {
                    self.add_unicode_token(TokenType::Identifier);
                } else {
                    error::error(self.line, "")
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(self.line, "")
        }

        self.advance();
        self.start+=1;
        self.current-=1;
        let string = self.get_text();
        self.start-=1;
        self.current+=1;
        self.add_token(TokenType::String { literal: string })
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let number: f64 = self
            .get_text()
            .to_string()
            .parse()
            .expect("could not parse number");
        self.add_token(TokenType::Number { literal: number });
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        self.add_token(
            self.keywords
                .get(&self.get_text())
                .unwrap_or(TokenType::Identifier)
                .clone(),
        );
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        // ;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source.chars();
        let mut final_text = String::new();
        for i in text.enumerate() {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        }
        self.token_list
            .push(Token::new(token_type, final_text.as_str(), self.line));
    }

    fn add_unicode_token(&mut self, token_type: TokenType) {
        let text = format!(
            "{}",
            self.source.chars().nth(self.start).expect("Error")
        );

        self.token_list
            .push(Token::new(token_type, &text, self.line));
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn get_text(&self) -> String {
        let text = self.source.chars();
        let mut final_text = String::new();
        for i in text.enumerate() {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        }
        final_text
    }
}