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

pub enum Varidiac {
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1,
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0,
}

pub enum Boolean {
    /// &
    True,
    /// |
    False,
    /// ?
    Maybee,
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
}

#[derive(Debug)]
pub struct ParseError<'a> {
    kind: ParseErrorType,
    input: &'a str,
}

/// a parser for things (T) is a function of Strings
/// to a list of pairs of things (T) and strings
// type Parser<T> = dyn Fn(&str) -> Result<(T, &str), ParseError> + 'static;

pub fn digit() -> Box<Parser<usize>> {
    Box::new(|input: &str| {
        println!("{}`{input:?}` -> digit", indent());
        match input.chars().next() {
            Some(n) => match n.to_digit(10) {
                Some(d) => Ok((d as usize, input.split_at(1).1)),
                None => Err(ParseError {kind: ParseErrorType::NotADigit(n), input: input}),
            },
            None => Err(ParseError {kind: ParseErrorType::EOF, input: ""}),
        }
    })
}

pub fn char(looking_for: char) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        println!("{}`{input:?}` -> `{looking_for:?}`", indent());
        match input.chars().next() {
            Some(n) => {
                if n == looking_for {
                    Ok((n, input.split_at(1).1))
                } else {
                    Err(ParseError {kind: ParseErrorType::Mismatch(looking_for, n), input: input})
                }
            }
            None => Err(ParseError {kind: ParseErrorType::EOF, input: ""}),
        }
    })
}

pub fn not_char(looking_for: char) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        println!("{}`{input:?}` -> !`{looking_for:?}`", indent());
        match input.chars().next() {
            Some(n) => {
                if n != looking_for {
                    Ok((n, input.split_at(1).1))
                } else {
                    Err(ParseError {kind: ParseErrorType::Mismatch(looking_for, n), input: input})
                }
            }
            None => Err(ParseError {kind: ParseErrorType::EOF, input: ""}),
        }
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

pub fn alt<T: 'static>(parser1: Box<Parser<T>>, parser2: Box<Parser<T>>) -> Box<Parser<T>> {
    Box::new(move |input| {
        println!("alt s `{input}`");
        match parser1(input) {
            Ok((res, input)) => {
                println!("alt m `{input}`");
                Ok((res, input))
            },
            Err(_) => {
                println!("alt e `{input}`");
                parser2(input)
            },
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
        (None, input) => Err(ParseError {kind: ParseErrorType::NotEnoughMatches, input: input}),
        (Some(v), input) => Ok((v, input)),
    })
}

pub fn fail<T>() -> Box<Parser<T>> {
    Box::new(move |input| Err(ParseError {kind: ParseErrorType::Fail, input: input}))
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
                Err(_) => continue
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
        res.ok_or(ParseError{kind: ParseErrorType::NoMatchFound, input})
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
        (None, input) => Err(ParseError {kind: ParseErrorType::NotEnoughMatches, input: input}),
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
    If(Box<LispExpr>, Box<LispExpr>, Box<LispExpr>)
}

static mut INDENT: usize = 0;
pub fn inc_indent() {
    unsafe { INDENT+=1 }
} 
pub fn dec_indent() {
    unsafe { INDENT-=1 }
} 
pub fn indent() -> String {
    format!("{}{}",unsafe { INDENT }," ".repeat(unsafe {
        INDENT * 4
    }))
}

pub fn lispnumber() -> Box<Parser<LispExpr>> {
    Box::new(|input|  {
    println!("{}number", indent());
    map(many1(digit()), |i| {
        let collect = &i.map(|i| i.to_string()).collect::<String>();
        LispExpr::Number(collect.parse().unwrap())
    })(input)})
}

pub fn lisplist() -> Box<Parser<LispExpr>> {
    Box::new(|input|  {
    println!("{}list", indent());
    inc_indent();
    let res = 
        inbetween(
            keep_left(char('('), white_space()),
            map(sep(lispexpr(), white_space()),
            |i| LispExpr::List(i.map_or(vec![], Iterator::collect))),
            keep_right(white_space(), char(')')),
    
    )(input);
    dec_indent();
    res
})
}

pub fn lispif()  -> Box<Parser<LispExpr>> {
    map(seq(vec![keep_right(string("if"), lispexpr()),keep_right(chain(white_space(), string("then")), lispexpr()), keep_right(chain(white_space(), string("else")), lispexpr())]), 
    |mut i|
    {
        LispExpr::If(Box::new(i.next().unwrap()), Box::new(i.next().unwrap()), Box::new(i.next().unwrap()))
    }

)
}

pub fn lispsymbol() -> Box<Parser<LispExpr>> {
    Box::new(|input|  {
    println!("{}sybmol", indent());
    map(many1(not_any_of(['\n', ' ', '\t', '.', '(',')'])), |i|LispExpr::Symbol(i.collect()))(input)
})
}

pub fn lispexpr() -> Box<Parser<LispExpr>> {
    Box::new(|input|  {
        println!("{}listexpr", indent());
        inc_indent();
        let res = keep_right(white_space(),choice([
            lispnumber(),
            lisplist(), 
            lispif(),
            lispsymbol(),
            ].to_vec()))(input);
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