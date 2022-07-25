use crate::{
    error,
    token::{Token, TokenType},
};
use hexponent::FloatLiteral;

use unic_emoji_char as emoji;

pub struct Lexer {
    token_list: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.token_list
            .push(Token::new(TokenType::EOF, "", self.line));
        self.token_list.to_owned()
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
            '⧼' => self.add_token(TokenType::CodeBlockBegin),
            '⧽' => self.add_token(TokenType::CodeBlockEnd),
            '!' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            ':' => self.add_token(TokenType::Colon),
            '.' => self.add_token(TokenType::Dot),
            '<' => self.add_token(TokenType::LessThanSymbol),
            '>' => self.add_token(TokenType::GreaterThanSymbol),
            '\n' => self.line += 1,
            '`' => self.string(),
            '$' => self.function_agument(),
            c => {
                if c.is_alphabetic() || c == '-' {
                    if c == 't' || c == 'f' {
                        if !self.boolean() {
                            self.identifier();
                        }
                    } else if c == 'h' {
                        if !self.hempty() {
                            self.identifier();
                        }
                    } else {
                        self.identifier();
                    }
                } else if c.is_ascii_whitespace() {
                } else if c.is_ascii_digit() {
                    if self.peek() == 'x' {
                        self.advance();
                        self.start += 2
                    }
                    self.number();
                } else if emoji::is_emoji(c) {
                    self.add_unicode_token(TokenType::FunctionIdentifier { name: c });
                } else {
                    error::error(self.line, format!("uknown character {}", c));
                }
            }
        }
    }

    fn boolean(&mut self) -> bool {
        if self.peek() == 'r' {
            self.advance();
            if self.peek() == 'u' {
                self.advance();
                if self.peek() == 'e' {
                    self.advance();
                    if self.peek().is_alphanumeric() || self.peek() == '-' {
                        return false;
                    }
                    self.add_token(TokenType::Boolean { value: true });
                    return true;
                }
            }
        } else if self.peek() == 'a' {
            self.advance();
            if self.peek() == 'l' {
                self.advance();
                if self.peek() == 's' {
                    self.advance();
                    if self.peek() == 'e' {
                        self.advance();
                        if self.peek().is_alphanumeric() || self.peek() == '-' {
                            return false;
                        }
                        self.add_token(TokenType::Boolean { value: false });
                        return true;
                    }
                }
            }
        }
        false
    }

    fn hempty(&mut self) -> bool {
        if self.peek() == 'e' {
            self.advance();
            if self.peek() == 'm' {
                self.advance();
                if self.peek() == 'p' {
                    self.advance();
                    if self.peek() == 't' {
                        self.advance();
                        if self.peek() == 'y' {
                            self.advance();
                            if self.peek().is_alphanumeric() || self.peek() == '-' {
                                return false;
                            }
                            self.add_token(TokenType::Hempty);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn string(&mut self) {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error::error(self.line, "unterminated string");
        }

        self.advance();
        self.start += 1;
        self.current -= 1;
        let string = self.get_text();
        self.start -= 1;
        self.current += 1;
        self.add_token(TokenType::String { literal: string })
    }

    fn number(&mut self) {
        let hex_char = vec!['A', 'B', 'B', 'C', 'E', 'F'];

        while self.peek().is_ascii_digit() || hex_char.contains(&self.peek()) {
            self.advance();
        }
        if self.peek() == '.'
            && (self.peek_next().is_ascii_digit() || hex_char.contains(&self.peek_next()))
        {
            self.advance();
            while self.peek().is_ascii_digit() || hex_char.contains(&self.peek()) {
                self.advance();
            }
        }
        let number: FloatLiteral = format!("0x{}", self.get_text())
            .parse()
            .expect("could not parse number");
        self.add_token(TokenType::Number {
            literal: number.convert::<f64>().inner(),
        });
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '-' {
            self.advance();
        }
        self.add_token(
            crate::KEYWORDS
                .get(&self.get_text())
                .unwrap_or(TokenType::Identifier {
                    name: self.get_text(),
                }),
        );
    }

    fn function_agument(&mut self) {
        self.start += 1; // advance start past the $ so that we can parse it into a number
        let hex_char = vec!['A', 'B', 'C', 'D', 'E', 'F'];

        while self.peek().is_ascii_digit() || hex_char.contains(&self.peek()) {
            self.advance();
        }
        let identifier = format!(
            "${}",
            match format!("0x{}", self.get_text()).parse::<FloatLiteral>() {
                Ok(contents) => {
                    contents.convert::<f64>().inner()
                }
                Err(error) => {
                    error::error(self.line, error.to_string().as_str());
                }
            }
        );
        self.add_token(TokenType::Identifier { name: identifier })
    }

    fn advance(&mut self) -> char {
        self.current += 1;
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
        let text = format!("{}", self.source.chars().nth(self.start).expect("Error"));

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
