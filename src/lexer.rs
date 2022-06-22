use crate::{
    error,
    token::{Token, TokenType},
};

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
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token()
        }
        self.token_list.push(Token::new(TokenType::EOF, "", self.line));
        &self.token_list
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
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
            },
            '.' => self.add_token(TokenType::Dot),
            '<' => self.add_token(TokenType::LessThan),
            '>' => self.add_token(TokenType::GreaterThan),
            ':' => self.add_token(TokenType::Colon),
            '\n' => self.line += 1,
            '`' => self.string(),
            char if char.is_whitespace() => (),

            _ => {}
        }
    }

    fn string(&mut self) {
        while self.peek() != '`' && !self.is_at_end(){
            if self.peek() == '\n' {
                // rn at
                // http://craftinginterpreters.com/scanning.html
                // https://github.com/jeschkies/lox-rs/commit/9fef15e73fdf57a3e428bb074059c7e144e257f7#diff-42cb6807ad74b3e201c5a7ca98b911c5fa08380e942be6e4ac5807f8377f87fc
            }    
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let char_vec: Vec<char> = self.source.chars().collect();
        char_vec[self.current- 1]
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("empty source");
        self.token_list
            .push(Token::new(token_type, text, self.line));
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')    }
}