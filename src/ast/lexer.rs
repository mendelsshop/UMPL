#![allow(dead_code)]

use std::iter;

use parse_int::parse;

use crate::{
    ast::{Application, Boolean, PrintType, UMPL2Expr},
    pc::{
        alt, any_of, chain, char, choice, inbetween, integer, keep_left, keep_right, many, many1,
        map, not_any_of, not_char, opt, satify, seq, string, try_map, ParseError, ParseErrorType,
        Parser,
    },
};

fn ws_or_comment() -> Box<Parser<Option<Box<dyn Iterator<Item = char>>>>> {
    map(
        many(alt(
            keep_right(char('!'), keep_left(many(not_char('\n')), opt(char('\n')))),
            map(any_of([' ', '\n', '\t']), |i| Some(opaquify(iter::once(i)))),
        )),
        |r| -> Option<Box<dyn Iterator<Item = char>>> {
            r.map(|r| opaquify(r.flatten().flatten()))
        },
    )
}

fn opaquify(f: impl Iterator<Item = char> + 'static) -> Box<dyn Iterator<Item = char>> {
    Box::new(f)
}

fn scope(p: Box<Parser<UMPL2Expr>>) -> Box<Parser<UMPL2Expr>> {
    inbetween(
        keep_right(ws_or_comment(), char('᚜')),
        map(many(p), |r| {
            UMPL2Expr::Scope(r.map_or_else(Vec::new, Iterator::collect))
        }),
        opt(keep_right(ws_or_comment(), char('᚛'))),
    )
}

fn umpl2expr() -> Box<Parser<UMPL2Expr>> {
    // needs to be its own new closure so that we don't have infinite recursion while creating the parser (so we add a level of indirection)
    Box::new(|input| {
        keep_right(
            ws_or_comment(),
            choice(
                [
                    literal(),
                    stmt(),
                    terminal_umpl(),
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
                keep_right(ws_or_comment(), any_of(call_start().iter().copied())),
                many(umpl2expr()),
                opt(keep_right(
                    ws_or_comment(),
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
        |r| {
            UMPL2Expr::Application(Application::new(
                r.0.map_or_else(Vec::new, Iterator::collect),
                r.1,
            ))
        },
    )
}

pub fn parse_umpl(input: &str) -> Result<UMPL2Expr, ParseError<'_>> {
    umpl2expr()(input).map(|res| res.0)
}

pub fn umpl_parse(input: &str) -> Result<Vec<UMPL2Expr>, ParseError<'_>> {
    map(many(umpl2expr()), |r| r.map_or(vec![], Iterator::collect))(input).map(|res| res.0)
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
            let_stmt(),
        ]
        .to_vec(),
    )
}

fn let_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("let"),
            keep_right(ws_or_comment(), ident_umpl()),
            umpl2expr(),
        ]),
        |r| UMPL2Expr::Application(Application::new(r.take(3).collect(), PrintType::None)),
    )
}

fn if_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("if"),
            umpl2expr(),
            keep_right(ws_or_comment(), umpl_ident_string("do")),
            scope(umpl2expr()),
            keep_right(ws_or_comment(), umpl_ident_string("otherwise")),
            scope(umpl2expr()),
        ]),
        |r| {
            // let if_ident = UMPL2Expr::Ident("if".into());
            // let cond = r.next().unwrap_or_default();
            // let do_ident = UMPL2Expr::Ident("do".into());
            // let cons = r.next().unwrap_or_default();
            // let otherwise_ident = UMPL2Expr::Ident("otherwise".into());
            // let alt = r.next().unwrap_or_default();

            UMPL2Expr::Application(Application::new(r.take(6).collect(), PrintType::None))
        },
    )
}

fn umpl_ident_string(s: &str) -> Box<Parser<UMPL2Expr>> {
    map(string(s), |str| UMPL2Expr::Ident(str.into()))
}

// fn get_scope(i: Option<UMPL2Expr>) -> Vec<UMPL2Expr> {
//     i.and_then(UMPL2Expr::get_scope_owned).unwrap_or_default()
// }

// TODO: unless maybe should follow form wher condition not in the beginning
fn unless_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("unless"),
            umpl2expr(),
            keep_right(ws_or_comment(), umpl_ident_string("than")),
            scope(umpl2expr()),
            keep_right(ws_or_comment(), umpl_ident_string("else")),
            scope(umpl2expr()),
        ]),
        |r| UMPL2Expr::Application(Application::new(r.take(6).collect(), PrintType::None)),
    )
}

fn until_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("until"),
            umpl2expr(),
            keep_right(ws_or_comment(), umpl_ident_string("then")),
            scope(umpl2expr()),
        ]),
        |r| UMPL2Expr::Application(Application::new(r.take(4).collect(), PrintType::None)),
    )
}

fn go_through_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("go-through"),
            keep_right(ws_or_comment(), ident_umpl()),
            // TODO: use identifier parserl, not the full blown expression parser
            keep_right(ws_or_comment(), umpl_ident_string("of")),
            umpl2expr(),
            scope(umpl2expr()),
        ]),
        |r| UMPL2Expr::Application(Application::new(r.take(4).collect(), PrintType::None)),
    )
}

fn continue_doing_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            umpl_ident_string("continue-doing"),
            scope(umpl2expr()),
        ]),
        |r| UMPL2Expr::Application(Application::new(r.take(2).collect(), PrintType::None)),
    )
}

fn link_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(string("link"), keep_right(ws_or_comment(), label_umpl())),
            many1(keep_right(ws_or_comment(), label_umpl())),
        ),
        |res| {
            let UMPL2Expr::Label(to_link) = res.0 else { panic!() };
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
        // chain(
        //     keep_right(
        //         string("fanction"),
        // opt(keep_right(
        //     ws_or_comment(),
        //     satify(unic_emoji_char::is_emoji_presentation),
        // )),
        //     ),
        //     chain(
        //         opt(keep_right(ws_or_comment(), integer())),
        // chain(
        //     opt(keep_right(
        //         ws_or_comment(),
        //         map(any_of(['*', '+']), |char|
        //             // its ok to unwrap b/c we already know that it is a correct form
        //              Varidiac::from_char(char).unwrap()),
        //     )),
        //     scope(umpl2expr()),
        // ),
        //     ),
        // ),
        chain(
            chain(
                umpl_ident_string("fanction"),
                opt(keep_right(
                    ws_or_comment(),
                    map(satify(unic_emoji_char::is_emoji_presentation), |f| {
                        UMPL2Expr::Ident(f.to_string().into())
                    }),
                )),
            ),
            chain(
                opt(keep_right(
                    ws_or_comment(),
                    map(integer(), |i| UMPL2Expr::Number(i as f64)),
                )),
                chain(
                    opt(keep_right(
                        ws_or_comment(),
                        alt(umpl_ident_string("*"), umpl_ident_string("+")),
                    )),
                    scope(umpl2expr()),
                ),
            ),
        ),
        |r| {
            let mut fn_type = vec![r.0 .0];
            if let Some(name) = r.0 .1 {
                fn_type.push(name);
            }
            // TODO: maybe if no count given then randomly choose a count
            if let Some(arg_count) = r.1 .0 {
                fn_type.push(arg_count);
            }
            if let Some(varidiac) = r.1 .1 .0 {
                fn_type.push(varidiac);
            }
            fn_type.push(r.1 .1 .1);
            UMPL2Expr::Application(Application::new(fn_type, PrintType::None))
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

const fn special_char() -> &'static [char] {
    &[
        '!', ' ', '᚜', '᚛', '.', '&', '|', '?', '*', '+', '@', '\'', '"', ';', '\n', '\t', '<', '>',
    ]
}

const fn call_start() -> &'static [char] {
    &[
        '(', '༺', '༼', '⁅', '⁽', '₍', '⌈', '⌊', '❨', '❪', '❬', '❮', '❰', '❲', '❴', '⟅', '⟦', '⟨',
        '⟪', '⟬', '⟮', '⦃', '⦅', '⦇', '⦉', '⦋', '⦍', '⦏', '⦑', '⦓', '⦕', '⦗', '⧘', '⧚', '⸢', '⸤',
        '⸦', '⸨', '\u{2e55}', '\u{2e57}', '\u{2e59}', '\u{2e5b}', '〈', '《', '「', '『', '【',
        '〔', '〖', '〘', '〚', '﹙', '﹛', '﹝', '（', '［', '｛', '｟', '｢', '{', '[',
    ]
}

const fn call_end() -> &'static [char] {
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
    map(keep_right(char(';'), umpl2expr()), |quoted| {
        let quoted_ident = UMPL2Expr::Ident("quoted".into());
        UMPL2Expr::Application(Application::new(
            vec![quoted_ident, quoted],
            PrintType::None,
        ))
    })
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

macro_rules! get_expr {
    ($type:ident, $ret:ty, $method_name:ident) => {
        impl UMPL2Expr {
            #[must_use]
            pub const fn $method_name(&self) -> Option<&$ret> {
                match self {
                    Self::$type(t) => Some(t),
                    _ => None,
                }
            }
        }
    };
}

get_expr! {Scope, Vec<UMPL2Expr>, get_scope}

macro_rules! get_expr_owned {
    ($type:ident, $ret:ty, $method_name:ident) => {
        impl UMPL2Expr {
            #[allow(clippy::missing_const_for_fn)] // taking self doesnt work well with const fn (something about destructors)
            #[must_use]
            pub fn $method_name(self) -> Option<$ret> {
                match self {
                    Self::$type(t) => Some(t),
                    _ => None,
                }
            }
        }
    };
}

get_expr_owned! {Scope, Vec<UMPL2Expr>, get_scope_owned}
