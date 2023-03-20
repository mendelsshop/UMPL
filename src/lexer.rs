use crate::{
    error::error,
    token::{Info, Position, Token, TokenType},
};
use hexponent::FloatLiteral;

use unic_emoji_char as emoji;
#[derive(Debug, PartialEq)]
pub struct Lexer<'a, 'b> {
    token_list: Vec<Token<'b, 'a>>,
    source: &'a str,
    start: usize,
    current: usize,
    line: u32,
    module: Vec<char>,
    name: &'a str,
    text_buffer: &'b mut &'b String,
    colunm: u32,
}

impl<'a, 'b> Lexer<'a, 'b> {
    pub fn new(source: &'b str, name: &'a str, text_buffer: &'b mut &'b String) -> Self
    where
        'b: 'a,
    {
        Self {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
            module: Vec::new(),
            name,
            text_buffer,
            colunm: 1,
        }
    }

    pub const fn get_source(&self) -> &'a str {
        self.source
    }

    pub const fn get_info(&self) -> Info<'a> {
        Info {
            begin: Position::new(self.line, self.colunm),
            end: Position::new(self.line, self.colunm),
            file_name: self.name,
        }
    }

    pub fn set_module(&mut self, module: Vec<char>) {
        self.module = module;
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'_, 'a>>
    where
        'a: 'b,
    {
        let mut at_end = self.is_at_end();
        while !at_end {
            self.start = self.current;
            if let Some(t) = self.scan_token() {
                self.token_list.push(t.clone());
            }
            at_end = self.is_at_end();
        }
        let info = self.get_info();

        self.token_list.push(Token::new(TokenType::EOF, "", info));
        std::mem::take(&mut self.token_list)
    }

    fn is_at_end(&self) -> bool {
        (self.current) >= (self.source.chars().count())
    }

    fn scan_token(&mut self) -> Option<Token<'b, 'a>>
    where
        'a: 'b,
    {
        let c: char = self.advance();
        match c {
            '{' => Some(self.add_token(TokenType::LeftBrace)),
            '[' => Some(self.add_token(TokenType::LeftBracket)),
            '<' => Some(self.add_token(TokenType::LessThanSymbol)),
            '>' => Some(self.add_token(TokenType::GreaterThanSymbol)),
            '}' => Some(self.add_token(TokenType::RightBrace)),
            ']' => Some(self.add_token(TokenType::RightBracket)),
            '⧼' => Some(self.add_token(TokenType::CodeBlockBegin)),
            '⧽' => Some(self.add_token(TokenType::CodeBlockEnd)),
            // any of the opening brackets from https://unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt
            '(' | '༺' | '༼' | '᚛' | '⁅' | '⁽' | '₍' | '⌈' | '⌊' | '❨' | '❪' | '❬' | '❮' | '❰'
            | '❲' | '❴' | '⟅' | '⟦' | '⟨' | '⟪' | '⟬' | '⟮' | '⦃' | '⦅' | '⦇' | '⦉' | '⦋' | '⦍'
            | '⦏' | '⦑' | '⦓' | '⦕' | '⦗' | '⧘' | '⧚' | '⸢' | '⸤' | '⸦' | '⸨' | '\u{2e55}'
            | '\u{2e57}' | '\u{2e59}' | '\u{2e5b}' | '〈' | '《' | '「' | '『' | '【' | '〔'
            | '〖' | '〘' | '〚' | '﹙' | '﹛' | '﹝' | '（' | '［' | '｛' | '｟' | '｢' => {
                Some(self.add_token(TokenType::CallBegin))
            }
            // any of the closing brackets from https://unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt
            ')' | '༻' | '༽' | '᚜' | '⁆' | '⁾' | '₎' | '⌉' | '⌋' | '❩' | '❫' | '❭' | '❯' | '❱'
            | '❳' | '❵' | '⟆' | '⟧' | '⟩' | '⟫' | '⟭' | '⟯' | '⦄' | '⦆' | '⦈' | '⦊' | '⦌' | '⦎'
            | '⦐' | '⦒' | '⦔' | '⦖' | '⦘' | '⧙' | '⧛' | '⸣' | '⸥' | '⸧' | '⸩' | '\u{2e56}'
            | '\u{2e58}' | '\u{2e5a}' | '\u{2e5c}' | '〉' | '》' | '」' | '』' | '】' | '〕'
            | '〗' | '〙' | '〛' | '﹚' | '﹜' | '﹞' | '）' | '］' | '｝' | '｠' | '｣' => {
                Some(self.add_token(TokenType::CallEnd))
            }
            '!' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
            }
            '.' => Some(self.add_token(TokenType::Dot)),

            '\n' => {
                self.line += 1;
                self.colunm = 1;
                None
            }
            '`' => Some(self.string()),
            '$' => Some(self.function_agument()),
            '*' => Some(self.add_token(TokenType::Star)),
            c if c.is_ascii_whitespace() => None,
            c if c.is_ascii_digit() => {
                if self.peek() == 'x' {
                    self.advance();
                } else {
                    self.push_text(c);
                }
                Some(self.number())
            }
            c if emoji::is_emoji(c) => {
                for module in self.module.clone() {
                    self.add_token(TokenType::ModuleIdentifier(module));
                }
                Some(self.add_unicode_token(TokenType::FunctionIdentifier(c)))
            }
            c if c.is_lowercase() => {
                self.push_text(c);
                self.lex_lit()
            }
            c => {
                error(self.get_info(), format!("uknown character {c}"));
            }
        }
    }

    fn lex_lit(&mut self) -> Option<Token<'b, 'a>>
    where
        'a: 'b,
    {
        while self.peek().is_alphabetic() {
            self.advance_insert();
        }
        if self.get_text() == "true" {
            Some(self.add_token(TokenType::Boolean(true)))
        } else if self.get_text() == "false" {
            Some(self.add_token(TokenType::Boolean(false)))
        } else if self.get_text() == "hempty" {
            Some(self.add_token(TokenType::Hempty))
        } else {
            self.identifier()
        }
    }

    #[allow(clippy::too_many_lines)]
    fn string(&mut self) -> Token<'b, 'a>
    where
        'a: 'b,
    {
        while self.peek() != '`' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            // check for escape sequence \` \n \\ \t \r \a \b \f \v \e \Xhh \0ooo \Uhhhhhhhh
            if self.peek() == '\\' {
                self.advance();
                if self.peek() == '`' {
                    self.push_text('`');
                    self.advance();
                } else if self.peek() == 'n' {
                    self.push_text('\n');
                    self.advance();
                } else if self.peek() == '\\' {
                    self.push_text('\\');
                    self.advance();
                } else if self.peek() == 't' {
                    self.push_text('\t');
                    self.advance();
                } else if self.peek() == 'r' {
                    self.push_text('\r');
                    self.advance();
                } else if self.peek() == 'a' {
                    self.push_text('\x07');
                    self.advance();
                } else if self.peek() == 'b' {
                    self.push_text('\x08');
                    self.advance();
                } else if self.peek() == 'f' {
                    self.push_text('\x0C');
                    self.advance();
                } else if self.peek() == 'v' {
                    self.push_text('\x0b');
                    self.advance();
                } else if self.peek() == 'e' {
                    self.push_text('\x1b');
                    self.advance();
                } else if self.peek() == 'x' {
                    match self.peek() {
                        x if x.is_ascii_hexdigit() => match self.peek() {
                            y if y.is_ascii_hexdigit() => {
                                self.push_text(
                                    u8::from_str_radix(format!("{x}{y}").as_str(), 16)
                                        .unwrap_or_else(|_| {
                                            error(self.get_info(), "invalid hex escape sequence");
                                        }) as char,
                                );
                                self.advance();
                            }
                            y => {
                                self.push_text(
                                    u8::from_str_radix(format!("{x}").as_str(), 16).unwrap_or_else(
                                        |_| {
                                            error(self.get_info(), "invalid hex escape sequence");
                                        },
                                    ) as char,
                                );
                                self.push_text(y);
                                self.advance();
                            }
                        },
                        x => {
                            self.push_text(x);
                            self.advance();
                        }
                    }
                } else if self.peek() == 'u' {
                    // can be from 0 to 6 hex digits
                    // loop until either 6 digits or we find a digit a non-hex digit
                    // while looping remove the character and append to a string
                    let mut hex_string = String::new();
                    let mut i = 0;
                    self.advance();
                    while i < 6 && self.peek().is_ascii_hexdigit() {
                        hex_string.push(self.advance());
                        i += 1;
                    }
                    self.push_text(
                        char::from_u32(
                            u32::from_str_radix(hex_string.as_str(), 16).unwrap_or_else(|_| {
                                error(
                                    self.get_info(),
                                    format!("invalid unicode escape sequence '{hex_string}'"),
                                );
                            }),
                        )
                        .unwrap_or_else(|| {
                            error(
                                self.get_info(),
                                format!("invalid unicode escape sequence '{hex_string}'"),
                            );
                        }),
                    );
                } else {
                    error(
                        self.get_info(),
                        format!("unknown escape sequence '{}'", self.peek()),
                    );
                }
            } else {
                self.advance_insert();
            }
        }
        if self.is_at_end() {
            error(
                self.get_info(),
                format!("unterminated string: '{}'", self.get_text()),
            );
        }
        self.advance();
        self.add_token(TokenType::String(self.get_text()))
    }

    fn number(&mut self) -> Token<'b, 'a> {
        while self.peek().is_ascii_hexdigit() {
            self.push_text(self.peek());
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
            .unwrap_or_else(|_| {
                error(
                    self.get_info(),
                    format!("invalid number: '{}'", self.get_text()),
                )
            });
        self.add_token(TokenType::Number(number.convert::<f64>().inner()))
    }

    fn identifier(&mut self) -> Option<Token<'b, 'a>>
    where
        'a: 'b,
    {
        if !self
            .get_text()
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '-')
            && !self.get_text().contains('+')
        {
            error(
                self.get_info(),
                format!("invalid identifier '{}'", self.get_text()),
            );
        }

        while self.peek().is_lowercase() || self.peek() == '-' || self.peek().is_numeric() {
            self.advance_insert();
        }
        if self.peek() == '+' {
            self.advance_insert();
            // see if the next character is an emoji
            while self.peek().is_lowercase() && self.peek_next() == '+' {
                self.advance_insert();
                self.advance_insert();
            }
            if emoji::is_emoji(self.peek()) {
                self.advance_insert();
                let name = self.get_text().to_string();
                #[allow(clippy::needless_collect)]
                let parts = name
                    .chars()
                    .map(|char_part| match char_part {
                        '+' => self.add_token(TokenType::PlusSymbol),
                        module if module.is_ascii_lowercase() => {
                            self.add_token(TokenType::ModuleIdentifier(char_part))
                        }
                        function if emoji::is_emoji(function) => {
                            self.add_unicode_token(TokenType::FunctionIdentifier(function))
                        }
                        char => error(
                            self.get_info(),
                            format!("{char} not allowed in function or module name"),
                        ),
                    })
                    // need to collect becuase both iterators use self
                    .collect::<Vec<_>>();
                parts.into_iter().for_each(|p| self.token_list.push(p));
                None
            } else {
                // error out
                error(
                    self.get_info(),
                    format!("Unexpected character after + {}", self.peek()),
                );
            }
        } else {
            let text = self.get_text();
            if let Some(token) = crate::KEYWORDS.string_is_builtin_function(text) {
                Some(self.add_token(TokenType::BuiltinFunction(token)))
            } else if let Some(token) = crate::KEYWORDS.string_is_keyword(text) {
                Some(self.add_token(token))
            } else {
                Some(self.add_token(TokenType::Identifier(self.get_text())))
            }
        }
    }

    fn function_agument(&mut self) -> Token<'b, 'a> {
        self.start += 1; // advance start past the $ so that we can parse it into a number
        while self.peek().is_ascii_hexdigit() {
            self.advance_insert();
        }
        let number: u64 = self.get_text().parse().unwrap_or_else(|_| {
            error(
                self.get_info(),
                format!(
                    "invalid function argument need number: '{}'",
                    self.get_text()
                ),
            )
        });
        self.add_token(TokenType::FunctionArgument(number))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.colunm += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn advance_insert(&mut self) -> char {
        let new_char = self.advance();
        self.push_text(new_char);
        new_char
    }

    fn add_token(&mut self, token_type: TokenType<'b>) -> Token<'b, 'a> {
        let text: std::str::Chars<'_> = self.source.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        self.clear_text();
        Token::new(token_type, final_text.as_str(), self.get_info())
    }

    fn add_unicode_token(&mut self, token_type: TokenType<'b>) -> Token<'b, 'a> {
        let text: String = format!(
            "{}",
            self.source.chars().nth(self.start).unwrap_or_else(|| {
                error(
                    self.get_info(),
                    format!("could not get unicode character at {}", self.start),
                );
            })
        );
        Token::new(token_type, &text, self.get_info())
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn get_text(&self) -> &'b str {
        self.text_buffer
    }

    fn push_text(&mut self, char: char) {
        let mut boxed = Box::new((**self.text_buffer).to_string());
        boxed.push(char);
        *self.text_buffer = Box::leak(boxed);
    }

    fn clear_text(&mut self) {
        let boxed = Box::<String>::default();
        *self.text_buffer = Box::leak(boxed);
    }
}
