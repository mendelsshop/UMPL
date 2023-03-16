use crate::{
    error::error,
    parser::rules::{FnCall, PrintType},
    token::{Info, Position, Token, TokenType},
};

use self::rules::{
    Accesor, Cons, Expr, ExprType, FnDef, Ident, IdentType, If, Interlaced, Lambda, Lit, Loop, Var,
};
pub(crate) mod rules;

macro_rules! parse_car_cdr {
    ($self:ident, $var:expr, $fntype:ident) => {{
        let mut cars_and_cdrs = vec![];
        while $self.peek().token_type == TokenType::Dot {
            $self.advance("parse_from_token - dot");
            $self.advance("parse_from_token - dot looking for car or cdr");
            match $self.token.token_type.clone() {
                TokenType::Car => {
                    cars_and_cdrs.push(Accesor::Car);
                }
                TokenType::Cdr => {
                    cars_and_cdrs.push(Accesor::Cdr);
                }
                tt => {
                    error(
                        $self.token.info,
                        format!("expected car or cdr after dot, found {tt}"),
                    );
                }
            }
        }
        Expr::new_identifier(
            $self.token.info,
            Ident::new(
                $self.token.info,
                IdentType::$fntype(Interlaced::new($var, cars_and_cdrs)),
            ),
        )
    }};
}
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
    info: Info::new("", Position::new(0, 0), Position::new(0, 0)),
    lexeme: String::new(),
};

macro_rules! parse_break_return {
    ($self:ident, $ok:expr, $new_method:ident, $err1:expr, $err2:expr) => {
        if $ok {
            let expr = Some(Expr::$new_method(
                $self.token.info,
                $self
                    .parse_from_token_advance()
                    .unwrap_or_else(|| error($self.token.info, $err1)),
            ));
            expr
        } else {
            error($self.token.info, $err2);
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
            error(START_TOKEN.info, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        match self.token.token_type.clone() {
            // we have entered an expression
            TokenType::CallBegin => Some(self.parse_parenthissized()),
            // we have entered a function
            TokenType::Potato => self.parse_function(),
            // TokenType::CodeBlockBegin => self.parse_function_body(),
            TokenType::CodeBlockEnd | TokenType::CallEnd => None,
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
                    error(self.token.info, "continue can only be used inside a loop");
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
            TokenType::Identifier(name) => Some(parse_car_cdr!(self, name, Var)),
            // built in functions
            TokenType::BuiltinFunction(name) => Some(Expr::new_identifier(
                self.token.info,
                Ident::new(self.token.info, IdentType::Builtin(name)),
            )),
            // function parameters
            TokenType::FunctionArgument(num) => Some(parse_car_cdr!(self, num, FnParam)),
            // fn identifiers
            TokenType::FunctionIdentifier(name) => Some(Expr::new_identifier(
                self.token.info,
                Ident::new(
                    self.token.info,
                    IdentType::FnIdent(Interlaced::new(name, vec![])),
                ),
            )),
            TokenType::ModuleIdentifier(_) => Some(self.parse_function_path()),

            // if we hit any other token type we have an error
            keyword => {
                error(self.token.info, format!("unexpected token {keyword}"));
            }
        }
    }

    fn parse_function_path(&mut self) -> Expr<'a> {
        // advance tokens until function identifier is reached
        // each module identifier is followed by a plus sign
        let mut modules = vec![];
        while self.peek().token_type == TokenType::PlusSymbol
            || matches!(self.token.token_type, TokenType::FunctionIdentifier(_))
        {
            match self.token.token_type.clone() {
                TokenType::ModuleIdentifier(name) => {
                    modules.push(name);
                }
                TokenType::FunctionIdentifier(name) => {
                    return Expr::new_identifier(
                        self.token.info,
                        Ident::new(
                            self.token.info,
                            IdentType::FnIdent(Interlaced::new(name, modules)),
                        ),
                    );
                }
                tt => error(
                    self.token.info,
                    format!("expected module identifier or function identifier, found {tt}"),
                ),
            }
            // advance twice because we have a checked two tokens the module identifier and the plus sign
            self.advance("parse_from_token - module identifier loop");
            self.advance("parse_from_token - module identifier loop");
        }
        // check if we have a function identifier
        error(self.token.info, "expected function identifier")
    }

    fn parse_var(&mut self) -> Expr<'a> {
        self.advance("parse_from_token - create");
        let name = match self.token.token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => error(self.token.info, "expected identifier after create"),
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
                        self.token.info,
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
        let start_line = self.token.info.begin;
        let mut args = Vec::new();
        while self.token.token_type != TokenType::CallEnd {
            if let Some(token) = self.parse_from_token_advance() {
                args.push(token);
            }
        }
        // check printing indicator (<) no print (>) print (>>) print no newline
        self.advance("parse_parenthissized - looking for printing indicator");
        match self.token.token_type {
            TokenType::LessThanSymbol => Expr::new_call(
                Info::new(self.file_path, start_line, self.token.info.end),
                FnCall::new(
                    Info::new(self.file_path, start_line, self.token.info.end),
                    args,
                    PrintType::None,
                ),
            ),
            TokenType::GreaterThanSymbol => {
                // TODO: check if next token is a > if it is use PrintType::NoNewline
                Expr::new_call(
                    Info::new(self.file_path, start_line, self.token.info.end),
                    FnCall::new(
                        Info::new(self.file_path, start_line, self.token.info.end),
                        args,
                        PrintType::Newline,
                    ),
                )
            }
            _ => {
                // should never happen as this is caught in the advance method
                error(self.token.info, "Expected printing indicator");
            }
        }
    }

    fn parse_function(&mut self) -> Option<Expr<'a>> {
        let start_line = self.token.info.begin;
        self.advance("parse_function - start");
        match self.token.token_type.clone() {
            TokenType::FunctionIdentifier(name) => {
                // this is not validated in the lexer because it is not possible to know if the function identifier is being used to define a function or to call a function
                // because if is a a call it can have modules seperated by + (the module operator)
                let fn_def = self.parse_named_function(name, vec![]);
                Some(Expr::new_fn(
                    Info::new(self.file_path, start_line, self.token.info.end),
                    fn_def,
                ))
            }
            TokenType::Star | TokenType::CodeBlockBegin | TokenType::Number { .. } => {
                let lambda = self.parse_function_body();
                Some(Expr::new_lambda(
                    Info::new(self.file_path, start_line, self.token.info.end),
                    lambda,
                ))
            }
            tt => error(
                self.token.info,
                format!("expected function name, number, * or ⧼ found {tt} in function defintion"),
            ),
        }
    }

    fn parse_named_function(&mut self, name: char, modules: Vec<char>) -> FnDef<'a> {
        let start_line = self.token.info.begin;
        self.advance("parsed_named_function");
        let body = match self.token.token_type.clone() {
            TokenType::Number { .. } | TokenType::Star | TokenType::CodeBlockBegin => {
                self.parse_function_body()
            }
            tt => error(
                self.token.info,
                format!("expected number, * or ⧼ found {tt} in function defintion"),
            ),
        };
        FnDef::new(
            Info::new(self.file_path, start_line, self.token.info.end),
            name,
            modules,
            body,
        )
    }

    /// not only used to parse the body of a function but also to parse anonymous functions
    fn parse_function_body(&mut self) -> Lambda<'a> {
        let mut arg_count = 0;
        let mut extra_args = false;
        let start_line = self.token.info.begin;
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
                    error(self.token.info, "Expected integer number of arguments");
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
                    self.token.info,
                    "Expected number, * or ⧼ in function definition",
                );
            }
        };
        self.in_function = true;
        let body = self.parse_list_exprs();
        self.in_function = false;
        Lambda::new(
            Info::new(self.file_path, start_line, self.token.info.end),
            arg_count,
            extra_args,
            body,
        )
    }

    fn parse_loop(&mut self) -> Loop<'a> {
        let start_line = self.token.info.begin;
        self.check_next(
            "expected ⧼ after loop keyword found",
            &TokenType::CodeBlockBegin,
            "parse_loop",
        );
        self.in_loop = true;
        let loop_exprs = self.parse_list_exprs();
        self.in_loop = false;
        // println!("loop parsed");
        Loop::new(
            Info::new(self.file_path, start_line, self.token.info.end),
            loop_exprs,
        )
    }

    fn parse_if(&mut self) -> If<'a> {
        let start_line = self.token.info.begin;
        self.check_next(
            "expected {{ after if keyword",
            &TokenType::LeftBrace,
            "parse if",
        );
        let cond = if let Some(expr) = self.parse_from_token_advance() {
            expr
        } else {
            error(self.token.info, "expected expression in conditonal")
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
            Info::new(self.file_path, start_line, self.token.info.end),
            cond,
            if_then_exprs,
            else_exprs,
        )
    }

    fn check_next(&mut self, message: &str, token_type: &TokenType, advancer: &str) {
        self.advance(advancer);
        if &self.token.token_type != token_type {
            error(
                self.token.info,
                format!("{} found {}", message, self.token.token_type),
            );
        }
    }

    fn parse_list(&mut self) -> Expr<'a> {
        let start_line = self.token.info.begin;
        self.advance("parse list");
        match self.token.token_type.clone() {
            TokenType::LeftBracket => Expr::new_cons(
                Info::new(self.file_path, start_line, self.token.info.end),
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
                    Info::new(self.file_path, start_line, self.token.info.end),
                    Var::new(
                        Info::new(self.file_path, start_line, self.token.info.end),
                        name,
                        Expr::new_cons(
                            Info::new(self.file_path, start_line, self.token.info.end),
                            self.parse_list_inner(),
                        ),
                    ),
                )
            }
            _ => {
                error(
                    self.token.info,
                    format!(
                        "expected [ or identifier after list keyword found {}",
                        self.token.token_type
                    ),
                );
            }
        }
    }

    fn parse_list_inner(&mut self) -> Cons<'a, Expr<'a>> {
        let start_line = self.token.info.begin;
        self.advance("parse list inner");
        let car = if let Some(expr) = self.parse_from_token() {
            expr
        } else {
            error(self.token.info, "expected expression in list")
        };
        self.advance("parse list inner");
        let cdr = if let Some(expr) = self.parse_from_token() {
            expr
        } else {
            error(self.token.info, "expected expression in list")
        };
        self.check_next(
            "expected ] after list item",
            &TokenType::RightBracket,
            "parse list inner",
        );
        return Cons::new(
            Info::new(self.file_path, start_line, self.token.info.end),
            car,
            Some(cdr),
        );
    }

    fn parse_list_exprs(&mut self) -> Cons<'a, Expr<'a>> {
        let mut exprs = Cons::new_cdr_empty(
            self.token.info,
            Expr::new_literal(self.token.info, Lit::new_hempty(self.token.info)),
        );
        let in_loop = self.in_loop;
        let in_function = self.in_function;
        while self.peek().token_type != TokenType::CodeBlockEnd {
            self.in_loop = in_loop;
            self.in_function = in_function;
            if let Some(expr) = self.parse_from_token_advance() {
                exprs.set_cdr(expr);
            }
        }

        // remove hempty at the beginning if there is something after it
        let exprs = if let Some(expr) = &exprs.cdr {
            if let ExprType::Cons(cons) = &expr.expr {
                cons.clone()
            } else {
                exprs
            }
        } else {
            exprs
        };

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
                        self.tokens[self.current_position].info,
                        "Return statement outside of function",
                    );
                }
            }
            TokenType::Break | TokenType::Continue => {
                if self.in_loop {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].info,
                        "Break or continue statement outside of loop",
                    );
                }
            }
            TokenType::EOF => {
                self.done = true;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::CallBegin => {
                self.paren_count += 1;
                self.token = self.tokens[self.current_position].clone();
            }
            TokenType::CallEnd => {
                if self.paren_count == 0 {
                    error(
                        self.tokens[self.current_position].info,
                        "unmatched right parenthesis",
                    );
                }
                self.paren_count -= 1;

                if ![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                    .contains(&self.tokens[self.current_position + 1].token_type)
                {
                    error(
                        self.tokens[self.current_position].info,
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
                match (
                    self.token.token_type.clone(),
                    self.peek().token_type.clone(),
                ) {
                    (TokenType::CallEnd, TokenType::LessThanSymbol)
                    | (
                        TokenType::CallEnd | TokenType::GreaterThanSymbol,
                        TokenType::GreaterThanSymbol,
                    ) => {
                        self.token = self.tokens[self.current_position].clone();
                    }
                    _ => error(
                        self.tokens[self.current_position].info,
                        "expected call before print indicators: < or >",
                    ),
                }
            }
            _ => {
                self.token = self.tokens[self.current_position].clone();
            }
        };
        // println!("token: {:?} caller: {}", self.token, caller);
        self.current_position += 1;
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current_position]
    }
}
