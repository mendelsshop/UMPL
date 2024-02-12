pub(crate) mod rules;
use crate::{
    error::error,
    parser::rules::{
        BlockNode, Break, BreakNode, CallNode, Continue, ContinueNode, Declaration,
        DeclarationNode, Function, FunctionNode, If, IfNode, Loop, LoopNode, PrintType, Return,
        ReturnNode,
    },
    token::{Token, TokenType},
};
use log::{debug, info, warn};

use self::rules::{Ast, Block, Call, Identifier, IdentifierNode, Literal, LiteralNode};

pub struct Parser {
    paren_count: usize,
    weird_bracket_count: usize,
    current_position: usize,
    tokens: Vec<Token>,
    token: Token,
    done: bool,
    in_function: bool,
    in_loop: bool,
    variables: Vec<String>,
    filename: String,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, name: String) -> Self {
        Self {
            paren_count: 0,
            current_position: 0,
            tokens,
            token: Token {
                token_type: TokenType::EOF,
                line: 0,
                lexeme: String::new(),
                filename: name.clone(),
            },
            done: false,
            weird_bracket_count: 0,
            in_function: false,
            in_loop: false,
            variables: Vec::new(),
            filename: name,
        }
    }

    pub fn advance(&mut self, fn_name: &str) {
        match self.tokens[self.current_position].token_type {
            TokenType::Return { .. } => {
                if self.in_function {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].line,
                        "Return statement outside of function",
                    );
                }
            }
            TokenType::Break | TokenType::Continue => {
                if self.in_loop {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].line,
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
                        self.tokens[self.current_position].line,
                        "unmatched right parenthesis",
                    );
                }
                self.paren_count -= 1;
                if self.paren_count == 0
                    && !(vec![TokenType::GreaterThanSymbol, TokenType::LessThanSymbol]
                        .contains(&self.tokens[self.current_position + 1].token_type))
                {
                    error(
                        self.tokens[self.current_position].line,
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
                if self.paren_count == 0 {
                    self.token = self.tokens[self.current_position].clone();
                } else {
                    error(
                        self.tokens[self.current_position].line,
                        "greater than symbol (>) or less than symbol (<) not allowed in middle of expression",
                    );
                }
            }
            _ => {
                self.token = self.tokens[self.current_position].clone();
            }
        };
        info!("{}", self.paren_count); //
        info!("new token: {} in function {}", self.token, fn_name);
        self.current_position += 1;
    }

    pub fn parse(&mut self) -> Vec<Ast> {
        let mut program = Vec::new();
        // loop until we have no more self.tokens
        // in the loop, we use parse_from_tokens to parse the next expression
        // and add it to the program tree
        info!("{:?}", self.tokens);
        while !self.done {
            let expr = self.parse_from_token();
            if let Some(t) = expr {
                debug!("{:?}", t);
                program.push(t);
            }
        }
        info!("Done parsing");
        program
    }
    #[allow(clippy::too_many_lines)]
    fn parse_from_token(&mut self) -> Option<Ast> {
        self.advance("parse_from_token");
        info!("new iteration");
        if self.tokens.is_empty() {
            error(0, "no self.tokens found");
        }
        if self.done {
            return None;
        }
        info!("PARSEfromTOKEN {}", self.token);
        match self.token.token_type.clone() {
            TokenType::LeftParen => match self.after_left_paren() {
                Callorexpression::Expression(e) => Some(e),
                _ => error(self.token.line, "expected expression"),
            },
            TokenType::CodeBlockEnd => None,
            TokenType::CodeBlockBegin => {
                let block = self.parse_block_without_begin(BlockType::None);
                return Some(Ast::Block(block.clone()));
            }
            TokenType::Identifier { .. } => {
                error(self.token.line, "variable not allowed in this context");
            }
            keyword if crate::KEYWORDS.is_keyword(&keyword) => {
                info!("found keyword {}", self.token.token_type);
                match self.token.token_type.clone() {
                    TokenType::Potato => {
                        let start_line = self.token.line;
                        self.advance("parse_from_token after function looking for function name");
                        match self.token.token_type.clone() {
                            TokenType::FunctionIdentifier { name, path } if path.is_empty() => {
                                info!("function identifier found");
                                self.advance("parse_from_token after function name looking for function arguments");
                                // check if the next token is a number and save it in a vairable num_args
                                let num_of_args_and_extra: (f64, bool) = match self.token.token_type
                                {
                                    TokenType::Number { literal } => {
                                        if literal.trunc() == literal {
                                            self.advance("parse_from_token found number or args looking for function body");
                                            if self.token.token_type == TokenType::Star {
                                                self.advance("parse_from_token found star looking for function body");
                                                (literal, true)
                                            } else {
                                                (literal, false)
                                            }
                                        } else {
                                            error(
                                                self.token.line,
                                                format!("number expected in function declaration found floating point number literal with {literal}"),
                                            );
                                        }
                                    }
                                    TokenType::Star => {
                                        self.advance(
                                            "parse_from_token found star looking for function body",
                                        );
                                        (0.0, true)
                                    }
                                    TokenType::CodeBlockBegin => (0.0, false),
                                    _ => {
                                        error(
                                            self.token.line,
                                            format!("number expected after function identifier, found {}", self.token),
                                        );
                                    }
                                };
                                let function = self.parse_block_without_begin(BlockType::Function);
                                info!("int function declaration before code block");
                                debug!("new function {:?}", function);
                                Some(Ast::Function(Function::new(
                                    FunctionNode::new(
                                        name,
                                        num_of_args_and_extra.0,
                                        num_of_args_and_extra.1,
                                        function,
                                    ),
                                    start_line,
                                    self.token.line,
                                    self.filename.clone(),
                                )))
                            }
                            tokentype => {
                                error(
                                    self.token.line,
                                    format!("function identifier expected after \"potato\", found TokenType::{tokentype:?}"),
                                );
                            }
                        }
                    }
                    TokenType::List => {
                        self.advance("parse_from_token");
                        match self.token.token_type.clone() {
                            TokenType::Identifier { name } => {
                                info!("list identifier found");
                                self.advance("parse_from_token");
                                if self.token.token_type == TokenType::With {
                                    self.advance("parse_from_token");
                                    if self.token.token_type == TokenType::LeftBracket {
                                        self.advance("parse_from_token");
                                        info!("list with");
                                        let thing: Ast = self.parse_to_other_stuff();
                                        self.advance("parse_from_token");
                                        let thing1: Ast = self.parse_to_other_stuff();
                                        self.advance("  ");
                                        if self.token.token_type == TokenType::RightBracket {
                                            self.variables.push(name.clone());
                                            Some(Ast::Declaration(Declaration::new(
                                                DeclarationNode::new(
                                                    name,
                                                    &[thing, thing1],
                                                    self.token.line,
                                                ),
                                                self.token.line,
                                                // TODO: use constructer for located without
                                                // endline
                                                self.token.line,
                                                self.filename.clone(),
                                            )))
                                        } else {
                                            error(
                                                self.token.line,
                                                format!(
                                                    "right bracket expected after list, found {}",
                                                    self.token.token_type
                                                )
                                                .as_str(),
                                            );
                                        }
                                    } else {
                                        error(
                                            self.token.line,
                                            format!(
                                                "left bracket expected after \"with\", found {}",
                                                self.token.token_type
                                            )
                                            .as_str(),
                                        );
                                    }
                                } else {
                                    error(
                                        self.token.line,
                                        format!(
                                            "with keyword expected, found TokenType::{:?}",
                                            self.token.token_type
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            tokentype => {
                                error(
                                    self.tokens[1].line,
                                    format!(
                                        "identifier expected, after \"list\" found TokenType::{tokentype:?}"
                                    )
                                    .as_str(),
                                );
                            }
                        }
                    }
                    TokenType::Create => {
                        self.advance("parse_from_token");
                        match self.token.token_type.clone() {
                            TokenType::Identifier { name } => {
                                info!("create identifier found");
                                self.advance("parse_from_token");
                                if self.token.token_type == TokenType::With {
                                    self.advance("parse_from_token");
                                    info!("create identifier with {}", self.token.token_type);
                                    let thing = self.get_value();
                                    self.variables.push(name.clone());
                                    Some(Ast::Declaration(Declaration::new(
                                        DeclarationNode::new(name, &[thing], self.token.line),
                                        self.token.line,
                                        // TODO: use constructer for located without
                                        // endline
                                        self.token.line,
                                        self.filename.clone(),
                                    )))
                                } else {
                                    error(
                                        self.token.line,
                                        format!(
                                            "with keyword expected, found TokenType::{:?}",
                                            self.token.token_type
                                        )
                                        .as_str(),
                                    );
                                }
                            }
                            tokentype => {
                                error(
                                    self.token.line,
                                    format!(
                                        "identifier expected after \"create\", found TokenType::{tokentype:?}"
                                    )
                                        .as_str(),
                                );
                            }
                        }
                    }
                    TokenType::Loop => {
                        info!("loop found");
                        let start_line = self.token.line;
                        let loop_body = self.parse_block(BlockType::Loop);
                        info!("Done parsing loop body");
                        self.advance("parse_from_token after loop body looking for loop end");
                        Some(Ast::Loop(Loop::new(
                            LoopNode::new(loop_body),
                            start_line,
                            self.token.line,
                            self.filename.clone(),
                        )))
                    }
                    TokenType::If => {
                        let start_line = self.token.line;
                        self.advance("parse_from_token after if expecting left brace");
                        if self.token.token_type == TokenType::LeftBrace {
                            info!("if statement");
                            self.advance("parse_from_token finding condition");
                            let thing: Ast = match self.token.clone().token_type {
                                TokenType::Boolean { literal } => self.boolean(literal),
                                TokenType::LeftParen => match self.after_left_paren() {
                                    Callorexpression::Expression(thing) => thing,
                                    _ => {
                                        error(self.token.line, "call found expected expression");
                                    }
                                },
                                TokenType::Identifier { name } => Ast::Identifier(self.var(name)),
                                tokentype => {
                                    error(
                                        self.token.line,
                                        format!(
                                            "boolean expected, in if statement condition found TokenType::{tokentype:?}"
                                        )
                                            .as_str(),
                                    );
                                }
                            };
                            info!("after conditon if statement");
                            self.advance("parse_from_token if expecting left brace");
                            if self.token.token_type == TokenType::RightBrace {
                                let if_body = self.parse_block(BlockType::None);
                                self.advance("parse_from_token before else");
                                if self.token.token_type == TokenType::Else {
                                    let else_body = self.parse_block(BlockType::None);
                                    Some(Ast::If(If::new(
                                        IfNode::new(Box::new(thing), if_body, else_body),
                                        start_line,
                                        self.token.line,
                                        self.filename.clone(),
                                    )))
                                } else {
                                    error(
                                        self.token.line,
                                        "else keyword expected after if statement",
                                    );
                                }
                            } else {
                                error(self.token.line, "right brace expected after if condition");
                            }
                        } else {
                            error(
                                self.token.line,
                                format!(
                                    "{{ expected after \"if\" found TokenType::{:?}",
                                    self.token.token_type
                                )
                                .as_str(),
                            );
                        }
                    }
                    TokenType::Return { .. } => {
                        if self.tokens[self.current_position].token_type == TokenType::Colon {
                            self.advance("parse_from_token return expecting expression");
                            return Some(Ast::Return(Return::new(
                                ReturnNode::new_empty(),
                                self.token.line,
                                // TODO: empty end line constructor
                                self.token.line,
                                self.filename.clone(),
                            )));
                        }
                        self.advance("parse_from_token return expecting expression");
                        let thing = self.parse_to_other_stuff();
                        Some(Ast::Return(Return::new(
                            ReturnNode::new(thing),
                            self.token.line,
                            // TODO: empty end line constructor
                            self.token.line,
                            self.filename.clone(),
                        )))
                    }
                    TokenType::Break => {
                        info!("break statement");
                        Some(Ast::Break(Break::new_single_line(
                            BreakNode::new(),
                            self.token.line,
                            self.filename.clone(),
                        )))
                    }
                    TokenType::Continue => {
                        info!("continue statement");
                        Some(Ast::Continue(Continue::new_single_line(
                            ContinueNode::new(),
                            self.token.line,
                            self.filename.clone(),
                        )))
                    }
                    _ => {
                        error(
                            self.token.line,
                            "keyword not allowed in expression before left parenthesis",
                        );
                    }
                }
            }
            _ => {
                error(
                    self.token.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn parse_block(&mut self, kind: BlockType) -> Block {
        info!("parsing code block");
        self.advance("loocking for code block begin");

        if self.token.token_type == TokenType::CodeBlockBegin {
            self.parse_block_without_begin(kind)
        } else {
            error(
                self.token.line,
                "code block begin expected for start of block",
            )
        }
    }
    fn parse_block_without_begin(&mut self, kind: BlockType) -> Block {
        let set_value = |this: &mut Self, val| match kind {
            BlockType::Loop => this.in_loop = val,
            BlockType::Function => this.in_function = val,
            _ => (),
        };
        let start_line = self.token.line;
        let mut block: Vec<Ast> = Vec::new();
        while self.tokens[self.current_position].token_type != TokenType::CodeBlockEnd {
            set_value(self, true);
            if let Some(t) = self.parse_from_token() {
                block.push(t);
            }
        }
        self.advance("parse_from_token after block, body looking for block end");
        set_value(self, false);
        debug!("new block {:?}", block);
        Block::new(
            BlockNode::new(block),
            start_line,
            self.token.line,
            self.token.filename.clone(),
        )
    }

    fn after_left_paren(&mut self) -> Callorexpression {
        let start_line = self.token.line;
        if self.paren_count == 1 {
            info!("found expresssion");
            self.advance("after_left_paren expression");
            let mut stuff = self.parse_to_stuff();
            info!("done parsing expression {}", stuff);
            self.advance("after left paren expr");
            if self.token.token_type == TokenType::RightParen {
                info!("right paren found");
            } else {
                error(self.token.line, "right parenthesis expected");
            }
            self.advance("after left paren expr");
            info!("found express");
            let prints = match self.token.token_type {
                TokenType::GreaterThanSymbol => PrintType::PrintLn,
                TokenType::LessThanSymbol => PrintType::None,
                _ => {
                    error(
                        self.token.line,
                        "greater than symbol or less than symbol expected",
                    );
                }
            };
            let prints = if prints == PrintType::PrintLn {
                match self.tokens[self.current_position].token_type {
                    TokenType::GreaterThanSymbol => {
                        self.advance("after left paren expr");
                        PrintType::Print
                    }
                    _ => prints,
                }
            } else {
                prints
            };
            warn!("{:?}", prints);
            stuff.set_print(prints);
            Callorexpression::Expression(stuff)
        } else {
            self.advance("after left paren");
            if self.token.token_type == TokenType::New {
                self.advance("after left paren");
                match self.token.token_type {
                    TokenType::FunctionIdentifier { .. } => {}
                    ref tt => {
                        error(
                            self.token.line,
                            &format!("function identifier expected after new found {tt}"),
                        );
                    }
                }
            }
            let keyword: TokenType = self.token.token_type.clone();
            let line: i32 = self.token.line;
            info!("found call {}", keyword);
            self.advance("after left paren");
            let mut args = Vec::new();

            while self.token.token_type != TokenType::RightParen {
                info!("looking for args");
                args.push(self.parse_to_stuff());
                self.advance("after left paren");
            }
            Callorexpression::Call(Call::new(
                CallNode::new(keyword, args),
                start_line,
                line,
                self.filename.clone(),
            ))
        }
    }

    fn var(&mut self, name: String) -> Identifier {
        if name.starts_with('$') && self.in_function {
            if self.tokens[self.current_position].token_type == TokenType::With {
                error(
                    self.tokens[self.current_position].line,
                    "function arguments are immutable",
                );
            } else {
                self.ident(name)
            }
        } else if self.tokens[self.current_position].token_type == TokenType::Dot {
            self.advance("Var");
            self.advance("Var");
            if let TokenType::Car | TokenType::Cdr = self.token.token_type {
                // make a string with the name + . and
                info!("found dot {}", self.token.token_type);
                let name: String =
                    name + "." + &format!("{:?}", self.token.token_type).to_lowercase();
                self.ident(name)
            } else {
                error(self.token.line, "car or Cdr expected after dot");
            }
        } else {
            self.ident(name)
        }
    }
    fn get_value(&mut self) -> Ast {
        match self.token.token_type.clone() {
            TokenType::Number { literal } => self.number(literal),
            TokenType::String { literal } => self.string(literal),
            TokenType::Hempty => self.hempty(),
            TokenType::Boolean { literal } => self.boolean(literal),
            TokenType::LeftParen => match self.after_left_paren() {
                Callorexpression::Expression(expression) => expression,
                _ => error(
                    self.token.line,
                    "expression expected after left parenthesis, found call",
                ),
            },
            TokenType::Identifier { name } => Ast::Identifier(self.var(name)),
            tokentype => {
                error(
                    self.token.line,
                    format!("identifier expected, after \"create\" found TokenType::{tokentype:?}")
                        .as_str(),
                );
            }
        }
    }

    fn parse_to_stuff(&mut self) -> Ast {
        info!("parsing stuff");
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                info!("found left paren");
                // self.advance("parse to stuff");
                match self.after_left_paren() {
                    Callorexpression::Call(call) => Ast::Call(call),
                    Callorexpression::Expression(a) => error(
                        self.token.line,
                        format!("call expected after left parenthesis found {a:?}"),
                    ),
                }
            }
            TokenType::Number { literal } => self.number(literal),
            TokenType::String { literal } => self.string(literal),
            TokenType::Hempty => self.hempty(),
            TokenType::Boolean { literal } => self.boolean(literal),
            TokenType::Identifier { name } => Ast::Identifier(self.var(name)),
            TokenType::Create => {
                todo!("variables created in an expression");
            }
            TokenType::If => {
                todo!("if statements in an expression");
            }
            TokenType::Loop => {
                todo!("loops in an expression");
            }
            TokenType::Potato => {
                todo!("function definitions in an expression");
            }
            TokenType::List => {
                todo!("list definitions in an expression");
            }
            _ => {
                error(
                    self.token.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }

    fn number(&self, literal: f64) -> Ast {
        Ast::Literal(Literal::new_single_line(
            LiteralNode::new_number(literal),
            self.token.line,
            self.filename.clone(),
        ))
    }

    fn string(&self, literal: String) -> Ast {
        Ast::Literal(Literal::new_single_line(
            LiteralNode::new_string(literal),
            self.token.line,
            self.filename.clone(),
        ))
    }
    fn boolean(&self, literal: bool) -> Ast {
        Ast::Literal(Literal::new_single_line(
            LiteralNode::new_boolean(literal),
            self.token.line,
            self.filename.clone(),
        ))
    }
    fn hempty(&self) -> Ast {
        Ast::Literal(Literal::new_single_line(
            LiteralNode::new_hempty(),
            self.token.line,
            self.filename.clone(),
        ))
    }
    fn ident(&self, ident: String) -> Identifier {
        Identifier::new_single_line(
            IdentifierNode::new(ident),
            self.token.line,
            self.filename.clone(),
        )
    }
    fn parse_to_other_stuff(&mut self) -> Ast {
        match self.token.token_type.clone() {
            TokenType::LeftParen => {
                // self.advance("parse to other stuff");
                match self.after_left_paren() {
                    Callorexpression::Expression(expression) => expression,
                    _ => error(
                        self.token.line,
                        "expression expected after left parenthesis, found call",
                    ),
                }
            }
            TokenType::Number { literal } => self.number(literal),
            TokenType::String { literal } => self.string(literal),
            TokenType::Hempty => self.hempty(),
            TokenType::Boolean { literal } => self.boolean(literal),
            TokenType::Identifier { name } => Ast::Identifier(self.var(name)),
            _ => {
                error(
                    self.token.line,
                    format!("{:?} not allowed in this context", self.token.token_type),
                );
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    None,
    Loop,
    Function,
}
#[derive(PartialEq, Clone)]
pub enum Callorexpression {
    Call(Call),
    Expression(Ast),
}
