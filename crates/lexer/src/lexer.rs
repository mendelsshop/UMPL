use std::collections::HashMap;

use crate::tokens::{Token, TokenType};

// this struct is used so we can iterate over the text in the lexer
struct TextIter<'a> {
    text: &'a str,
    pos: usize,
}
impl<'a> TextIter<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, pos: 0 }
    }
}
impl<'a> Iterator for TextIter<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        // if we are at the end of the text, return None
        if self.pos >= self.text.len() {
            None
        } else {
            // otherwise return the character at the current position and in an option
            // and increment the position
            let c = self.text.chars().nth(self.pos).unwrap();
            self.pos += 1;
            Some(c)
        }
    }
}

pub struct Lexer<'input> {
    text: TextIter<'input>,                  // iterable text to be lexed
    current_char: Option<char>,              // the current character in the text
    keyword_map: HashMap<String, TokenType>, // map of keywords to their token types should be a moved elsewhere
    file: String,                            // the file name of the text being lexed
    line: u32,                               // the line number of the text being lexed
}

impl<'input> Lexer<'input> {
    pub fn new(text: &'input str, file: String, line: u32) -> Lexer<'input> {
        let keywords = [
            // this is a list of keywords be stored somwhere else
            "plus", "minus", "multiply", "divide", "power", "eq", "bang", "lparen", "rparen",
            "number",
        ];
        let mut keyword_map = HashMap::new(); // same as above
        for i in keywords.iter() {
            keyword_map.insert(i.to_string(), TokenType::token_type_from_string(i));
        }
        Lexer {
            text: TextIter::new(text),
            current_char: Some(' '),
            keyword_map,
            file,
            line,
        }
    }

    // updates the current_char field
    fn advance(&mut self) {
        self.current_char = self.text.next();
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new(); // vector of tokens to be returned
        while self.current_char != None {
            // while there is still text to be lexed
            match self.current_char.unwrap() {
                // match the current character
                char if char.is_whitespace() => {
                    // if the character is a whitespace continue
                    self.advance();
                }
                char if char.is_ascii_digit() => {
                    // if the character is a digit
                    let num = self.read_number(); // read the number
                    tokens.push(Token::new(
                        TokenType::Number,
                        num,
                        self.file.clone(),
                        self.line,
                    )); // tokenize the number and push it to the vector
                }
                char if char.is_alphabetic() => {
                    // if the character is an alphabetic character doent include +-*/=!() so we can force the use of plus, minus, etc.
                    let ident = self.read_identifier(); // read the text
                    let t = self.match_identifier(ident.as_str()); // match if the text is a keyword or an identifier and return the token type

                    tokens.push(t); // push the token to the vector
                }
                _ => {
                    panic!("Unknown character: {}", self.current_char.unwrap());
                    // if the character is not a digit, letter, or whitespace, panic
                }
            }
        }
        tokens
    }
    fn read_number(&mut self) -> String {
        let mut num = String::new();
        while self.current_char != None && self.current_char.unwrap().is_ascii_digit() {
            // while there is still text to be lexed and the current character is a digit
            num.push(self.current_char.unwrap()); // push the current character to the number and advance
            self.advance();
        }
        num // return the number
    }
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while self.current_char != None && self.current_char.unwrap().is_alphanumeric() {
            // while there is still text to be lexed and the current character is a letter or digit
            ident.push(self.current_char.unwrap()); // push the current character to the identifier and advance
            self.advance();
        }
        ident // return the identifier
    }

    fn match_identifier(&mut self, ident: &str) -> Token {
        if let Some(token_type) = self.keyword_map.get(ident) {
            // if the identifier is a keyword
            Token::new(*token_type, "".to_string(), self.file.clone(), self.line)
        // return a token with the with the type of keyword
        } else {
            Token::new(
                TokenType::Identifier,
                ident.to_string(),
                self.file.clone(),
                self.line,
            ) // otherwise return a token with the type of identifier and the identifier
        }
    }
}
