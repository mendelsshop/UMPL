use crate::{
    error::error,
    parser::rules::{FnCall, PrintType},
    token::{Info, Token, TokenType},
};

use self::rules::{Expr, FnDef, If, Lambda, Lit, Loop, List, Var};
pub(crate) mod rules;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_position: usize,
    done: bool,
    token: Token<'a>,
    in_function: bool,
    in_loop: bool,
    paren_count: usize,
    weird_bracket_count: usize,
    file_path: &'a str,
}
static START_TOKEN: Token<'static> = Token {
    token_type: TokenType::EOF,
    info: Info::new("", 0, 0),
    lexeme: String::new(),
};

type Module<'a> = Vec<Expr<'a>>;
impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>, file_path: &'a str) -> Self {
        Self {
            tokens,
            current_position: 0,
            done: false,
            token: START_TOKEN.clone(),
            in_function: false,
            in_loop: false,
            paren_count: 0,
            weird_bracket_count: 0,
            file_path,
        }
    }

    pub fn parse(&mut self) -> Module<'a> {
        let mut module = Vec::new();
        while !self.done {
            if let Some(token) = self.parse_from_token_advance() {
                module.push(token);
            }
        }
        module
    }

    fn parse_from_token(&mut self) -> Option<Expr<'a>> {
        // advance in token stream - important especially for the first time each time we parse something
        // becuase token is set to nothing so this initializes token

        // check if we ran out of tokens or are done parsing (if EOF is encountered)
        if self.tokens.is_empty() {
            error(0, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        match self.token.token_type.clone() {
            // we have entered an expression
            TokenType::LeftParen => Some(self.parse_parenthissized()),
            // we have entered a function
            TokenType::Potato => self.parse_function(),
            // TokenType::CodeBlockBegin => self.parse_function_body(),
            TokenType::CodeBlockEnd => {
                self.in_function = false;
                None
            }
            TokenType::RightParen => None,
            // parsing literals
            TokenType::String { literal } => Some(Expr::new_literal(
                self.token.info,
                Lit::new_string(self.token.info, literal),
            )),
            TokenType::Number { literal } => Some(Expr::new_literal(
                self.token.info,
                Lit::new_number(self.token.info, literal),
            )),
            TokenType::Boolean { literal } => Some(Expr::new_literal(
                self.token.info,
                Lit::new_boolean(self.token.info, literal),
            )),
            TokenType::Hempty => Some(Expr::new_literal(
                self.token.info,
                Lit::new_hempty(self.token.info),
            )),
            TokenType::Loop => Some(Expr::new_loop(self.token.info, self.parse_loop())),
            TokenType::If => Some(Expr::new_if(self.token.info, self.parse_if())),
            TokenType::Break => {
                if self.in_loop {
                    Some(Expr::new_break(self.token.info, self.parse_from_token()))
                } else {
                    error(self.token.info.line, "break can only be used inside a loop");
                }
            }
            TokenType::Continue => {
                if self.in_loop {
                    Some(Expr::new_continue(self.token.info))
                } else {
                    error(
                        self.token.info.line,
                        "continue can only be used inside a loop",
                    );
                }
            }
            TokenType::Return { .. } => Some(Expr::new_return(
                self.token.info,
                self.parse_from_token_advance(),
            )),
            TokenType::List => Some(
                self.parse_list(),
            ),
            // we hit a keyword
            keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                todo!("keyword: {:?}", keyword)
            }
            keyword => {
                todo!("keyword: {:?}", keyword)
            }
        }
    }

    fn parse_from_token_advance(&mut self) -> Option<Expr<'a>> {
        self.advance("parse_from_token - start");
        self.parse_from_token()
    }

    fn parse_parenthissized(&mut self) -> Expr<'a> {
        println!("parse_parenthissized");
        // save the line number because the next token can be on the next line
        let start_line = self.token.info.line;
        let mut args = Vec::new();
        // self.advance("parse_parenthissized - start looking for args");
        while self.token.token_type != TokenType::RightParen {
            if let Some(token) = self.parse_from_token_advance() {
                args.push(token);
            }
            // self.advance("parse_parenthissized - looking for args");
        }
        // check printing indicator (<) no print (>) print (>>) print no newline
        self.advance("parse_parenthissized - looking for printing indicator");
        println!("printing indicator: {:?}", self.token);
        match self.token.token_type {
            TokenType::LessThanSymbol => {
                println!("call parsed");
                Expr::new_call(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    FnCall::new(
                        Info::new(self.file_path, start_line, self.token.info.line),
                        args,
                        PrintType::None,
                    ),
                )
            }
            TokenType::GreaterThanSymbol => {
                // TODO: check if next token is a > if it is use PrintType::NoNewline
                println!("call parsed");
                Expr::new_call(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    FnCall::new(
                        Info::new(self.file_path, start_line, self.token.info.line),
                        args,
                        PrintType::Newline,
                    ),
                )
            }
            _ => {
                // should never happen as this is caught in the advance method
                error(self.token.info.line, "Expected printing indicator");
            }
        }
    }

    fn parse_function(&mut self) -> Option<Expr<'a>> {
        let start_line = self.token.info.line;
        self.advance("parse_function - start");
        match self.token.token_type.clone() {
            TokenType::FunctionIdentifier { name } => {
                let fn_def = self.parse_named_function(name);
                Some(Expr::new_fn(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    fn_def,
                ))
            }
            TokenType::Star | TokenType::CodeBlockBegin | TokenType::Number { .. } => {
                let lambda = self.parse_function_body();
                Some(Expr::new_lambda(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    lambda,
                ))
            }
            tt => error(
                self.token.info.line,
                format!("expected function name, number, * or ⧼ found {tt} in function defintion"),
            ),
        }
    }

    fn parse_named_function(&mut self, name: String) -> FnDef<'a> {
        let start_line = self.token.info.line;
        self.advance("parsed_named_function");
        let body = match self.token.token_type.clone() {
            TokenType::Number { .. } | TokenType::Star | TokenType::CodeBlockBegin => {
                self.parse_function_body()
            }
            tt => error(
                self.token.info.line,
                format!("expected number, * or ⧼ found {tt} in function defintion"),
            ),
        };
        println!("named function parsed");
        FnDef::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            name,
            body,
        )
    }

    /// not only used to parse the body of a function but also to parse anonymous functions
    fn parse_function_body(&mut self) -> Lambda<'a> {
        let mut arg_count = 0;
        let mut extra_args = false;
        match self.token.token_type {
            TokenType::Star => {
                extra_args = true;
                self.advance("parse_function_body");
                if self.token.token_type != TokenType::CodeBlockBegin {
                    error(
                        self.token.info.line,
                        "Expected ⧼ after * in function definition",
                    );
                }
            }
            TokenType::Number { literal } => {
                if literal.round() != literal {
                    error(self.token.info.line, "Expected integer number of arguments");
                }
                arg_count = literal as usize;
                self.advance("parse_function_body");
                if self.token.token_type == TokenType::Star {
                    extra_args = true;
                    self.advance("parse_function_body");
                }
                if self.token.token_type != TokenType::CodeBlockBegin {
                    error(
                        self.token.info.line,
                        format!(
                            "Expected ⧼ after {} in function definition",
                            if extra_args { "*" } else { "number" }
                        ),
                    );
                }
            }
            TokenType::CodeBlockBegin => {}
            _ => {
                error(
                    self.token.info.line,
                    "Expected number, * or ⧼ in function definition",
                );
            }
        };
        let mut body = Vec::new();
        while self.token.token_type != TokenType::CodeBlockEnd {
            self.in_function = true;
            if let Some(expr) = self.parse_from_token_advance() {
                body.push(expr);
            }
        }
        self.in_function = false;
        println!("lambda parsed");
        Lambda::new(
            Info::new(self.file_path, self.token.info.line, self.token.info.line),
            arg_count,
            extra_args,
            body,
        )
    }

    fn parse_loop(&mut self) -> Loop<'a> {
        let start_line = self.token.info.line;
        self.advance("parse_loop");
        if self.token.token_type != TokenType::CodeBlockBegin {
            error(
                start_line,
                format!(
                    "expected ⧼ after loop keyword found {}",
                    self.token.token_type
                ),
            );
        }
        let mut loop_exprs = vec![];
        while self.token.token_type != TokenType::CodeBlockEnd {
            if let Some(loop_expr) = self.parse_from_token_advance() {
                loop_exprs.push(loop_expr);
            }
        }
        Loop::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            loop_exprs,
        )
    }

    fn parse_if(&mut self) -> If<'a> {
        let start_line = self.token.info.line;
        self.advance("parse if");
        if self.token.token_type != TokenType::LeftBrace {}
        let cond = if let Some(expr) = self.parse_from_token_advance() {
            expr
        } else {
            error(self.token.info.line, "expected expression in conditonal")
        };
        self.advance("parse if");
        if self.token.token_type != TokenType::RightBrace {}
        self.advance("parse if");
        if self.token.token_type != TokenType::CodeBlockBegin {}
        let mut if_then_exprs = vec![];
        while self.token.token_type != TokenType::CodeBlockEnd {
            if let Some(if_then_expr) = self.parse_from_token_advance() {
                if_then_exprs.push(if_then_expr);
            }
        }

        let mut else_exprs = vec![];
        while self.token.token_type != TokenType::CodeBlockEnd {
            if let Some(else_expr) = self.parse_from_token_advance() {
                else_exprs.push(else_expr);
            }
        }
        If::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            cond,
            if_then_exprs,
            else_exprs,
        )
    }

    fn parse_list(&mut self) -> Expr<'a> {
        let start_line = self.token.info.line;
        self.advance("parse list");
        match self.token.token_type.clone() {
            TokenType::LeftBracket => {
                Expr::new_list(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    self.parse_list_inner(),
                )
            }
            TokenType::Identifier { name } => {
                self.advance("parse list");
                if self.token.token_type !=  TokenType::With {
                    error(
                        self.token.info.line,
                        format!(
                            "expected with keyword after list identifier found {}",
                            self.token.token_type
                        ),
                    );
                }
                self.advance("parse list");
                if self.token.token_type!= TokenType::LeftBracket {
                    error(
                        self.token.info.line,
                        format!(
                            "expected [ after with keyword found {}",
                            self.token.token_type
                        ),
                    );
                }
                Expr::new_var(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    Var::new(
                        Info::new(self.file_path, start_line, self.token.info.line),
                        name,
                        Expr::new_list(
        
                        Info::new(self.file_path, start_line, self.token.info.line),
                        self.parse_list_inner(),
                    )),
                    
                )
        
            }
            _ => {
                error(
                    self.token.info.line,
                    format!(
                        "expected [ or identifier after list keyword found {}",
                        self.token.token_type
                    ),
                );
            }
        }

    }

    fn parse_list_inner(&mut self) -> List<'a> {
        let start_line = self.token.info.line;
        self.advance("parse list inner");
        let car = if let Some(expr) = self.parse_from_token() {
            expr
        } else {
            error(self.token.info.line, "expected expression in list")
        };
        self.advance("parse list inner");
        let cdr = if let Some(expr) = self.parse_from_token() {
            expr
        } else {
            error(self.token.info.line, "expected expression in list")
        };
        self.advance("parse list inner");
        if self.token.token_type!= TokenType::RightBracket {
            error(
                self.token.info.line,
                format!(
                    "expected ] after list item found {}",
                    self.token.token_type
                    ),
            );
        }
        return List::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            car,
            cdr,
        );
    }

    fn advance(&mut self, caller: &str) {
        match self.tokens[self.current_position].token_type {
            TokenType::Return { .. } => {
                if self.in_function {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].info.line,
                        "Return statement outside of function",
                    );
                }
            }
            TokenType::Break | TokenType::Continue => {
                if self.in_loop {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].info.line,
                        "Break or continue statement outside of loop",
                    );
                }
            }
            TokenType::EOF => {
                self.done = true;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::LeftParen => {
                self.paren_count += 1;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::RightParen => {
                if self.paren_count == 0 {
                    error(
                        self.tokens[self.current_position].info.line,
                        "unmatched right parenthesis",
                    );
                }
                self.paren_count -= 1;
                if !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                    .contains(&self.tokens[self.current_position + 1].token_type))
                {
                    error(
                        self.tokens[self.current_position].info.line,
                        format!(
                            "greater than symbol (>) or less than symbol (<) expected found {}",
                            self.tokens[self.current_position + 1].token_type
                        )
                        .as_str(),
                    )
                }
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::CodeBlockBegin => {
                self.weird_bracket_count += 1;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::CodeBlockEnd => {
                self.weird_bracket_count -= 1;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::GreaterThanSymbol | TokenType::LessThanSymbol => {
                // TODO: check if the last token was a parentheisis or in the case of > a parenthesis of >
                self.token = self.tokens[self.current_position].clone();
            }
            _ => {
                self.token = self.tokens[self.current_position].clone();
            }
        };
        println!("token: {:?} caller: {}", self.token, caller);
        self.current_position += 1;
    }
}
