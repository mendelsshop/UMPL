#![allow(dead_code)]

use std::iter::{self, empty};

use crate::interior_mut::RC;

pub enum TokenType {
    Number(f64),
    String(RC<str>),
    Symbol(RC<str>),
    Boolean(Boolean),
    KeyWord(KeyWord),
    /// open bracket used for lists, function
    OpenBidiBracket,
    /// close bracket used for lists, function
    CloseBidiBracket,
    /// bracket type used to denote a new scope
    OpenScope,
    /// bracket type used to denote a scope is done
    CloseScope,
    /// accessor for function argument
    ArgumentRef(usize),
    /// denotes that the expression following this must be quouted
    Quote,
    /// denotes that the expression following this must be a label
    Label,
    /// used for function signatues ie + or *
    Varidiac(Varidiac),
}

pub enum KeyWord {
    /// used for both return and break
    Stop,
    Contiue,
    Loop,
    ForEach,
    In,
    Function,
    Until,
    Do,
    Unless,
    If,
    Esle,
    OtherWise,
    When,
    Goto,
}

pub fn to_keyword(source: &str) -> Option<KeyWord> {
    match source {
        "stop" => Some(KeyWord::Stop),
        "continue" => Some(KeyWord::Contiue),
        "continue-doing" => Some(KeyWord::Loop),
        "go-through" => Some(KeyWord::ForEach),
        "of" => Some(KeyWord::In),
        "fanction" => Some(KeyWord::Function),
        "until" => Some(KeyWord::Until),
        "do" => Some(KeyWord::Do),
        "unless" => Some(KeyWord::Unless),
        "if" => Some(KeyWord::If),
        "else" => Some(KeyWord::Esle),
        "otherwise" => Some(KeyWord::OtherWise),
        "when" => Some(KeyWord::OtherWise),
        "goto" => Some(KeyWord::Goto),
        _ => None,
    }
}

pub struct Token {
    tt: TokenType,
    info: Info,
}

pub struct Info {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_coloumn: usize,
}

pub struct lexer {
    current_info: Info,
}

#[derive(Debug)]
pub enum ParseErrorType {
    EOF,
    Other(String),
    NotADigit(char),
    Mismatch(char, char),
    Fail,
    NotEnoughMatches,
    NoMatchFound,
    SatisfyMismatch(char),
}

#[derive(Debug)]
pub struct ParseError<'a> {
    kind: ParseErrorType,
    input: &'a str,
}

/// a parser for things (T) is a function of Strings
/// to a list of pairs of things (T) and strings
// type Parser<T> = dyn Fn(&str) -> Result<(T, &str), ParseError> + 'static;

// TODO reimplement char parsing with satisfy

pub fn digit() -> Box<Parser<usize>> {
    Box::new(|input: &str| {
        println!("{}`{input:?}` -> digit", indent());
        match input.chars().next() {
            Some(n) => match n.to_digit(10) {
                Some(d) => Ok((d as usize, input.split_at(1).1)),
                None => Err(ParseError {
                    kind: ParseErrorType::NotADigit(n),
                    input,
                }),
            },
            None => Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
            }),
        }
    })
}

pub fn char(looking_for: char) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        println!("{}`{input:?}` -> `{looking_for:?}`", indent());
        match input.chars().next() {
            Some(n) => {
                if n == looking_for {
                    Ok((n, input.split_at(n.len_utf8()).1))
                } else {
                    Err(ParseError {
                        kind: ParseErrorType::Mismatch(looking_for, n),
                        input,
                    })
                }
            }
            None => Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
            }),
        }
    })
}

pub fn integer() -> Box<Parser<usize>> {
    map(many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])), |input| {
        input.collect::<String>().parse().unwrap()
    })
}

pub fn satify(checker: impl Fn(char) -> bool + 'static + Clone) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        // println!("{}`{input:?}` -> `{looking_for:?}`", indent());
        match input.chars().next() {
            Some(n) => {
                if checker(n) {
                    println!("found match {n}");
                    Ok((n, input.split_at(n.len_utf8()).1))
                } else {
                    println!("not match {n}");
                    Err(ParseError {
                        kind: ParseErrorType::SatisfyMismatch(n),
                        input,
                    })
                }
            }
            None => Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
            }),
        }
    })
}

pub fn not_char(looking_for: char) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        println!("{}`{input:?}` -> !`{looking_for:?}`", indent());
        match input.chars().next() {
            Some(n) => {
                if n != looking_for {
                    Ok((n, input.split_at(n.len_utf8()).1))
                } else {
                    Err(ParseError {
                        kind: ParseErrorType::Mismatch(looking_for, n),
                        input,
                    })
                }
            }
            None => Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
            }),
        }
    })
}

fn take() -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        let chars = &mut input.chars();
        chars.next().map_or_else(
            || {
                Err(ParseError {
                    input: "",
                    kind: ParseErrorType::EOF,
                })
            },
            |char| Ok((char, input.split_at(char.len_utf8()).1)),
        )
    })
}

pub fn chain<T: 'static, U: 'static>(
    parser1: Box<Parser<T>>,
    parser2: Box<Parser<U>>,
) -> Box<Parser<(T, U)>> {
    Box::new(move |input: &str| {
        // println!("chain s `{input}`");
        let (res1, input) = parser1(input)?;
        // println!("chain m `{input}`");
        let (res2, input) = parser2(input)?;
        // println!("chain e `{input}`");
        Ok(((res1, res2), input))
    })
}

pub fn map<T: 'static, U: 'static, F: Fn(T) -> U + 'static + Clone>(
    parser: Box<Parser<T>>,
    map_fn: F,
) -> Box<Parser<U>> {
    Box::new(move |input| {
        // println!("map s `{input}`");
        let (ir, input) = parser(input)?;
        // println!("map e `{input}`");
        Ok((map_fn(ir), input))
    })
}

pub fn try_map<
    T: 'static,
    U: 'static,
    F: Fn(T) -> Result<U, ParseError<'static>> + 'static + Clone,
>(
    parser: Box<Parser<T>>,
    map_fn: F,
) -> Box<Parser<U>> {
    Box::new(move |input| {
        // println!("map s `{input}`");
        let (ir, input) = parser(input)?;
        // println!("map e `{input}`");
        map_fn(ir).map(|ir| (ir, input))
    })
}

pub fn alt<T: 'static>(parser1: Box<Parser<T>>, parser2: Box<Parser<T>>) -> Box<Parser<T>> {
    Box::new(move |input| {
        println!("alt s `{input}`");
        match parser1(input) {
            Ok((res, input)) => {
                println!("alt m `{input}`");
                Ok((res, input))
            }
            Err(_) => {
                println!("alt e `{input}`");
                parser2(input)
            }
        }
    })
}

pub fn opt<T: 'static>(parser: Box<Parser<T>>) -> Box<Parser<Option<T>>> {
    Box::new(move |input| match parser(input) {
        Ok(ok) => Ok((Some(ok.0), ok.1)),
        // TODO: error should return leftover substring
        Err(e) => Ok((None, e.input)),
    })
}

pub fn many<T: 'static>(
    parser: Box<Parser<T>>,
) -> Box<Parser<Option<Box<dyn Iterator<Item = T>>>>> {
    Box::new(move |mut input| {
        // println!("many s `{input}`");
        let mut init: Option<Box<dyn Iterator<Item = T>>> = None;
        while let Ok((v, new_input)) = parser(input) {
            input = new_input;
            // println!("many m`{input}`");
            let v = iter::once(v);
            init = match init {
                Some(old_v) => Some(Box::new(old_v.chain(v))),
                None => Some(Box::new(v)),
            };
        }
        Ok((init, input))
    })
}

pub fn keep_left<T: 'static, U: 'static>(
    left_parser: Box<Parser<T>>,
    right_parser: Box<Parser<U>>,
) -> Box<Parser<T>> {
    map(chain(left_parser, right_parser), |i| i.0)
}

pub fn keep_right<T: 'static, U: 'static>(
    left_parser: Box<Parser<T>>,
    right_parser: Box<Parser<U>>,
) -> Box<Parser<U>> {
    map(chain(left_parser, right_parser), |i| i.1)
}

pub fn inbetween<T: 'static, U: 'static, V: 'static>(
    left_parser: Box<Parser<T>>,
    middle_parser: Box<Parser<U>>,
    right_parser: Box<Parser<V>>,
) -> Box<Parser<U>> {
    keep_left(keep_right(left_parser, middle_parser), right_parser)
}

pub fn many1<T: 'static>(parser: Box<Parser<T>>) -> Box<Parser<Box<dyn Iterator<Item = T>>>> {
    let many = many(parser);
    Box::new(move |input| match many(input)? {
        (None, input) => Err(ParseError {
            kind: ParseErrorType::NotEnoughMatches,
            input,
        }),
        (Some(v), input) => Ok((v, input)),
    })
}

pub fn fail<T>() -> Box<Parser<T>> {
    Box::new(move |input| {
        Err(ParseError {
            kind: ParseErrorType::Fail,
            input,
        })
    })
}

pub fn unit<T: 'static + Clone>(val: T) -> Box<Parser<T>> {
    Box::new(move |input| Ok((val.clone(), input)))
}
pub fn seq<T: 'static>(parsers: Vec<Box<Parser<T>>>) -> Box<Parser<impl Iterator<Item = T>>> {
    Box::new(move |mut input| {
        let mut res: Box<dyn Iterator<Item = T>> = Box::new(empty());
        for parser in &parsers {
            let (res_part, new_input) = parser(input)?;
            input = new_input;
            res = Box::new(res.chain(iter::once(res_part)));
        }

        Ok((res, input))
    })
    // parsers
    //     .into_iter()
    //     .fold::<Box<Parser<Box<dyn Iterator<Item = T>>>>, _>(Box::new(move |input| -> Result<(Box<dyn Iterator<Item = _>>, _), _> {Ok((Box::new(empty()), input))}), |a, b| {
    //         Box::new(move |input: &str| -> Result<_, _> {
    //             let ((rest, first), input) = chain(a, b)(input)?;
    //             Ok((Box::new(rest.chain(iter::once(first))), input))
    //         })
    //     })
}

pub fn choice<T: 'static>(parsers: Vec<Box<Parser<T>>>) -> Box<Parser<T>> {
    // {
    //     let mut this = parsers.into_iter();
    //     let init = fail();
    //     let mut f = &alt;
    //     let mut accum = init;
    //     while let Some(x) = this.next() {
    //         accum = f(accum, x);
    //     }
    //     accum
    // }
    Box::new(move |input| {
        for parser in parsers.clone().into_iter() {
            println!("choice s `{input}`");
            match parser(input) {
                Ok(ok) => return Ok(ok),
                Err(_) => continue,
            }
        }
        fail()(input)
    })
}

pub fn not_choice<T: 'static>(parsers: Vec<Box<Parser<T>>>) -> Box<Parser<T>> {
    // {
    //     let mut this = parsers.into_iter();
    //     let init = fail();
    //     let mut f = &alt;
    //     let mut accum = init;
    //     while let Some(x) = this.next() {
    //         accum = f(accum, x);
    //     }
    //     accum
    // }
    Box::new(move |input| {
        let mut res = None;
        for parser in parsers.clone().into_iter() {
            println!("choice s `{input}`");
            res = Some(parser(input)?);
        }
        res.ok_or(ParseError {
            kind: ParseErrorType::NoMatchFound,
            input,
        })
    })
}

pub fn any_of(chars: impl IntoIterator<Item = char>) -> Box<Parser<char>> {
    choice(chars.into_iter().map(char).collect())
}

pub fn not_any_of(chars: impl IntoIterator<Item = char>) -> Box<Parser<char>> {
    not_choice(chars.into_iter().map(not_char).collect())
}

pub fn string(to_match: &str) -> Box<Parser<String>> {
    map(seq(to_match.chars().map(|c| char(c)).collect()), |chars| {
        chars.collect::<String>()
    })
}

pub fn sep<T: 'static, U: 'static>(
    parser: Box<Parser<T>>,
    delimeter: Box<Parser<U>>,
) -> Box<Parser<Option<Box<dyn Iterator<Item = T>>>>> {
    let rest = many(keep_right(delimeter, parser.clone_box()));
    Box::new(move |input| {
        let (first, new_input) = match parser(input) {
            Ok(v) => v,
            Err(e) => return Ok((None, e.input)),
        };
        let first = iter::once(first);
        let (rest, input) = rest(new_input)?;
        Ok(match rest {
            None => (Some(Box::new(first)), new_input),
            Some(v) => (Some(Box::new(first.chain(v))), input),
        })
    })
}

pub fn sep1<T: 'static, U: 'static>(
    parser: Box<Parser<T>>,
    delimeter: Box<Parser<U>>,
) -> Box<Parser<Box<dyn Iterator<Item = T>>>> {
    let sep = sep(parser, delimeter);
    Box::new(move |input| match sep(input)? {
        (None, input) => Err(ParseError {
            kind: ParseErrorType::NotEnoughMatches,
            input,
        }),
        (Some(v), input) => Ok((v, input)),
    })
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum Expr {
    Number(usize),
    MathOp(Box<Expr>, Op, Box<Expr>),
}

pub fn number() -> Box<Parser<Expr>> {
    map(digit(), Expr::Number)
}

pub fn op() -> Box<Parser<Op>> {
    alt(
        map(char('/'), |_| Op::Div),
        alt(
            map(char('*'), |_| Op::Mul),
            alt(map(char('+'), |_| Op::Add), map(char('-'), |_| Op::Sub)),
        ),
    )
}

#[test]
fn test() {
    // let input = "   (  1   +  1  )";
    // let expr = expr()(input).unwrap().0;
    // let res = eval(&expr);
    // println!("{res}");
    // let parsers = vec![char('"'), char('a')];
    // let p1 = seq(parsers);
    // // println!("done");
    // let res = p1("\"a").unwrap();
    // println!("'{}'", res.0.into_iter().collect::<String>());
    // let choi = choice(
    //     vec![
    //         any_of(vec!['b']),
    //         map(digit(), |number| char::from_u32(number as u32).unwrap()),
    //     ]
    //     .into_iter(),
    // );
    // let res = choi("b").unwrap();
    // let if_ = string("if");
    // let if_ = if_("if");
    // println!("{if_:?}");
    let sepped = sep1(digit(), char(','));
    // let res = sepped.clone_box()("1").unwrap().0.unwrap().collect::<Vec<_>>();
    let res2 = sepped("").unwrap().0.collect::<Vec<_>>();
    println!("{res2:?}")
}

fn white_space() -> Box<Parser<Option<Box<dyn Iterator<Item = char>>>>> {
    many(any_of([' ', '\n', '\t']))
}

pub fn expr() -> Box<Parser<Expr>> {
    Box::new(|input| keep_right(white_space(), alt(number(), op_expr()))(input))
}

fn op_expr() -> Box<Parser<Expr>> {
    map(
        // chain(
        //     char('('),
        //     chain(chain(expr(), keep_right(white_space(), op())), chain(expr(), char(')'))),
        // ),
        inbetween(
            char('('),
            chain(expr(), chain(keep_right(white_space(), op()), expr())),
            keep_right(white_space(), char(')')),
        ),
        |ir| Expr::MathOp(Box::new(ir.0), ir.1 .0, Box::new(ir.1 .1)),
    )
}

pub fn eval(input: &Expr) -> usize {
    match input {
        Expr::Number(n) => *n,
        Expr::MathOp(e1, op, e2) => {
            let e1 = eval(&*e1);
            let e2 = eval(&*e2);
            match op {
                Op::Add => e1 + e2,
                Op::Sub => e1 - e2,
                Op::Div => e1 / e2,
                Op::Mul => e1 * e2,
            }
        }
    }
}

pub trait CloneFn<T>: Fn(&str) -> Result<(T, &str), ParseError> {
    fn clone_box<'a>(&self) -> Box<dyn CloneFn<T> + 'a>
    where
        Self: 'a;
}

impl<T, F> CloneFn<T> for F
where
    F: Fn(&str) -> Result<(T, &str), ParseError> + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn CloneFn<T> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a> Clone for Box<dyn 'a + CloneFn<T>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

pub type Parser<T> = dyn CloneFn<T>;

#[derive(Debug)]
pub enum LispExpr {
    Number(usize),
    Symbol(String),
    List(Vec<LispExpr>),
    If(Box<LispExpr>, Box<LispExpr>, Box<LispExpr>),
}

static mut INDENT: usize = 0;
pub fn inc_indent() {
    unsafe { INDENT += 1 }
}
pub fn dec_indent() {
    unsafe { INDENT -= 1 }
}
pub fn indent() -> String {
    format!("{}{}", unsafe { INDENT }, " ".repeat(unsafe { INDENT * 4 }))
}

pub fn lispnumber() -> Box<Parser<LispExpr>> {
    Box::new(|input| {
        println!("{}number", indent());
        map(many1(digit()), |i| {
            let collect = &i.map(|i| i.to_string()).collect::<String>();
            LispExpr::Number(collect.parse().unwrap())
        })(input)
    })
}

pub fn lisplist() -> Box<Parser<LispExpr>> {
    Box::new(|input| {
        println!("{}list", indent());
        inc_indent();
        let res = inbetween(
            keep_left(char('('), white_space()),
            map(sep(lispexpr(), white_space()), |i| {
                LispExpr::List(i.map_or(vec![], Iterator::collect))
            }),
            keep_right(white_space(), char(')')),
        )(input);
        dec_indent();
        res
    })
}

pub fn lispif() -> Box<Parser<LispExpr>> {
    map(
        seq(vec![
            keep_right(string("if"), lispexpr()),
            keep_right(chain(white_space(), string("then")), lispexpr()),
            keep_right(chain(white_space(), string("else")), lispexpr()),
        ]),
        |mut i| {
            LispExpr::If(
                Box::new(i.next().unwrap()),
                Box::new(i.next().unwrap()),
                Box::new(i.next().unwrap()),
            )
        },
    )
}

pub fn lispsymbol() -> Box<Parser<LispExpr>> {
    Box::new(|input| {
        println!("{}sybmol", indent());
        map(many1(not_any_of(['\n', ' ', '\t', '.', '(', ')'])), |i| {
            LispExpr::Symbol(i.collect())
        })(input)
    })
}

pub fn lispexpr() -> Box<Parser<LispExpr>> {
    Box::new(|input| {
        println!("{}listexpr", indent());
        inc_indent();
        let res = keep_right(
            white_space(),
            choice([lispnumber(), lisplist(), lispif(), lispsymbol()].to_vec()),
        )(input);
        dec_indent();
        res
    })
}

#[test]
fn lisp() {
    // let sym  = lispsymbol();
    // sym("()").unwrap();
    let p = lispexpr();
    let res = p("(t55 if t then 6 else 5)");

    println!("{res:?}")
}
#[derive(Debug, Default, PartialEq)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(String),
    Scope(Vec<UMPL2Expr>),
    Ident(String),
    If(Box<UMPL2Expr>, Box<UMPL2Expr>, Box<UMPL2Expr>),
    Unless(Box<UMPL2Expr>, Box<UMPL2Expr>, Box<UMPL2Expr>),
    Stop(Box<UMPL2Expr>),
    Skip,
    Until(Box<UMPL2Expr>, Box<UMPL2Expr>),
    GoThrough(String, Box<UMPL2Expr>, Box<UMPL2Expr>),
    ContiueDoing(Box<UMPL2Expr>),
    Fanction(char, usize, Option<Varidiac>, Box<UMPL2Expr>),
    Application(Vec<UMPL2Expr>),
    #[default]
    Hempty,
}

#[derive(Debug, PartialEq)]
pub enum Boolean {
    /// &
    True,
    /// |
    False,
    /// ?
    Maybee,
}

#[derive(Debug, PartialEq)]
pub enum Varidiac {
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1,
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0,
}

impl Varidiac {
    fn from_char(c: char) -> Option<Varidiac> {
        match c {
            '*' => Some(Self::AtLeast0),
            '+' => Some(Self::AtLeast1),
            _ => None,
        }
    }
}

fn scope(p: Box<Parser<UMPL2Expr>>) -> Box<Parser<UMPL2Expr>> {
    inbetween(
        keep_right(white_space(), char('᚜')),
        map(many(p), |r| {
            UMPL2Expr::Scope(r.map_or_else(Vec::new, Iterator::collect))
        }),
        opt(keep_right(white_space(), char('᚛'))),
    )
}

fn umpl2expr() -> Box<Parser<UMPL2Expr>> {
    Box::new(|input| keep_right(white_space(), choice([literal(), stmt(), ident_umpl(), application()].to_vec()))(input))
}

fn application() -> Box<dyn CloneFn<UMPL2Expr>> {
    inbetween(
        keep_right(white_space(), any_of(call_start().iter().copied())),
        map(many(umpl2expr()), |r| {
            UMPL2Expr::Application(r.map_or_else(Vec::new, Iterator::collect))
        }),
        opt(keep_right(white_space(), any_of(call_end().iter().copied()))),
    )
}

pub fn parse_umpl(input: &str) -> Result<UMPL2Expr, ParseError> {
    umpl2expr()(input).map(|res| res.0)
}

fn literal() -> Box<Parser<UMPL2Expr>> {
    choice([boolean(), hexnumber(), stringdot()].to_vec())
}

fn boolean() -> Box<Parser<UMPL2Expr>> {
    choice(vec![
        map(string("&"), |_| UMPL2Expr::Bool(Boolean::True)),
        map(string("|"), |_| UMPL2Expr::Bool(Boolean::False)),
        map(string("?"), |_| UMPL2Expr::Bool(Boolean::Maybee)),
    ])
}

fn hexnumber() -> Box<Parser<UMPL2Expr>> {
    let digit = any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let hex_digit = choice([digit.clone(), any_of(['a', 'b', 'c', 'd', 'e', 'f'])].to_vec());
    let parese_num = |digit_type: Box<Parser<char>>| {
        try_map(
            chain(
                many(digit_type.clone()),
                opt(keep_right(char('%'), many1(digit_type))),
            ),
            |r| {
                let number = match r {
                    (None, None) => {
                        return Err(ParseError {
                            kind: ParseErrorType::Other("not a digit".to_string()),
                            input: "",
                        })
                    }
                    (None, Some(s)) => (String::new(), s.collect()),
                    (Some(s), None) => (s.collect(), String::new()),
                    (Some(s), Some(r)) => (s.collect(), r.collect()),
                };
                Ok(UMPL2Expr::Number(
                    format!("0x{}.{}", number.0, number.1)
                        .parse::<hexponent::FloatLiteral>()
                        .unwrap()
                        .into(),
                ))
            },
        )
    };
    alt(
        keep_right(string("0x"), parese_num(hex_digit)),
        parese_num(digit),
    )
}

fn stringdot() -> Box<Parser<UMPL2Expr>> {
    inbetween(
        char('.'),
        map(many(not_char('.')), |r| {
            UMPL2Expr::String(r.map_or_else(String::new, Iterator::collect))
        }),
        opt(char('.')),
    )
}

fn stmt() -> Box<Parser<UMPL2Expr>> {
    choice(
        [
            if_stmt(),
            unless_stmt(),
            until_stmt(),
            go_through_stmt(),
            continue_doing_stmt(),
            fn_stmt(),
        ]
        .to_vec(),
    )
}

fn if_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("if"), umpl2expr()),
            keep_right(keep_right(white_space(), string("do")), scope(umpl2expr())),
            keep_right(
                keep_right(white_space(), string("otherwise")),
                scope(umpl2expr()),
            ),
        ]),
        |mut r| {
            let cond = r.next().unwrap_or_default();
            let cons = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            let alt = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::If(Box::new(cond), Box::new(cons), Box::new(alt))
        },
    )
}

// TODO: unless maybe should follow form wher condition not in the beginning
fn unless_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("unless"), umpl2expr()),
            keep_right(
                keep_right(white_space(), string("than")),
                scope(umpl2expr()),
            ),
            keep_right(
                keep_right(white_space(), string("else")),
                scope(umpl2expr()),
            ),
        ]),
        |mut r| {
            let cond = r.next().unwrap_or_default();
            let alt = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            let cons = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::Unless(Box::new(cond), Box::new(alt), Box::new(cons))
        },
    )
}

fn until_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("until"), umpl2expr()),
            keep_right(
                keep_right(white_space(), string("then")),
                scope(umpl2expr()),
            ),
        ]),
        |mut r| {
            let cond = r.next().unwrap_or_default();
            let loop_scope = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::Until(Box::new(cond), Box::new(loop_scope))
        },
    )
}

fn go_through_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("go-through"), keep_right(white_space(), ident_umpl())), // TODO: use identifier parserl, not the full blown expression parser
            keep_right(keep_right(white_space(), string("of")), umpl2expr()),
            scope(umpl2expr()),
        ]),
        |mut r| {
            let for_ident = match r.next().unwrap_or_default() {
                UMPL2Expr::Ident(str) => str,
                // TODO don't panic use try_map, or randomly create an ident string
                _ => panic!()
            };
            let iterable = r.next().unwrap_or_default();
            let loop_scope = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::GoThrough(for_ident, Box::new(iterable), Box::new(loop_scope))
        },
    )
}

fn continue_doing_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        seq(vec![keep_right(
            string("continue-doing"),
            scope(umpl2expr()),
        )]),
        |mut r| {
            let loop_scope = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::ContiueDoing(Box::new(loop_scope))
        },
    )
}

fn fn_stmt() -> Box<dyn CloneFn<UMPL2Expr>> {
    // fanction - through away, name - keep char | umpl2expr
    // optinal param count (base10) - keep -> optinal umpl2expr | usize
    // optinal varidac keep scope > optional char | varidac
    // scope keep umpl2expr

    // (chain (keep right "fanction" name(char)), (chain, (opt number) (chain (opt varidiac), scope))
    map(
        chain(
            keep_right(string("fanction"), keep_right(white_space(),satify(unic_emoji_char::is_emoji))),
                chain(
                    opt(keep_right(white_space(), integer())),
                        chain(
                            opt(keep_right(white_space(),map(any_of(['*', '+']), |char|
                            // its ok to unwrap b/c we already know that it is a correct form
                             Varidiac::from_char(char).unwrap()))),
                            scope(umpl2expr()),
                        
                    ),
                ),
            
        ),
        |r| {
            let name = r.0;
            // TODO: maybe if no count given then randomly choose a count
            let param_count = r.1 .0.unwrap_or_default();
            let variadic = r.1 .1 .0;
            let scope = r.1 .1 .1;
            UMPL2Expr::Fanction(name, param_count, variadic, Box::new(scope))
        },
    )
}

fn ident_umpl() -> Box<dyn CloneFn<UMPL2Expr>> {
    map(
        many1(not_any_of(
            call_start()
                .iter()
                .chain(special_char())
                .chain(call_end())
                .copied(),
        )),
        |res| UMPL2Expr::Ident(res.collect()),
    )
}

fn special_char() -> &'static [char] {
    &[
        '!', ' ', '᚜', '᚛', '.', '&', '|', '?', '*', '+', '@', '\'', '"', ';', '\n', '\t',
    ]
}

fn call_start() -> &'static [char] {
    &[
        '(', '༺', '༼', '⁅', '⁽', '₍', '⌈', '⌊', '❨', '❪', '❬', '❮', '❰', '❲', '❴', '⟅', '⟦', '⟨',
        '⟪', '⟬', '⟮', '⦃', '⦅', '⦇', '⦉', '⦋', '⦍', '⦏', '⦑', '⦓', '⦕', '⦗', '⧘', '⧚', '⸢', '⸤',
        '⸦', '⸨', '\u{2e55}', '\u{2e57}', '\u{2e59}', '\u{2e5b}', '〈', '《', '「', '『', '【',
        '〔', '〖', '〘', '〚', '﹙', '﹛', '﹝', '（', '［', '｛', '｟', '｢',
    ]
}

fn call_end() -> &'static [char] {
    &[
        ')', '༻', '༽', '⁆', '⁾', '₎', '⌉', '⌋', '❩', '❫', '❭', '❯', '❱', '❳', '❵', '⟆', '⟧', '⟩',
        '⟫', '⟭', '⟯', '⦄', '⦆', '⦈', '⦊', '⦌', '⦎', '⦐', '⦒', '⦔', '⦖', '⦘', '⧙', '⧛', '⸣', '⸥',
        '⸧', '⸩', '\u{2e56}', '\u{2e58}', '\u{2e5a}', '\u{2e5c}', '〉', '》', '」', '』', '】',
        '〕', '〗', '〙', '〛', '﹚', '﹜', '﹞', '）', '］', '｝', '｠', '｣',
    ]
}

#[cfg(test)]
mod tests {
    use crate::lexer::{parse_umpl, Boolean, UMPL2Expr, Varidiac};

    #[test]
    pub(crate) fn umpl() {
        println!("{:?}", parse_umpl("if 1 do ᚜1 unless 1 than ᚜1 2᚛ else ᚜1᚛ 2᚛ otherwise ᚜if 1 do ᚜1 2᚛ otherwise ᚜until 1 then ᚜1 2᚛᚛᚛"));
    }

    #[test]
    pub(crate) fn umpl_no_end() {
        println!("{:?}", parse_umpl("if 1 do ᚜1 unless 1 than ᚜1 2 else ᚜1 2 otherwise ᚜if 1 do ᚜1 2 otherwise ᚜until 1 then ᚜1 2"));
    }

    #[test]
    pub(crate) fn umpl_if() {
        let test_result = parse_umpl("if ? do ᚜2 6 6᚛  otherwise ᚜4᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Unless(
                Box::new(UMPL2Expr::Bool(Boolean::Maybee)),
                Box::new(UMPL2Expr::Scope(vec![
                    UMPL2Expr::Number(2.0),
                    UMPL2Expr::Number(6.0),
                    UMPL2Expr::Number(6.0)
                ])),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Number(4.0)]))
            )
        )
    }

    #[test]
    fn umpl_unless() {
        let test_result = parse_umpl("unless & than ᚜4᚛ else ᚜.t.᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Unless(
                Box::new(UMPL2Expr::Bool(Boolean::True)),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Number(4.0)])),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::String("t".to_string())]))
            )
        )
    }

    #[test]
    fn umpl_until() {
        let test_result = parse_umpl("until | then ᚜ ab/᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Until(
                Box::new(UMPL2Expr::Bool(Boolean::False)),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident("ab/".to_string())]))
            )
        )
    }


    #[test]
    fn umpl_go_through() {
        let test_result = parse_umpl("go-through a of (tree 5 6 7) ᚜ .ab/.᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::GoThrough(
                "a".to_string(),
                Box::new(UMPL2Expr::Application(vec![UMPL2Expr::Ident("tree".to_string()), UMPL2Expr::Number(5.0), UMPL2Expr::Number(6.0), UMPL2Expr::Number(7.0)])),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::String("ab/".to_string())]))
            )
        )
    }

    #[test]
    fn umpl_continue_doing() {
        let test_result = parse_umpl("continue-doing ᚜ lg` ᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::ContiueDoing(
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident("lg`".to_string())]))
            )
        )
    }

    #[test]
    fn umpl_fn() {
        let test_result = parse_umpl("fanction 🚗  1 * ᚜ ^l ᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Fanction(
                '🚗',
                1,
                Some(Varidiac::AtLeast0),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident("^l".to_string())]))
            )
        )
    }
}