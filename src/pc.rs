use std::{collections::HashSet, iter::empty};

use std::iter;

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
    pub kind: ParseErrorType,
    pub input: &'a str,
}

#[must_use]
pub fn digit() -> Box<Parser<usize>> {
    // Box::new(|input: &str| {
    //     // println!("`{input:?}` -> digit");
    //     match input.chars().next() {
    //         Some(n) => match n.to_digit(10) {
    //             Some(d) => Ok((d as usize, input.split_at(1).1)),
    //             None => Err(ParseError {
    //                 kind: ParseErrorType::NotADigit(n),
    //                 input,
    //             }),
    //         },
    //         None => Err(ParseError {
    //             kind: ParseErrorType::EOF,
    //             input: "",
    //         }),
    //     }
    // })
    map(satify(|c| c.is_ascii_digit()), |d| d as usize)
}

#[must_use]
pub fn char(looking_for: char) -> Box<Parser<char>> {
    // Box::new(move |input: &str| {
    //     // println!("`{input:?}` -> `{looking_for:?}`");
    //     match input.chars().next() {
    //         Some(n) => {
    //             if n == looking_for {
    //                 Ok((n, input.split_at(n.len_utf8()).1))
    //             } else {
    //                 Err(ParseError {
    //                     kind: ParseErrorType::Mismatch(looking_for, n),
    //                     input,
    //                 })
    //             }
    //         }
    //         None => Err(ParseError {
    //             kind: ParseErrorType::EOF,
    //             input: "",
    //         }),
    //     }
    // })
    satify(move |c| c == looking_for)
}

#[must_use]
pub fn not_char(looking_for: char) -> Box<Parser<char>> {
    // Box::new(move |input: &str| {
    //     // println!("`{input:?}` -> !`{looking_for:?}`");
    //     input.chars().next().map_or(Err(ParseError {
    //         kind: ParseErrorType::EOF,
    //         input: "",
    //     }), |n| if n != looking_for {
    //             Ok((n, input.split_at(n.len_utf8()).1))
    //         } else {
    //             Err(ParseError {
    //                 kind: ParseErrorType::Mismatch(looking_for, n),
    //                 input,
    //             })
    //         })

    // })
    satify(move |c| c != looking_for)
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn integer() -> Box<Parser<usize>> {
    map(
        many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])),
        |input| input.collect::<String>().parse().unwrap(),
    )
}

pub fn satify(checker: impl Fn(char) -> bool + 'static + Clone) -> Box<Parser<char>> {
    Box::new(move |input: &str| {
        // println!("{}`{input:?}` -> `{looking_for:?}`", indent());
        input.chars().next().map_or(
            Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
            }),
            |n| {
                if checker(n) {
                    // println!("found match {n}");
                    Ok((n, input.split_at(n.len_utf8()).1))
                } else {
                    // println!("not match {n}");
                    Err(ParseError {
                        kind: ParseErrorType::SatisfyMismatch(n),
                        input,
                    })
                }
            },
        )
    })
}

#[must_use]
pub fn take() -> Box<Parser<char>> {
    satify(|_| true)
}

#[must_use]
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

#[must_use]
pub fn alt<T: 'static>(parser1: Box<Parser<T>>, parser2: Box<Parser<T>>) -> Box<Parser<T>> {
    Box::new(move |input| {
        // println!("alt s `{input}`");
        match parser1(input) {
            Ok((res, input)) => {
                // println!("alt m `{input}`");
                Ok((res, input))
            }
            Err(_) => {
                // println!("alt e `{input}`");
                parser2(input)
            }
        }
    })
}

#[must_use]
pub fn opt<T: 'static>(parser: Box<Parser<T>>) -> Box<Parser<Option<T>>> {
    Box::new(move |input| match parser(input) {
        Ok(ok) => Ok((Some(ok.0), ok.1)),
        // TODO: error should return leftover substring
        Err(e) => Ok((None, e.input)),
    })
}

#[must_use]
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

#[must_use]
pub fn keep_left<T: 'static, U: 'static>(
    left_parser: Box<Parser<T>>,
    right_parser: Box<Parser<U>>,
) -> Box<Parser<T>> {
    map(chain(left_parser, right_parser), |i| i.0)
}

#[must_use]
pub fn keep_right<T: 'static, U: 'static>(
    left_parser: Box<Parser<T>>,
    right_parser: Box<Parser<U>>,
) -> Box<Parser<U>> {
    map(chain(left_parser, right_parser), |i| i.1)
}

#[must_use]
pub fn inbetween<T: 'static, U: 'static, V: 'static>(
    left_parser: Box<Parser<T>>,
    middle_parser: Box<Parser<U>>,
    right_parser: Box<Parser<V>>,
) -> Box<Parser<U>> {
    keep_left(keep_right(left_parser, middle_parser), right_parser)
}

#[must_use]
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

#[must_use]
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

#[must_use]
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

#[must_use]
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
        for parser in parsers.clone() {
            // println!("choice s `{input}`");
            match parser(input) {
                Ok(ok) => return Ok(ok),
                Err(_) => continue,
            }
        }
        fail()(input)
    })
}

#[must_use]
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
        for parser in parsers.clone() {
            // println!("choice s `{input}`");
            res = Some(parser(input)?);
        }
        res.ok_or(ParseError {
            kind: ParseErrorType::NoMatchFound,
            input,
        })
    })
}

pub fn any_of(chars: impl IntoIterator<Item = char>) -> Box<Parser<char>> {
    let p: HashSet<char> = chars.into_iter().collect();
    satify(move |c| p.contains(&c))
}

pub fn not_any_of(chars: impl IntoIterator<Item = char>) -> Box<Parser<char>> {
    let p: HashSet<char> = chars.into_iter().collect();
    satify(move |c| !p.contains(&c))
    // not_choice(chars.into_iter().map(not_char).collect())
}

#[must_use]
pub fn string(to_match: &str) -> Box<Parser<String>> {
    map(seq(to_match.chars().map(|c| char(c)).collect()), |chars| {
        chars.collect::<String>()
    })
}

#[must_use]
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

#[must_use]
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

#[must_use]
pub fn white_space() -> Box<Parser<Option<Box<dyn Iterator<Item = char>>>>> {
    many(any_of([' ', '\n', '\t']))
}

pub trait CloneFn<T>: Fn(&str) -> Result<(T, &str), ParseError<'_>> {
    fn clone_box<'a>(&self) -> Box<dyn CloneFn<T> + 'a>
    where
        Self: 'a;
}

impl<T, F> CloneFn<T> for F
where
    F: Fn(&str) -> Result<(T, &str), ParseError<'_>> + Clone,
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
