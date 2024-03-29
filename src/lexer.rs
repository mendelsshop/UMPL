use crate::{
    error::{self, error},
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
    name: String,
}

impl Lexer {
    pub const fn new(source: String, name: String) -> Self {
        Self {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
            name,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.token_list
            .push(Token::new(TokenType::EOF, "", self.line, &self.name));
        self.token_list
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
            '*' => self.add_token(TokenType::Star),
            c => {
                if c.is_lowercase() || c == '-' {
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
                        self.start += 2;
                    }
                    self.number();
                } else if emoji::is_emoji(c) {
                    self.add_unicode_token(TokenType::FunctionIdentifier {
                        name: c,
                        path: vec![],
                    });
                } else {
                    error::error(self.line, format!("uknown character {c}"));
                }
            }
        }
    }

    fn boolean(&mut self) -> bool {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        if self.get_text() == "true" {
            self.add_token(TokenType::Boolean { literal: true });
            return true;
        } else if self.get_text() == "false" {
            self.add_token(TokenType::Boolean { literal: false });
            return true;
        }
        false
    }

    fn hempty(&mut self) -> bool {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        if self.get_text() == "hempty" {
            self.add_token(TokenType::Hempty);
            return true;
        }
        false
    }

    #[allow(clippy::too_many_lines)]
    fn string(&mut self) {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            // check for escape sequence \` \n \\ \t \r \a \b \f \v \e \Xhh \0ooo \Uhhhhhhhh
            if self.peek() == '\\' {
                self.remove_text(self.current);
                if self.peek() == '`' {
                    self.advance();
                } else if self.peek() == 'n' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\n');
                    self.advance();
                } else if self.peek() == '\\' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\\');
                    self.advance();
                } else if self.peek() == 't' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\t');
                    self.advance();
                } else if self.peek() == 'r' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\r');
                    self.advance();
                } else if self.peek() == 'a' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\x07');
                    self.advance();
                } else if self.peek() == 'b' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\x08');
                    self.advance();
                } else if self.peek() == 'f' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\x0C');
                    self.advance();
                } else if self.peek() == 'v' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\x0b');
                    self.advance();
                } else if self.peek() == 'e' {
                    self.remove_text(self.current);
                    self.insert_text(self.current, '\x1b');
                    self.advance();
                } else if self.peek() == 'x' {
                    self.remove_text(self.current);
                    match self.remove_text(self.current) {
                        x if x.is_ascii_hexdigit() => match self.remove_text(self.current) {
                            y if y.is_ascii_hexdigit() => {
                                self.insert_text(
                                    self.current,
                                    u8::from_str_radix(format!("{x}{y}").as_str(), 16)
                                        .unwrap_or_else(|_| {
                                            error::error(self.line, "invalid hex escape sequence");
                                        }) as char,
                                );
                                self.advance();
                            }
                            y => {
                                self.insert_text(
                                    self.current,
                                    u8::from_str_radix(format!("{x}").as_str(), 16).unwrap_or_else(
                                        |_| {
                                            error::error(self.line, "invalid hex escape sequence");
                                        },
                                    ) as char,
                                );
                                self.insert_text(self.current + 1, y);
                                self.advance();
                            }
                        },
                        x => {
                            self.insert_text(self.current, x);
                            self.advance();
                        }
                    }
                } else if self.peek() == 'u' {
                    self.remove_text(self.current);
                    // can be from 0 to 6 hex digits
                    // loop until either 6 digits or we find a digit a non-hex digit
                    // while looping remove the character and append to a string
                    let mut hex_string = String::new();
                    let mut i = 0;
                    while i < 6 && self.peek().is_ascii_hexdigit() {
                        hex_string.push(self.remove_text(self.current));
                        i += 1;
                    }
                    self.insert_text(
                        self.current,
                        char::from_u32(
                            u32::from_str_radix(hex_string.as_str(), 16).unwrap_or_else(|_| {
                                error::error(self.line, "invalid unicode escape sequence");
                            }),
                        )
                        .unwrap_or_else(|| {
                            error::error(self.line, "invalid unicode escape sequence");
                        }),
                    );
                    self.current += 1;
                } else {
                    error::error(
                        self.line,
                        format!("unknown escape sequence {}", self.peek()),
                    );
                }
            } else {
                self.advance();
            }
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
        self.add_token(TokenType::String { literal: string });
    }

    fn number(&mut self) {
        while self.peek().is_ascii_hexdigit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_hexdigit() {
            self.advance();
            while self.peek().is_ascii_hexdigit() {
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
        while self.peek().is_lowercase() || self.peek() == '-' || self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '$' {
            let first_path = self.get_text().pop();
            // we can assume that the character is ascci and can use .len on string
            if self.get_text().len() != 1 || !first_path.unwrap().is_alphabetic() {
                error(self.line, format!("Module path has more than one character and or is non alphabetic, given module path: {}", self.get_text()));
            };
            let mut path = vec![first_path.unwrap()];
            self.advance();

            // see if the next character is an emoji
            while self.peek().is_lowercase() && self.peek_next() == '$' {
                path.push(self.advance());
                self.advance();
            }
            if emoji::is_emoji(self.peek()) {
                let name = self.advance();
                // maybe need to rework lexer so don't have to split after already done lexing
                self.add_unicode_token(TokenType::FunctionIdentifier { name, path });
            } else {
                // error out
                error::error(
                    self.line,
                    format!("Unexpected character after $ {}", self.peek()),
                );
            }
        } else {
            self.add_token(crate::KEYWORDS.get(&self.get_text()).unwrap_or(
                TokenType::Identifier {
                    name: self.get_text(),
                },
            ));
        }
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
        self.add_token(TokenType::Identifier { name: identifier });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: std::str::Chars<'_> = self.source.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        self.token_list.push(Token::new(
            token_type,
            final_text.as_str(),
            self.line,
            &self.name,
        ));
    }

    fn add_unicode_token(&mut self, token_type: TokenType) {
        let text: String = format!("{}", self.source.chars().nth(self.start).expect("Error"));
        self.token_list
            .push(Token::new(token_type, &text, self.line, &self.name));
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn get_text(&self) -> String {
        let text: std::str::Chars<'_> = self.source.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        final_text
    }

    fn remove_text(&mut self, pos: usize) -> char {
        // use chars to be able to use unicode, remove the pos
        let mut text: Vec<char> = self.source.chars().collect();
        let to_return = text[pos];
        text.remove(pos);
        self.source = text.iter().collect();
        to_return
    }

    fn insert_text(&mut self, pos: usize, texts: char) {
        let mut text: Vec<char> = self.source.chars().collect();
        text.insert(pos, texts);
        self.source = text.iter().collect();
    }
}
