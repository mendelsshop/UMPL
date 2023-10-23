#![allow(dead_code)]

use itertools::chain as vec_splat;
use parse_int::parse;
use std::iter;
// chars on us keyboard not used: `, , \,/,,,=
// qussiquote -> :
// unquote -> $
use crate::{
    ast::{Boolean, UMPL2Expr},
    interior_mut::RC,
    pc::{
        alt, any_of, chain, char, choice, inbetween, keep_left, keep_right, many, many1, map,
        not_any_of, not_char, opt, satify, seq, string, try_map, ParseError, ParseErrorType,
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
fn scope_list(p: Box<Parser<UMPL2Expr>>) -> Box<Parser<Vec<UMPL2Expr>>> {
    inbetween(
        keep_right(ws_or_comment(), char('᚜')),
        map(many(p), |r| r.map_or_else(Vec::new, Iterator::collect)),
        opt(keep_right(ws_or_comment(), char('᚛'))),
    )
}
fn scope(p: Box<Parser<UMPL2Expr>>) -> Box<Parser<UMPL2Expr>> {
    map(scope_list(p), |mut scope| {
        scope.insert(0, "begin".into());
        UMPL2Expr::Application(scope)
    })
}
fn umpl2expr() -> Box<Parser<UMPL2Expr>> {
    // needs to be its own new closure so that we don't have infinite recursion while creating the parser (so we add a level of indirection)
    map(
        chain(
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
                            scope(umpl2expr()),
                        ]
                        .to_vec(),
                    ),
                )(input)
            }),
            many(choice(vec![
                // match >> before > so >> doesn't become >, >
                string(">>"),
                string(">"),
                string("<"),
                keep_right(
                    char('^'),
                    choice(vec![string("car"), string("cdr"), string("cgr")]),
                ),
            ])),
        ),
        |mut r| {
            if let Some(accesors) = r.1 {
                let new_acces =
                    |accesor: String, expr| UMPL2Expr::Application(vec![accesor.into(), expr]);
                for mut accesor in accesors {
                    if accesor == ">>" {
                        accesor.clear();
                        accesor += "print";
                    } else if accesor == ">" {
                        // TODO: make printline function just calls print + newline
                        accesor.clear();
                        accesor += "println";
                    }
                    // if it says to not print we just ignore it
                    if accesor == "<" {
                        continue;
                    }

                    r.0 = new_acces(accesor, r.0);
                }
            }
            r.0
        },
    )
}

fn application() -> Box<Parser<UMPL2Expr>> {
    map(
        inbetween(
            keep_right(ws_or_comment(), any_of(call_start().iter().copied())),
            many(umpl2expr()),
            opt(keep_right(
                ws_or_comment(),
                any_of(call_end().iter().copied()),
            )),
        ),
        |r| UMPL2Expr::Application(r.map_or_else(Vec::new, Iterator::collect)),
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
        map(many(alt(escape_string(), not_char('.'))), |r| {
            UMPL2Expr::String(r.map_or_else(String::new, Iterator::collect).into())
        }),
        opt(char('.')),
    )
}
/// | Escape sequence | Description |
/// |:-:|:-:|
/// | `\n` | newline |
/// | `\t` | tab |
/// | `\r` | carriage return |
/// | `\b` | backspace |
/// | `\f` | form feed |
/// | `\a` | alert |
/// | `\v` | vertical tab |
/// | `\e` | escape |
/// | `\\` | backslash |
/// | ```\` ``` | single quote |
/// | ```\x{hex}``` | hexadecimal value in ascii representation |
/// | `\u{hex}` | Unicode character |\
fn escape_string() -> Box<Parser<char>> {
    keep_right(
        // TODO: this more escape characters
        char('\\'),
        choice(vec![
            map(char('a'), |_| '\x07'),
            map(char('n'), |_| '\n'),
            char('.'),
            char('\\'),
        ]),
    )
}

fn stmt() -> Box<Parser<UMPL2Expr>> {
    choice(
        [
            mod_stmt(),
            if_stmt(),
            until_stmt(),
            go_through_stmt(),
            continue_doing_stmt(),
            fn_stmt(),
            link_stmt(),
            let_stmt(),
            class_stmt(),
        ]
        .to_vec(),
    )
}

fn mod_stmt() -> Box<Parser<UMPL2Expr>> {
    keep_right(
        string("mod"),
        keep_right(
            ws_or_comment(),
            map(
                chain(
                    satify(|c| c.is_ascii_alphabetic()),
                    keep_right(
                        ws_or_comment(),
                        alt(scope_list(umpl2expr()), map(stringdot(), |s| vec![s])),
                    ),
                ),
                |(name, code)| {
                    let mut module = vec!["module".into(), name.to_string().into()];
                    module.extend(code);
                    UMPL2Expr::Application(module)
                },
            ),
        ),
    )
}

fn let_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        keep_right(
            string("let"),
            chain(
                keep_right(ws_or_comment(), ident_umpl()),
                keep_right(keep_right(ws_or_comment(), char('=')), umpl2expr()),
            ),
        ),
        |r| UMPL2Expr::Application(vec!["define".into(), r.0, r.1]),
    )
}

fn method_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(
                keep_right(
                    ws_or_comment(),
                    keep_right(
                        string("public"),
                        keep_right(ws_or_comment(), string("object")),
                    ),
                ),
                keep_right(
                    ws_or_comment(),
                    inbetween(
                        char('('),
                        chain(
                            keep_right(ws_or_comment(), ident()),
                            chain(
                                keep_right(ws_or_comment(), hexnumber()),
                                opt(keep_right(ws_or_comment(), alt(char('+'), char('*')))),
                            ),
                        ),
                        char(')'),
                    ),
                ),
            ),
            keep_right(
                keep_right(
                    ws_or_comment(),
                    keep_right(
                        string("throws"),
                        keep_right(ws_or_comment(), string("Exception")),
                    ),
                ),
                scope_list(umpl2expr()),
            ),
        ),
        |r| {
            let name = r.0 .0.into();
            let number = r.0 .1 .0;
            let varidiac = r.0 .1 .1;
            let fn_info = varidiac.map_or(number.clone(), |varidiac| {
                UMPL2Expr::Application(vec![number, varidiac.to_string().into()])
            });
            let scope = r.1;
            UMPL2Expr::Application(vec![
                name,
                UMPL2Expr::Application(vec_splat!(vec!["lambda".into(), fn_info], scope).collect()),
            ])
        },
    )
}

fn class_stmt() -> Box<Parser<UMPL2Expr>> {
    // lisp version of class will be
    // (class name (method1 (lambda ...)) (filed1 value) (filed2) ...)
    map(
        chain(
            keep_right(
                keep_right(ws_or_comment(), string("class")),
                keep_right(ws_or_comment(), ident()),
            ),
            inbetween(
                keep_right(ws_or_comment(), char('᚜')),
                many(keep_right(
                    ws_or_comment(),
                    alt(
                        method_stmt(),
                        map(ident_umpl(), |r| UMPL2Expr::Application(vec![r])),
                    ),
                )),
                opt(keep_right(ws_or_comment(), char('᚛'))),
            ),
        ),
        |r| {
            let name = r.0;
            let fields = r.1.map_or_else(Vec::new, Iterator::collect);
            UMPL2Expr::Application(vec_splat!(vec!["class".into(), name.into()], fields).collect())
        },
    )
}

fn if_stmt() -> Box<Parser<UMPL2Expr>> {
    // TODO: allow if else if
    map(
        seq(vec![
            keep_right(string("if"), umpl2expr()),
            keep_right(
                keep_right(ws_or_comment(), string("then")),
                scope(umpl2expr()),
            ),
            keep_right(
                keep_right(ws_or_comment(), string("else")),
                scope(umpl2expr()),
            ),
        ]),
        |mut r| {
            let if_ident = "if".into();
            let cond = r.next().unwrap_or_default();
            let cons = r.next().unwrap();
            let alt = r.next().unwrap();
            UMPL2Expr::Application(vec![if_ident, cond, cons, alt])
        },
    )
}

fn until_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(string("while"), umpl2expr()),
            keep_right(
                ws_or_comment(),
                keep_right(string("do"), scope_list(umpl2expr())),
            ),
        ),
        |(cond, scope)| {
            let while_ident = "while".into();
            UMPL2Expr::Application(vec_splat(vec![while_ident, cond], scope).collect())
        },
    )
}

fn go_through_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(string("for"), keep_right(ws_or_comment(), ident_umpl())), // TODO: use identifier parserl, not the full blown expression parser
            chain(
                keep_right(keep_right(ws_or_comment(), string("in")), umpl2expr()),
                scope_list(umpl2expr()),
            ),
        ),
        |(name, (iter, scope))| {
            let for_ident = "for".into();
            UMPL2Expr::Application(vec_splat(vec![for_ident, name, iter], scope).collect())
        },
    )
}

fn continue_doing_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(string("loop"), scope_list(umpl2expr())),
        |(ident, scope)| UMPL2Expr::Application(vec_splat(vec![ident.into()], scope).collect()),
    )
}

fn link_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(
                string("ln"),
                // makeing sure that there is atleast two labels
                keep_right(ws_or_comment(), label_umpl()),
            ),
            many1(keep_right(ws_or_comment(), label_umpl())),
        ),
        |res| {
            let link_ident = "link".into();
            let goto = res.0;
            let mut link = vec![link_ident, goto];
            link.extend(res.1);
            UMPL2Expr::Application(link)
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
                string("fn"),
                opt(keep_right(
                    ws_or_comment(),
                    satify(unic_emoji_char::is_emoji_presentation),
                )),
            ),
            chain(
                opt(keep_right(ws_or_comment(), hexnumber())),
                chain(
                    opt(keep_right(ws_or_comment(), any_of(['*', '+']))),
                    scope_list(umpl2expr()),
                ),
            ),
        ),
        |r| {
            let map_to_umpl = |c: Option<char>, mapper: fn(RC<str>) -> UMPL2Expr| {
                c.as_ref()
                    .map(ToString::to_string)
                    .map(Into::into)
                    .map(mapper)
            };
            let name = map_to_umpl(r.0, UMPL2Expr::Ident);
            // TODO: maybe if no count given then randomly choose a count
            let param_count = r.1 .0.unwrap();
            let variadic = map_to_umpl(r.1 .1 .0, UMPL2Expr::String);
            let scope = r.1 .1 .1;
            let fn_ident = "lambda".into();
            // function can either be (lambda (n *) exprs) or (lambda (n +) exprs) or (lambda n exprs) n = arg count
            let lambda = if let Some(variadic) = variadic {
                UMPL2Expr::Application(
                    vec_splat!(
                        vec![
                            fn_ident,
                            UMPL2Expr::Application(vec![param_count, variadic])
                        ],
                        scope
                    )
                    .collect(),
                )
            } else {
                UMPL2Expr::Application(vec_splat(vec![fn_ident, param_count], scope).collect())
            };
            if let Some(name) = name {
                let fn_ident = "define".into();
                UMPL2Expr::Application(vec![fn_ident, name, lambda])
            } else {
                lambda
            }
        },
    )
}

fn ident_umpl() -> Box<Parser<UMPL2Expr>> {
    map(ident(), Into::into)
}

fn ident() -> Box<Parser<String>> {
    map(
        many1(not_any_of(
            call_start()
                .iter()
                .chain(special_char())
                .chain(call_end())
                .copied(),
        )),
        std::iter::Iterator::collect,
    )
}

const fn special_char() -> &'static [char] {
    &[
        '!', ' ', '᚜', '᚛', '.', '&', '|', '?', '@', '\'', '"', ';', '\n', '\t', '<', '>', '^',
        '$', ':', '%',
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
        map(string("skip"), |s| UMPL2Expr::Application(vec![s.into()])),
        list_expr(string("stop")),
    )
}

fn special_start() -> Box<Parser<UMPL2Expr>> {
    choice(vec![
        quoted_umpl(),
        label_umpl(),
        param_umpl(),
        unquoted_umpl(),
        quassi_quoted_umpl(),
    ])
}

fn list_expr(first: Box<Parser<impl Into<UMPL2Expr> + 'static>>) -> Box<Parser<UMPL2Expr>> {
    map(
        chain(map(first, Into::into), umpl2expr()),
        |(first, expr)| UMPL2Expr::Application(vec![first, expr]),
    )
}

fn quoted_umpl() -> Box<Parser<UMPL2Expr>> {
    list_expr(map(string(";"), |_| "quote"))
}

fn quassi_quoted_umpl() -> Box<Parser<UMPL2Expr>> {
    list_expr(map(string(":"), |_| "quasiquote"))
}

fn unquoted_umpl() -> Box<Parser<UMPL2Expr>> {
    list_expr(map(string("$"), |_| "unquote"))
}

fn label_umpl() -> Box<Parser<UMPL2Expr>> {
    map(keep_right(char('@'), ident()), |res| {
        UMPL2Expr::Label(res.into())
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
    // TODO: remake some of thests now that >> > < are valid in any expression
    use crate::lexer::{parse_umpl, Boolean, UMPL2Expr};

    #[test]
    pub fn umpl() {
        println!("{:?}", parse_umpl("if 1 do ᚜1 unless 1 than ᚜1 2᚛ else ᚜1᚛ 2᚛ otherwise ᚜if 1 do ᚜1 2᚛ otherwise ᚜until 1 then ᚜1 2᚛᚛᚛"));
    }

    #[test]
    pub fn umpl_no_end() {
        println!("{:?}", parse_umpl("if 1 do ᚜1 unless 1 than ᚜1 2 else ᚜1 2 otherwise ᚜if 1 do ᚜1 2 otherwise ᚜until 1 then ᚜1 2"));
    }

    // #[test]
    // pub fn umpl_if() {
    //     let test_result = parse_umpl("if ? do ᚜2 6 6᚛  otherwise ᚜4᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::If(Box::new(If::new(
    //             UMPL2Expr::Bool(Boolean::Maybee),
    //             vec![
    //                 UMPL2Expr::Number(2.0),
    //                 UMPL2Expr::Number(6.0),
    //                 UMPL2Expr::Number(6.0)
    //             ],
    //             vec![UMPL2Expr::Number(4.0)]
    //         )))
    //     );
    // }

    // #[test]
    // fn umpl_unless() {
    //     let test_result = parse_umpl("unless & than ᚜4᚛ else ᚜.t.᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::Unless(Box::new(Unless::new(
    //             UMPL2Expr::Bool(Boolean::True),
    //             vec![UMPL2Expr::Number(4.0)],
    //             vec![UMPL2Expr::String("t".into())]
    //         )))
    //     );
    // }

    // #[test]
    // fn umpl_until() {
    //     let test_result = parse_umpl("until | then ᚜ ab/᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::Until(Box::new(Until::new(
    //             UMPL2Expr::Bool(Boolean::False),
    //             vec!["ab/".into()]
    //         )))
    //     );
    // }

    // #[test]
    // fn umpl_go_through() {
    //     let test_result = parse_umpl("go-through a of (tree 5 6 7)< ᚜ .ab/.᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::GoThrough(Box::new(GoThrough::new(
    //             "a".into(),
    //             UMPL2Expr::Application((vec![
    //                 "tree".into(),
    //                 UMPL2Expr::Number(5.0),
    //                 UMPL2Expr::Number(6.0),
    //                 UMPL2Expr::Number(7.0)
    //             ],)),
    //             vec![UMPL2Expr::String("ab/".into())]
    //         )))
    //     );
    // }

    // #[test]
    // fn umpl_continue_doing() {
    //     let test_result = parse_umpl("continue-doing ᚜ lg` ᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::ContiueDoing(vec!["lg`".into()])
    //     );
    // }

    // #[test]
    // fn umpl_fn() {
    //     let test_result = parse_umpl("fanction 🚗  1 * ᚜ l ᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::Fanction(Fanction::new(
    //             Some('🚗'),
    //             1,
    //             Some(Varidiac::AtLeast0),
    //             vec!["l".into()]
    //         ))
    //     );
    // }

    #[test]
    fn umpl_ident() {
        let test_result = parse_umpl("a===a");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), "a===a".into());
    }

    #[test]
    fn umpl_number() {
        let test_result = parse_umpl("0xf%9");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::Number(15.5625));
    }

    #[test]
    fn umpl_bool() {
        let test_result = parse_umpl("?");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::Bool(Boolean::Maybee));
    }

    // #[test]
    // fn umpl_application() {
    //     let test_result = parse_umpl("{mul 5 0x10 ]>>");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         UMPL2Expr::Application((vec![
    //             "mul".into(),
    //             UMPL2Expr::Number(5.0),
    //             UMPL2Expr::Number(16.0)
    //         ],))
    //     );
    // }

    #[test]
    fn umpl_acces_param() {
        let test_result = parse_umpl("'10'");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::FnParam(8));
    }

    #[test]
    fn umpl_with_comment() {
        let test_result = parse_umpl("!t\n (1!aaa\n 22 6 ]>");
        assert!(test_result.is_ok());
    }
}
