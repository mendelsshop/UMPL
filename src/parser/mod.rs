use crate::{
    error::error,
    parser::rules::{FnCall, PrintType},
    token::{Info, Token, TokenType},
};

use self::rules::{Expr, FnDef, Ident, IdentType, If, Lambda, List, Lit, Loop, Var};
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
    in_stopper: bool,
}
static START_TOKEN: Token<'static> = Token {
    token_type: TokenType::EOF,
    info: Info::new("", 0, 0),
    lexeme: String::new(),
};

macro_rules! parse_break_return {
    ($self:ident, $ok:expr, $new_method:ident, $err1:expr, $err2:expr) => {
        if $ok {
            $self.in_stopper = true;
            let expr = Some(Expr::$new_method(
                $self.token.info,
                $self
                    .parse_from_token_advance()
                    .unwrap_or_else(|| error($self.token.info.line, $err1)),
            ));
            $self.in_stopper = false;
            expr
        } else {
            error($self.token.info.line, $err2);
        }
    };
}

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
            in_stopper: false,
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
            TokenType::CodeBlockEnd | TokenType::RightParen => None,
            // parsing literals
            TokenType::String(literal) => Some(Expr::new_literal(
                self.token.info,
                Lit::new_string(self.token.info, literal),
            )),
            TokenType::Number(literal) => Some(Expr::new_literal(
                self.token.info,
                Lit::new_number(self.token.info, literal),
            )),
            TokenType::Boolean(literal) => Some(Expr::new_literal(
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
                parse_break_return!(
                    self,
                    self.in_loop,
                    new_break,
                    "expected expression after break keyword",
                    "break can only be used inside a loop"
                )
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
            TokenType::Return { .. } => {
                parse_break_return!(
                    self,
                    self.in_function,
                    new_return,
                    "expected expression after return keyword",
                    "return can only be used inside a function"
                )
            }
            TokenType::List => Some(self.parse_list()),
            TokenType::Create => Some(self.parse_var()),
            TokenType::Identifier(mut name) => {
                // TODO: check for car and cdr
                while self.peek().token_type == TokenType::Dot {
                    self.advance("parse_from_token - dot");
                    self.advance("parse_from_token - dot looking for car or cdr");
                    match self.token.token_type.clone() {
                        TokenType::Car => {
                            name.push_str(".car");
                        }
                        TokenType::Cdr => {
                            name.push_str(".cdr");
                        }
                        tt => {
                            error(
                                self.token.info.line,
                                format!("expected car or cdr after dot, found {tt}"),
                            );
                        }
                    }
                }
                Some(Expr::new_identifier(
                    self.token.info,
                    Ident::new(self.token.info, IdentType::Var(name)),
                ))
            }
            // built in functions
            TokenType::BuiltinFunction(name) => Some(Expr::new_identifier(
                self.token.info,
                Ident::new(self.token.info, IdentType::Builtin(name)),
            )),
            // fn identifiers
            TokenType::FunctionIdentifier(name) => Some(Expr::new_identifier(
                self.token.info,
                Ident::new(self.token.info, IdentType::FnIdent(name)),
            )),
            TokenType::ModuleIdentifier(m) => {
                // advance tokens until function identifier is reached
                // each module identifier is followed by a plus sign
                let mut module_name = String::new();
                self.check_next(
                    "expected plus symbol after module identifier",
                    &TokenType::PlusSymbol,
                    "parse_from_token - module identifier",
                );
                self.advance("parse_from_token - module identifier");
                module_name.push_str(&format!("{m}+"));
                while !self.tokens.is_empty() && self.peek().token_type == TokenType::PlusSymbol {
                    self.advance("parse_from_token - module identifier");
                    match self.token.token_type.clone() {
                        TokenType::ModuleIdentifier(name) => {
                            module_name.push(name);
                            module_name.push('+');
                        }
                        _ => error(self.token.info.line, "expected module identifier"),
                    }
                    self.advance("parse_from_token - module identifier");
                }
                // check if we have a function identifier
                match self.token.token_type.clone() {
                    TokenType::FunctionIdentifier(name) => {
                        module_name.push_str(&name);
                        Some(Expr::new_identifier(
                            self.token.info,
                            Ident::new(self.token.info, IdentType::FnIdent(module_name)),
                        ))
                    }
                    _ => error(self.token.info.line, "expected function identifier"),
                }
            }
            // return/break without expression
            TokenType::QuestionMark if self.in_stopper => Some(Expr::new_literal(
                self.token.info,
                Lit::new_hempty(self.token.info),
            )),
            // we hit a keyword
            keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                todo!("keyword: {:?}", keyword)
            }
            // we hit a function
            keyword => {
                todo!("keyword: {:?}", keyword)
            }
        }
    }

    fn parse_var(&mut self) -> Expr<'a> {
        self.advance("parse_from_token - create");
        let name = match self.token.token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => error(self.token.info.line, "expected identifier after create"),
        };
        self.check_next(
            "expected with after create",
            &TokenType::With,
            "parse_from_token - create",
        );
        Expr::new_var(
            self.token.info,
            Var::new(
                self.token.info,
                name.clone(),
                if let Some(s) = self.parse_from_token_advance() {
                    s
                } else {
                    error(
                        self.token.info.line,
                        format!("expected expression after for variable {name}"),
                    );
                },
            ),
        )
    }

    fn parse_from_token_advance(&mut self) -> Option<Expr<'a>> {
        self.advance("parse_from_token - start");
        self.parse_from_token()
    }

    fn parse_parenthissized(&mut self) -> Expr<'a> {
        // save the line number because the next token can be on the next line
        let start_line = self.token.info.line;
        let mut args = Vec::new();
        while self.token.token_type != TokenType::RightParen {
            if let Some(token) = self.parse_from_token_advance() {
                args.push(token);
            }
        }
        // check printing indicator (<) no print (>) print (>>) print no newline
        self.advance("parse_parenthissized - looking for printing indicator");
        match self.token.token_type {
            TokenType::LessThanSymbol => Expr::new_call(
                Info::new(self.file_path, start_line, self.token.info.line),
                FnCall::new(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    args,
                    PrintType::None,
                ),
            ),
            TokenType::GreaterThanSymbol => {
                // TODO: check if next token is a > if it is use PrintType::NoNewline
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
            TokenType::FunctionIdentifier(name) => {
                // this is not validated in the lexer because it is not possible to know if the function identifier is being used to define a function or to call a function
                // because if is a a call it can have modules seperated by + (the module operator)
                if name.chars().count() > 1 {
                    error(
                        self.token.info.line,
                        format!("function name {name} can only be one character long"),
                    );
                }
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
                self.check_next(
                    "Expected ⧼ after * in function definition",
                    &TokenType::CodeBlockBegin,
                    "parse_function_body",
                );
            }
            TokenType::Number(literal) => {
                if literal.round() != literal {
                    error(self.token.info.line, "Expected integer number of arguments");
                }
                arg_count = literal as usize;
                if self.peek().token_type == TokenType::Star {
                    extra_args = true;
                    self.advance("parse_function_body");
                }
                self.check_next(
                    &format!(
                        "Expected ⧼ after {} in function definition",
                        if extra_args { "*" } else { "number" }
                    ),
                    &TokenType::CodeBlockBegin,
                    "parse_function_body",
                );
            }
            TokenType::CodeBlockBegin => {}
            _ => {
                error(
                    self.token.info.line,
                    "Expected number, * or ⧼ in function definition",
                );
            }
        };
        self.in_function = true;
        let body = self.parse_list_exprs();
        self.in_function = false;
        Lambda::new(
            Info::new(self.file_path, self.token.info.line, self.token.info.line),
            arg_count,
            extra_args,
            body,
        )
    }

    fn parse_loop(&mut self) -> Loop<'a> {
        let start_line = self.token.info.line;
        self.check_next(
            "expected ⧼ after loop keyword found",
            &TokenType::CodeBlockBegin,
            "parse_loop",
        );
        self.in_loop = true;
        let loop_exprs = self.parse_list_exprs();
        self.in_loop = false;
        println!("loop parsed");
        Loop::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            loop_exprs,
        )
    }

    fn parse_if(&mut self) -> If<'a> {
        let start_line = self.token.info.line;
        self.check_next(
            "expected {{ after if keyword",
            &TokenType::LeftBrace,
            "parse if",
        );
        let cond = if let Some(expr) = self.parse_from_token_advance() {
            expr
        } else {
            error(self.token.info.line, "expected expression in conditonal")
        };
        self.check_next(
            "expected }} after conditional expression",
            &TokenType::RightBrace,
            "parse if",
        );
        self.check_next(
            "expected ⧼ after conditional expression",
            &TokenType::CodeBlockBegin,
            "parse if",
        );
        let if_then_exprs = self.parse_list_exprs();
        // check if there is an else block
        self.check_next(
            "expected `else` after if expressions",
            &TokenType::Else,
            "parse if - else",
        );
        self.check_next(
            "expected ⧼ after conditional expression",
            &TokenType::CodeBlockBegin,
            "parse if - else",
        );
        let else_exprs = self.parse_list_exprs();
        If::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            cond,
            if_then_exprs,
            else_exprs,
        )
    }

    fn check_next(&mut self, message: &str, token_type: &TokenType<'_>, advancer: &str) {
        self.advance(advancer);
        if &self.token.token_type != token_type {
            error(
                self.token.info.line,
                format!("{} found {}", message, self.token.token_type),
            );
        }
    }

    fn parse_list(&mut self) -> Expr<'a> {
        let start_line = self.token.info.line;
        self.advance("parse list");
        match self.token.token_type.clone() {
            TokenType::LeftBracket => Expr::new_list(
                Info::new(self.file_path, start_line, self.token.info.line),
                self.parse_list_inner(),
            ),
            TokenType::Identifier(name) => {
                self.check_next(
                    "expected with keyword after list identifier",
                    &TokenType::With,
                    "parse list",
                );
                self.check_next(
                    "expected [ after with keyword",
                    &TokenType::LeftBracket,
                    "parse list",
                );
                Expr::new_var(
                    Info::new(self.file_path, start_line, self.token.info.line),
                    Var::new(
                        Info::new(self.file_path, start_line, self.token.info.line),
                        name,
                        Expr::new_list(
                            Info::new(self.file_path, start_line, self.token.info.line),
                            self.parse_list_inner(),
                        ),
                    ),
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
        self.check_next(
            "expected ] after list item",
            &TokenType::RightBracket,
            "parse list inner",
        );
        return List::new(
            Info::new(self.file_path, start_line, self.token.info.line),
            car,
            cdr,
        );
    }

    fn parse_list_exprs(&mut self) -> Vec<Expr<'a>> {
        let mut exprs = vec![];
        let in_loop = self.in_loop;
        let in_function = self.in_function;
        while self.peek().token_type != TokenType::CodeBlockEnd {
            self.in_loop = in_loop;
            self.in_function = in_function;
            if let Some(expr) = self.parse_from_token_advance() {
                exprs.push(expr);
            }
        }
        self.advance("parse list exprs");
        exprs
    }

    #[allow(unused_variables)]
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

                if ![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                    .contains(&self.tokens[self.current_position + 1].token_type)
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

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current_position]
    }
}
