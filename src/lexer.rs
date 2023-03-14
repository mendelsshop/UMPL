use crate::{
    error::error,
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
    module: Vec<char>,
    name: &'a str,
    text_buffer: String,
}

impl<'a> Lexer<'a> {
    pub const fn new(source: &'a str, name: &'a str) -> Self {
        Self {
            token_list: Vec::new(),
            source,
            start: 0,   // bytes
            current: 0, // actual number of bytes in source
            line: 1,
            module: Vec::new(),
            name,
            text_buffer: String::new(),
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

    pub fn set_module(&mut self, module: Vec<char>) {
        self.module = module;
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
        (self.current) >= (self.source.chars().count())
    }

    fn scan_token<'b>(&mut self) -> Option<Token<'b>>
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
            '\u{0028}' | '\u{0F3A}' | '\u{0F3C}' | '\u{169B}' | '\u{2045}' | '\u{207D}'
            | '\u{208D}' | '\u{2308}' | '\u{230A}' | '\u{2329}' | '\u{2768}' | '\u{276A}'
            | '\u{276C}' | '\u{276E}' | '\u{2770}' | '\u{2772}' | '\u{2774}' | '\u{27C5}'
            | '\u{27E6}' | '\u{27E8}' | '\u{27EA}' | '\u{27EC}' | '\u{27EE}' | '\u{2983}'
            | '\u{2985}' | '\u{2987}' | '\u{2989}' | '\u{298B}' | '\u{298D}' | '\u{298F}'
            | '\u{2991}' | '\u{2993}' | '\u{2995}' | '\u{2997}' | '\u{29D8}' | '\u{29DA}'
            | '\u{2E22}' | '\u{2E24}' | '\u{2E26}' | '\u{2E28}' | '\u{2E55}' | '\u{2E57}'
            | '\u{2E59}' | '\u{2E5B}' | '\u{3008}' | '\u{300A}' | '\u{300C}' | '\u{300E}'
            | '\u{3010}' | '\u{3014}' | '\u{3016}' | '\u{3018}' | '\u{301A}' | '\u{FE59}'
            | '\u{FE5B}' | '\u{FE5D}' | '\u{FF08}' | '\u{FF3B}' | '\u{FF5B}' | '\u{FF5F}'
            | '\u{FF62}' => Some(self.add_token(TokenType::CallBegin)),
            // any of the closing brackets from https://unicode.org/Public/UCD/latest/ucd/BidiBrackets.txt
            '\u{0029}' | '\u{0F3B}' | '\u{0F3D}' | '\u{169C}' | '\u{2046}' | '\u{207E}'
            | '\u{208E}' | '\u{2309}' | '\u{230B}' | '\u{232A}' | '\u{2769}' | '\u{276B}'
            | '\u{276D}' | '\u{276F}' | '\u{2771}' | '\u{2773}' | '\u{2775}' | '\u{27C6}'
            | '\u{27E7}' | '\u{27E9}' | '\u{27EB}' | '\u{27ED}' | '\u{27EF}' | '\u{2984}'
            | '\u{2986}' | '\u{2988}' | '\u{298A}' | '\u{298C}' | '\u{298E}' | '\u{2990}'
            | '\u{2992}' | '\u{2994}' | '\u{2996}' | '\u{2998}' | '\u{29D9}' | '\u{29DB}'
            | '\u{2E23}' | '\u{2E25}' | '\u{2E27}' | '\u{2E29}' | '\u{2E56}' | '\u{2E58}'
            | '\u{2E5A}' | '\u{2E5C}' | '\u{3009}' | '\u{300B}' | '\u{300D}' | '\u{300F}'
            | '\u{3011}' | '\u{3015}' | '\u{3017}' | '\u{3019}' | '\u{301B}' | '\u{FE5A}'
            | '\u{FE5C}' | '\u{FE5E}' | '\u{FF09}' | '\u{FF3D}' | '\u{FF5D}' | '\u{FF60}'
            | '\u{FF63}' => Some(self.add_token(TokenType::CallEnd)),
            '!' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                None
            }
            '.' => Some(self.add_token(TokenType::Dot)),

            '\n' => {
                self.line += 1;
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
                    self.insert_text(c);
                }
                Some(self.number())
            }
            c if emoji::is_emoji(c) => {
                for module in self.module.clone() {
                    self.add_token(TokenType::ModuleIdentifier(module));
                }
                Some(self.add_unicode_token(TokenType::FunctionIdentifier(c)))
            }
            c if c == 't' || c == 'f' => {
                self.insert_text(c);
                self.boolean().map_or_else(|| self.identifier(), Some)
            }
            c if c == 'h' => {
                self.insert_text(c);
                self.hempty().map_or_else(|| self.identifier(), Some)
            }
            c if c.is_lowercase() => {
                self.insert_text(c);
                self.identifier()
            }
            c => {
                error(self.get_info(), format!("uknown character {c}"));
            }
        }
    }

    fn boolean(&mut self) -> Option<Token<'a>> {
        while self.peek().is_alphabetic() {
            self.advance_insert();
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
            self.advance_insert();
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
                self.advance();
                if self.peek() == '`' {
                    self.insert_text('`');
                    self.advance();
                } else if self.peek() == 'n' {
                    self.insert_text('\n');
                    self.advance();
                } else if self.peek() == '\\' {
                    self.insert_text('\\');
                    self.advance();
                } else if self.peek() == 't' {
                    self.insert_text('\t');
                    self.advance();
                } else if self.peek() == 'r' {
                    self.insert_text('\r');
                    self.advance();
                } else if self.peek() == 'a' {
                    self.insert_text('\x07');
                    self.advance();
                } else if self.peek() == 'b' {
                    self.insert_text('\x08');
                    self.advance();
                } else if self.peek() == 'f' {
                    self.insert_text('\x0C');
                    self.advance();
                } else if self.peek() == 'v' {
                    self.insert_text('\x0b');
                    self.advance();
                } else if self.peek() == 'e' {
                    self.insert_text('\x1b');
                    self.advance();
                } else if self.peek() == 'x' {
                    match self.peek() {
                        x if x.is_ascii_hexdigit() => match self.peek() {
                            y if y.is_ascii_hexdigit() => {
                                self.insert_text(
                                    u8::from_str_radix(format!("{x}{y}").as_str(), 16)
                                        .unwrap_or_else(|_| {
                                            error(self.get_info(), "invalid hex escape sequence");
                                        }) as char,
                                );
                                self.advance();
                            }
                            y => {
                                self.insert_text(
                                    u8::from_str_radix(format!("{x}").as_str(), 16).unwrap_or_else(
                                        |_| {
                                            error(self.get_info(), "invalid hex escape sequence");
                                        },
                                    ) as char,
                                );
                                self.insert_text(y);
                                self.advance();
                            }
                        },
                        x => {
                            self.insert_text(x);
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
                    self.insert_text(
                        char::from_u32(
                            u32::from_str_radix(hex_string.as_str(), 16).unwrap_or_else(|_| {
                                error(
                                    self.get_info(),
                                    format!("invalid unicode escape sequence '{}'", hex_string),
                                );
                            }),
                        )
                        .unwrap_or_else(|| {
                            error(
                                self.get_info(),
                                format!("invalid unicode escape sequence '{}'", hex_string),
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
        // self.start += 1;
        // self.current -= 1;
        let string = self.get_text();
        // self.start -= 1;
        // self.current += 1;
        self.add_token(TokenType::String(string))
    }

    fn number(&mut self) -> Token<'a> {
        while self.peek().is_ascii_hexdigit() {
            self.insert_text(self.peek());
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

    fn identifier(&mut self) -> Option<Token<'a>> {
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
                let name = self.get_text();
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
        while self.peek().is_ascii_hexdigit() {
            self.advance_insert();
        }
        let identifier = format!(
            "${}",
            match format!("0x{}", self.get_text()).parse::<FloatLiteral>() {
                Ok(contents) => {
                    contents.convert::<f64>().inner()
                }
                Err(errors) => {
                    error(self.get_info(), errors);
                }
            }
        );
        self.add_token(TokenType::Identifier(identifier))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current - 1]
    }

    fn advance_insert(&mut self) -> char {
        let char_vec: Vec<char> = self.source.chars().collect();
        self.insert_text(char_vec[self.current]);
        self.current += 1;
        char_vec[self.current - 1]
    }

    fn add_token(&mut self, token_type: TokenType) -> Token<'a> {
        let text: std::str::Chars<'_> = self.source.chars();
        let mut final_text: String = String::new();
        text.enumerate().for_each(|i| {
            if i.0 >= self.start && i.0 < self.current {
                final_text.push(i.1);
            }
        });
        self.text_buffer.clear();
        Token::new(token_type, final_text.as_str(), self.get_info())
    }

    fn add_unicode_token(&mut self, token_type: TokenType) -> Token<'a> {
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

    fn get_text(&self) -> String {
        self.text_buffer.clone()
    }

    fn insert_text(&mut self, text: char) {
        self.text_buffer.push(text);
    }
}
