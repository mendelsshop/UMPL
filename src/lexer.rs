#![allow(dead_code)]


use parse_int::parse;

use crate::{pc::{
    alt, any_of, chain, char, choice, inbetween, integer, keep_right, many, many1, map, not_any_of,
    not_char, opt, satify, seq, string, try_map, white_space, ParseError, ParseErrorType, Parser,
}, interior_mut::RC};


#[derive(Debug, Default, PartialEq)]
pub enum UMPL2Expr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Scope(Vec<UMPL2Expr>),
    Ident(RC<str>),
    If(Box<UMPL2Expr>, Box<UMPL2Expr>, Box<UMPL2Expr>),
    Unless(Box<UMPL2Expr>, Box<UMPL2Expr>, Box<UMPL2Expr>),
    Stop(Box<UMPL2Expr>),
    Skip,
    Until(Box<UMPL2Expr>, Box<UMPL2Expr>),
    GoThrough(RC<str>, Box<UMPL2Expr>, Box<UMPL2Expr>),
    ContiueDoing(Box<UMPL2Expr>),
    Fanction(char, usize, Option<Varidiac>, Box<UMPL2Expr>),
    Application(Vec<UMPL2Expr>, PrintType),
    Quoted(Box<UMPL2Expr>),
    Label(RC<str>),
    FnParam(usize),
    #[default]
    Hempty,
    Link(RC<str>, Vec<RC<str>>),
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

#[derive(Debug, PartialEq)]
pub enum PrintType {
    None,
    Print,
    PrintLN,
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
    // needs to be its own new closure so that we don't have infinite recursion while creating the parser (so we add a level of indirection)
    Box::new(|input| {
        keep_right(
            white_space(),
            choice(
                [
                    literal(),
                    stmt(),
                    ident_umpl(),
                    application(),
                    special_start(),
                ]
                .to_vec(),
            ),
        )(input)
    })
}

fn application() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            inbetween(
                keep_right(white_space(), any_of(call_start().iter().copied())),
                many(umpl2expr()),
                opt(keep_right(
                    white_space(),
                    any_of(call_end().iter().copied()),
                )),
            ),
            alt(
                map(char('<'), |_| PrintType::None),
                map(chain(char('>'), opt(char('>'))), |r| match r.1 {
                    None => PrintType::PrintLN,
                    Some(_) => PrintType::Print,
                }),
            ),
        ),
        |r| UMPL2Expr::Application(r.0.map_or_else(Vec::new, Iterator::collect), r.1),
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
                            kind: ParseErrorType::Other("not a digit".into()),
                            input: "",
                        })
                    }
                    (None, Some(s)) => (String::new(), s.collect()),
                    (Some(s), None) => (s.collect(), String::new()),
                    (Some(s), Some(r)) => (s.collect(), r.collect()),
                };
                println!("{}.{}", number.0, number.1);
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
            UMPL2Expr::String(r.map_or_else(String::new, Iterator::collect).into())
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
            link_stmt(),
        ]
        .to_vec(),
    )
}

fn if_stmt() -> Box<Parser<UMPL2Expr>> {
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
fn unless_stmt() -> Box<Parser<UMPL2Expr>> {
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

fn until_stmt() -> Box<Parser<UMPL2Expr>> {
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

fn go_through_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(
                string("go-through"),
                keep_right(white_space(), ident_umpl()),
            ), // TODO: use identifier parserl, not the full blown expression parser
            keep_right(keep_right(white_space(), string("of")), umpl2expr()),
            scope(umpl2expr()),
        ]),
        |mut r| {
            let for_ident = match r.next().unwrap_or_default() {
                UMPL2Expr::Ident(str) => str,
                // TODO don't panic use try_map, or randomly create an ident string
                _ => panic!(),
            };
            let iterable = r.next().unwrap_or_default();
            let loop_scope = r.next().unwrap_or_else(|| UMPL2Expr::Scope(vec![]));
            UMPL2Expr::GoThrough(for_ident, Box::new(iterable), Box::new(loop_scope))
        },
    )
}

fn continue_doing_stmt() -> Box<Parser<UMPL2Expr>> {
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

fn link_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(string("link"), keep_right(white_space(), label_umpl())),
            many1(keep_right(white_space(), label_umpl())),
        ),
        |res| {
            let to_link = match res.0 {
                UMPL2Expr::Label(l) => l,
                _ => panic!(),
            };
            let linked_list = res
                .1
                .map(|e| match e {
                    UMPL2Expr::Label(l) => l,
                    _ => panic!(),
                })
                .collect();
            UMPL2Expr::Link(to_link, linked_list)
        },
    )
}

fn fn_stmt() -> Box<Parser<UMPL2Expr>> {
    // fanction - through away, name - keep char | umpl2expr
    // optinal param count (base10) - keep -> optinal umpl2expr | usize
    // optinal varidac keep scope > optional char | varidac
    // scope keep umpl2expr

    // (chain (keep right "fanction" name(char)), (chain, (opt number) (chain (opt varidiac), scope))
    map(
        chain(
            keep_right(
                string("fanction"),
                keep_right(white_space(), satify(unic_emoji_char::is_emoji)),
            ),
            chain(
                opt(keep_right(white_space(), integer())),
                chain(
                    opt(keep_right(
                        white_space(),
                        map(any_of(['*', '+']), |char|
                            // its ok to unwrap b/c we already know that it is a correct form
                             Varidiac::from_char(char).unwrap()),
                    )),
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

fn ident_umpl() -> Box<Parser<UMPL2Expr>> {
    map(
        many1(not_any_of(
            call_start()
                .iter()
                .chain(special_char())
                .chain(call_end())
                .copied(),
        )),
        |res| UMPL2Expr::Ident(res.collect::<String>().into()),
    )
}

fn special_char() -> &'static [char] {
    &[
        '!', ' ', '᚜', '᚛', '.', '&', '|', '?', '*', '+', '@', '\'', '"', ';', '\n', '\t', '<', '>',
    ]
}

fn call_start() -> &'static [char] {
    &[
        '(', '༺', '༼', '⁅', '⁽', '₍', '⌈', '⌊', '❨', '❪', '❬', '❮', '❰', '❲', '❴', '⟅', '⟦', '⟨',
        '⟪', '⟬', '⟮', '⦃', '⦅', '⦇', '⦉', '⦋', '⦍', '⦏', '⦑', '⦓', '⦕', '⦗', '⧘', '⧚', '⸢', '⸤',
        '⸦', '⸨', '\u{2e55}', '\u{2e57}', '\u{2e59}', '\u{2e5b}', '〈', '《', '「', '『', '【',
        '〔', '〖', '〘', '〚', '﹙', '﹛', '﹝', '（', '［', '｛', '｟', '｢', '{', '[',
    ]
}

fn call_end() -> &'static [char] {
    &[
        ')', '༻', '༽', '⁆', '⁾', '₎', '⌉', '⌋', '❩', '❫', '❭', '❯', '❱', '❳', '❵', '⟆', '⟧', '⟩',
        '⟫', '⟭', '⟯', '⦄', '⦆', '⦈', '⦊', '⦌', '⦎', '⦐', '⦒', '⦔', '⦖', '⦘', '⧙', '⧛', '⸣', '⸥',
        '⸧', '⸩', '\u{2e56}', '\u{2e58}', '\u{2e5a}', '\u{2e5c}', '〉', '》', '」', '』', '】',
        '〕', '〗', '〙', '〛', '﹚', '﹜', '﹞', '）', '］', '｝', '｠', '｣', '}', ']',
    ]
}

fn terminal_umpl() -> Box<Parser<UMPL2Expr>> {
    alt(
        map(string("skip"), |_| UMPL2Expr::Skip),
        map(
            keep_right(string("stop"), map(umpl2expr(), Box::new)),
            UMPL2Expr::Stop,
        ),
    )
}

fn special_start() -> Box<Parser<UMPL2Expr>> {
    choice(vec![quoted_umpl(), label_umpl(), param_umpl()])
}

fn quoted_umpl() -> Box<Parser<UMPL2Expr>> {
    map(
        map(keep_right(char(';'), umpl2expr()), Box::new),
        UMPL2Expr::Quoted,
    )
}

fn label_umpl() -> Box<Parser<UMPL2Expr>> {
    map(keep_right(char('@'), ident_umpl()), |res| {
        UMPL2Expr::Label(match res {
            UMPL2Expr::Ident(i) => i,
            _ => panic!(),
        })
    })
}

fn param_umpl() -> Box<Parser<UMPL2Expr>> {
    inbetween(
        any_of(['\'', '"']),
        map(
            many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7'])),
            |res| UMPL2Expr::FnParam(parse(&format!("0o{}", res.collect::<String>())).unwrap()),
        ),
        opt(any_of(['\'', '"'])),
    )
}

#[cfg(test)]
mod tests {
    use crate::lexer::{parse_umpl, Boolean, PrintType, UMPL2Expr, Varidiac};

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
            UMPL2Expr::If(
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
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::String("t".into())]))
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
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident("ab/".into())]))
            )
        )
    }

    #[test]
    fn umpl_go_through() {
        let test_result = parse_umpl("go-through a of (tree 5 6 7)< ᚜ .ab/.᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::GoThrough(
                "a".into(),
                Box::new(UMPL2Expr::Application(
                    vec![
                        UMPL2Expr::Ident("tree".into()),
                        UMPL2Expr::Number(5.0),
                        UMPL2Expr::Number(6.0),
                        UMPL2Expr::Number(7.0)
                    ],
                    PrintType::None,
                )),
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::String("ab/".into())]))
            )
        )
    }

    #[test]
    fn umpl_continue_doing() {
        let test_result = parse_umpl("continue-doing ᚜ lg` ᚛");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::ContiueDoing(Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident(
                "lg`".into()
            )])))
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
                Box::new(UMPL2Expr::Scope(vec![UMPL2Expr::Ident("^l".into())]))
            )
        )
    }

    #[test]
    fn umpl_ident() {
        let test_result = parse_umpl("a===a");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Ident("a===a".into())
        )
    }

    #[test]
    fn umpl_number() {
        let test_result = parse_umpl("0xf%9");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::Number(15.5625))
    }

    #[test]
    fn umpl_bool() {
        let test_result = parse_umpl("?");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::Bool(Boolean::Maybee))
    }

    #[test]
    fn umpl_application() {
        let test_result = parse_umpl("{mul 5 0x10 ]>>");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Application(
                vec![
                    UMPL2Expr::Ident("mul".into()),
                    UMPL2Expr::Number(5.0),
                    UMPL2Expr::Number(16.0)
                ],
                PrintType::Print
            )
        )
    }

    #[test]
    fn umpl_acces_param() {
        let test_result = parse_umpl("'10'");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::FnParam(8))
    }
}
