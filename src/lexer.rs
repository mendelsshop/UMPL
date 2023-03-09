use crate::{
    error,
    token::{Info, Token, TokenType},
};
use hexponent::FloatLiteral;

use unic_emoji_char as emoji;
pub struct Lexer<'a> {
    token_list: Vec<Token<'a>>,
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
    module: Option<String>,
    name: &'a str,
    text_buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, name: &'a str) -> Self {
        Self {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
            module: None,
            name,
            text_buffer: source.to_string(),
        }
    }

    pub const fn get_source(&self) -> &'a str {
        self.source
    }

    pub const fn get_info(&self) -> Info<'a> {
        Info {
            line: self.line,
            end_line: self.line,
            file_name: self.name,
        }
    }

    pub fn set_module(&mut self, module: String) {
        if module.chars().count() > 1 {
            error::error(
                self.line,
                format!("Module name must be a single character, got {module}").as_str(),
            );
        }
        self.module = Some(module);
    }

    pub fn scan_tokens(mut self) -> Vec<Token<'a>> {
        let mut at_end = self.is_at_end();
        while !at_end {
            self.start = self.current;
            if let Some(t) = self.scan_token() {
                self.token_list.push(t);
            }
            at_end = self.is_at_end();
        }
        let info = self.get_info();

        self.token_list.push(Token::new(TokenType::EOF, "", info));
        self.token_list
    }

    fn is_at_end(&self) -> bool {
        (self.current) >= (self.text_buffer.chars().count())
    }

    fn scan_token<'b>(&mut self) -> Option<Token<'b>>
    where
        'a: 'b,
    {
        let c: char = self.advance();
        match c {
            '(' => Some(self.add_token(TokenType::LeftParen)),
            '{' => Some(self.add_token(TokenType::LeftBrace)),
            '[' => Some(self.add_token(TokenType::LeftBracket)),
            '}' => Some(self.add_token(TokenType::RightBrace)),
            ')' => Some(self.add_token(TokenType::RightParen)),
            ']' => Some(self.add_token(TokenType::RightBracket)),
            '⧼' => Some(self.add_token(TokenType::CodeBlockBegin)),
            '⧽' => Some(self.add_token(TokenType::CodeBlockEnd)),
            '!' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
            }
            ':' => Some(self.add_token(TokenType::Colon)),
            '.' => Some(self.add_token(TokenType::Dot)),
            '<' => Some(self.add_token(TokenType::LessThanSymbol)),
            '>' => Some(self.add_token(TokenType::GreaterThanSymbol)),
            '\n' => {
                self.line += 1;
                None
            }
            '`' => Some(self.string()),
            '$' => Some(self.function_agument()),
            '*' => Some(self.add_token(TokenType::Star)),
            '?' => Some(self.add_token(TokenType::QuestionMark)),
            c => {
                if c.is_lowercase() || c == '-' {
                    if c == 't' || c == 'f' {
                        self.boolean().map_or_else(|| self.identifier(), Some)
                    } else if c == 'h' {
                        self.hempty().map_or_else(|| self.identifier(), Some)
                    } else {
                        self.identifier()
                    }
                } else if c.is_ascii_whitespace() {
                    None
                } else if c.is_ascii_digit() {
                    if self.peek() == 'x' {
                        self.advance();
                        self.start += 2;
                    }
                    Some(self.number())
                } else if emoji::is_emoji(c) {
                    let c = self
                        .module
                        .as_ref()
                        .map_or_else(|| c.to_string(), |module| format!("{module}+{c}"));
                    Some(self.add_unicode_token(TokenType::FunctionIdentifier(c)))
                } else {
                    error::error(self.line, format!("uknown character {c}"));
                }
            }
        }
    }

    fn boolean(&mut self) -> Option<Token<'a>> {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        if self.get_text() == "true" {
            Some(self.add_token(TokenType::Boolean(true)))
        } else if self.get_text() == "false" {
            Some(self.add_token(TokenType::Boolean(false)))
        } else {
            None
        }
    }

    fn hempty(&mut self) -> Option<Token<'a>> {
        while self.peek().is_alphabetic() {
            self.advance();
        }
        if self.get_text() == "hempty" {
            Some(self.add_token(TokenType::Hempty))
        } else {
            None
        }
    }

    #[allow(clippy::too_many_lines)]
    fn string(&mut self) -> Token<'a> {
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
        self.add_token(TokenType::String(string))
    }

    fn number(&mut self) -> Token<'a> {
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
        self.add_token(TokenType::Number(number.convert::<f64>().inner()))
    }

    fn identifier(&mut self) -> Option<Token<'a>> {
        if !self
            .get_text()
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '-') && !self.get_text().contains('+') {
            error::error(self.line, format!("invalid identifier {}", self.get_text()));
        }
        while self.peek().is_lowercase() || self.peek() == '-' || self.peek().is_numeric() {
            self.advance();
        }
        if self.peek() == '+' {
            self.advance();
            // see if the next character is an emoji
            while self.peek().is_lowercase() && self.peek_next() == '+' {
                self.advance();
                self.advance();
            }
            if emoji::is_emoji(self.peek()) {
                self.advance();
                let name = self.get_text();
                #[allow(clippy::needless_collect)]
                let parts = name
                    .chars()
                    .map(|char_part| match char_part {
                        '+' => self.add_token(TokenType::PlusSymbol),
                        module if module.is_ascii_lowercase() => {
                            self.add_token(TokenType::ModuleIdentifier(char_part))
                        }
                        function if emoji::is_emoji(function) => self
                            .add_unicode_token(TokenType::FunctionIdentifier(function.to_string())),
                        char => error::error(self.line, format!("{char} not allowed")),
                    })
                    // need to collect becuase both iterators use self
                    .collect::<Vec<_>>();
                parts.into_iter().for_each(|p| self.token_list.push(p));
                None
            } else {
                // error out
                error::error(
                    self.line,
                    format!("Unexpected character after + {}", self.peek()),
                );
            }
        } else {
            let text = self.get_text();
            if let Some(token) = crate::KEYWORDS.string_is_builtin_function(&text) {
                Some(self.add_token(TokenType::BuiltinFunction(token)))
            } else if let Some(token) = crate::KEYWORDS.string_is_keyword(&text) {
                Some(self.add_token(token))
            } else {
                Some(self.add_token(TokenType::Identifier(text)))
            }
        }
    }

    fn function_agument(&mut self) -> Token<'a> {
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
        self.add_token(TokenType::Identifier(identifier))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.text_buffer.chars().collect();
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType<'a>) -> Token<'a> {
        let text: std::str::Chars<'_> = self.text_buffer.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        Token::new(token_type, final_text.as_str(), self.get_info())
    }

    fn add_unicode_token(&mut self, token_type: TokenType<'a>) -> Token<'a> {
        let text: String = format!(
            "{}",
            self.text_buffer.chars().nth(self.start).expect("Error")
        );
        Token::new(token_type, &text, self.get_info())
    }

    fn peek(&self) -> char {
        self.text_buffer.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.text_buffer
            .chars()
            .nth(self.current + 1)
            .unwrap_or('\0')
    }

    fn get_text(&self) -> String {
        let text: std::str::Chars<'_> = self.text_buffer.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        final_text
    }

    fn insert_text(&mut self, pos: usize, text: char) {
        let mut text_list: Vec<char> = self.text_buffer.chars().collect();
        text_list.insert(pos, text);
        self.text_buffer = text_list.iter().collect();
    }

    fn remove_text(&mut self, pos: usize) -> char {
        let mut text_list: Vec<char> = self.text_buffer.chars().collect();
        let t = text_list.remove(pos);
        self.text_buffer = text_list.iter().collect();
        t
    }
}
