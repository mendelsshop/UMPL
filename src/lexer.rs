#![allow(dead_code)]

use std::iter;

use parse_int::parse;

use crate::{
    ast::{Application, Boolean, UMPL2Expr},
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
                let new_acces = |accesor: String, expr| {
                    UMPL2Expr::Application(Application::new(vec![
                        UMPL2Expr::Ident(accesor.into()),
                        expr,
                    ]))
                };
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
        |r| UMPL2Expr::Application(Application::new(r.map_or_else(Vec::new, Iterator::collect))),
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
            // unless_stmt(),
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
                        alt(
                            scope(umpl2expr()),
                            try_map(stringdot(), |string| {
                                let UMPL2Expr::String(path) = string else {
                                    panic!("error in parser combinator")
                                };
                                let fc =
                                    std::fs::read_to_string(path.to_string()).map_err(|e| {
                                        ParseError {
                                            kind: ParseErrorType::Other(format!(
                                                "failed to read file {path}: {e}"
                                            )),
                                            input: "",
                                        }
                                    })?;
                                // ther has to be abtter way then leaking maybe if moved this to eval we wuld need to leak
                                umpl_parse(Box::leak(Box::new(fc))).map(UMPL2Expr::Scope)
                            }),
                        ),
                    ),
                ),
                |(name, code)| {
                    UMPL2Expr::Application(Application::new(vec![
                        UMPL2Expr::Ident("module".into()),
                        UMPL2Expr::Ident(name.to_string().into()),
                        code,
                    ]))
                },
            ),
        ),
    )
}

fn let_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        keep_right(
            string("let"),
            chain(keep_right(ws_or_comment(), ident_umpl()), umpl2expr()),
        ),
        |r| {
            UMPL2Expr::Application(Application::new(vec![
                UMPL2Expr::Ident("define".into()),
                r.0,
                r.1,
            ]))
        },
    )
}

enum ClassStuff {
    Method(RC<str>, f64, Option<char>, UMPL2Expr),
    Field(RC<str>),
}

fn method_stmt() -> Box<Parser<ClassStuff>> {
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
                            keep_right(ws_or_comment(), ident_umpl()),
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
                scope(umpl2expr()),
            ),
        ),
        |r| {
            let UMPL2Expr::Ident(name) = r.0 .0 else {
                panic!()
            };
            let UMPL2Expr::Number(arg_num) = r.0 .1 .0 else {
                panic!()
            };
            let varidiac = r.0 .1 .1;
            let scope = r.1;
            ClassStuff::Method(name, arg_num, varidiac, scope)
        },
    )
}

fn class_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        chain(
            keep_right(
                keep_right(ws_or_comment(), string("class")),
                keep_right(ws_or_comment(), ident_umpl()),
            ),
            inbetween(
                keep_right(ws_or_comment(), char('᚜')),
                many(keep_right(
                    ws_or_comment(),
                    alt(
                        method_stmt(),
                        map(ident_umpl(), |r| {
                            let UMPL2Expr::Ident(name) = r else { panic!() };
                            ClassStuff::Field(name)
                        }),
                    ),
                )),
                opt(keep_right(ws_or_comment(), char('᚛'))),
            ),
        ),
        |r| {
            let UMPL2Expr::Ident(name) = r.0 else {
                panic!()
            };
            // let class
            let mut fields = vec![];
            let mut methods = vec![];
            r.1.unwrap_or(Box::new(iter::empty()))
                .for_each(|class| match class {
                    ClassStuff::Method(name, argc, varidic, scope) => {
                        let lambda = if let Some(variadic) = varidic {
                            UMPL2Expr::Application(Application::new(vec![
                                UMPL2Expr::Ident("lambda".into()),
                                UMPL2Expr::Number(argc),
                                UMPL2Expr::String(variadic.to_string().into()),
                                scope,
                            ]))
                        } else {
                            UMPL2Expr::Application(Application::new(vec![
                                UMPL2Expr::Ident("lambda".into()),
                                UMPL2Expr::Number(argc),
                                scope,
                            ]))
                        };
                        let method = UMPL2Expr::Application(Application::new(vec![
                            UMPL2Expr::Ident("define".into()),
                            UMPL2Expr::Ident(name.clone()),
                            lambda,
                        ]));
                        methods.push((name, argc, varidic, method));
                    }
                    ClassStuff::Field(field) => {
                        fields.push(field);
                    }
                });
            let mut class_method = vec![
                UMPL2Expr::Ident("lambda".into()),
                UMPL2Expr::Number(fields.len() as f64),
            ];

            let mut class_method_scope = vec![];
            class_method_scope.extend(fields.iter().enumerate().map(|(i, field)| {
                UMPL2Expr::Application(Application::new(vec![
                    UMPL2Expr::Ident("define".into()),
                    UMPL2Expr::Ident(field.clone()),
                    UMPL2Expr::FnParam(i),
                ]))
            }));
            class_method_scope.extend(methods.iter().map(|method| method.3.clone()));
            let mut cond_stmt = vec![UMPL2Expr::Ident("cond".into())];
            let mut count = 0;
            cond_stmt.extend(fields.iter().map(|field| {
                count += 1;
                UMPL2Expr::Application(Application::new(vec![
                    UMPL2Expr::Application(Application::new(vec![
                        UMPL2Expr::Ident("=".into()),
                        UMPL2Expr::FnParam(0),
                        UMPL2Expr::Number(count as f64 - 1.0),
                    ])),
                    UMPL2Expr::Ident(field.clone()),
                ]))
            }));

            cond_stmt.extend(methods.iter().map(|field| {
                count += 1;
                UMPL2Expr::Application(Application::new(vec![
                    UMPL2Expr::Application(Application::new(vec![
                        UMPL2Expr::Ident("=".into()),
                        UMPL2Expr::FnParam(0),
                        UMPL2Expr::Number(count as f64 - 1.0),
                    ])),
                    UMPL2Expr::Ident(field.0.clone()),
                ]))
            }));
            cond_stmt.push(UMPL2Expr::Application(Application::new(vec![
                UMPL2Expr::Ident("else".into()),
                UMPL2Expr::Application(Application::new(vec![
                    UMPL2Expr::Ident("error".into()),
                    UMPL2Expr::String(
                        format!("error not valid index for dispatch on `{name}`").into(),
                    ),
                ])),
            ])));
            let dispatch = UMPL2Expr::Application(Application::new(vec![
                UMPL2Expr::Ident("lambda".into()),
                UMPL2Expr::Number(1.0),
                UMPL2Expr::Scope(vec![UMPL2Expr::Application(Application::new(cond_stmt))]),
            ]));

            class_method_scope.push(dispatch);
            class_method.push(UMPL2Expr::Scope(class_method_scope));
            let class = vec![
                UMPL2Expr::Ident("define".into()),
                UMPL2Expr::Ident(name),
                UMPL2Expr::Application(Application::new(class_method)),
            ];
            // TODO: find a way to return multiple things ie: besides for the class method also have function not part of the class that index into the class
            UMPL2Expr::Application(Application::new(class))
        },
    )
}

fn if_stmt() -> Box<Parser<UMPL2Expr>> {
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
            let if_ident = UMPL2Expr::Ident("if".into());
            let cond = r.next().unwrap_or_default();
            let cons = r.next().unwrap();
            let alt = r.next().unwrap();
            UMPL2Expr::Application(Application::new(vec![if_ident, cond, cons, alt]))
        },
    )
}

fn get_scope(i: Option<UMPL2Expr>) -> Vec<UMPL2Expr> {
    i.and_then(UMPL2Expr::get_scope_owned).unwrap_or_default()
}

fn until_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("while"), umpl2expr()),
            keep_right(
                ws_or_comment(),
                keep_right(string("do"), scope(umpl2expr())),
            ),
        ]),
        |mut r| {
            let while_ident = UMPL2Expr::Ident("while".into());
            let cond = r.next().unwrap_or_default();
            let loop_scope = r.next().unwrap();
            UMPL2Expr::Application(Application::new(vec![while_ident, cond, loop_scope]))
        },
    )
}

fn go_through_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![
            keep_right(string("for"), keep_right(ws_or_comment(), ident_umpl())), // TODO: use identifier parserl, not the full blown expression parser
            keep_right(keep_right(ws_or_comment(), string("in")), umpl2expr()),
            scope(umpl2expr()),
        ]),
        |mut r| {
            let for_ident = UMPL2Expr::Ident("for".into());
            let iter_name = r.next().unwrap();
            let iterable = r.next().unwrap_or_default();
            let loop_scope = r.next().unwrap();
            UMPL2Expr::Application(Application::new(vec![
                for_ident, iter_name, iterable, loop_scope,
            ]))
        },
    )
}

fn continue_doing_stmt() -> Box<Parser<UMPL2Expr>> {
    map(
        seq(vec![keep_right(string("loop"), scope(umpl2expr()))]),
        |mut r| {
            let loop_ident = UMPL2Expr::Ident("loop".into());
            let loop_scope = r.next().unwrap();
            UMPL2Expr::Application(Application::new(vec![loop_ident, loop_scope]))
        },
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
            let link_ident = UMPL2Expr::Ident("link".into());
            let goto = res.0;
            let mut link = vec![link_ident, goto];
            link.extend(res.1);
            UMPL2Expr::Application(Application::new(link))
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
                    scope(umpl2expr()),
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
            // let fn_ident = UMPL2Expr::Ident("loop".into());
            let name = map_to_umpl(r.0, UMPL2Expr::Ident);
            // TODO: maybe if no count given then randomly choose a count
            let param_count = r.1 .0.unwrap();
            let variadic = map_to_umpl(r.1 .1 .0, UMPL2Expr::String);
            let scope = r.1 .1 .1;
            let fn_ident = UMPL2Expr::Ident("lambda".into());
            let lambda = if let Some(variadic) = variadic {
                UMPL2Expr::Application(Application::new(vec![
                    fn_ident,
                    param_count,
                    variadic,
                    scope,
                ]))
            } else {
                UMPL2Expr::Application(Application::new(vec![fn_ident, param_count, scope]))
            };
            if let Some(name) = name {
                let fn_ident = UMPL2Expr::Ident("define".into());
                UMPL2Expr::Application(Application::new(vec![fn_ident, name, lambda]))
            } else {
                lambda
            }
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
        '!', ' ', '᚜', '᚛', '.', '&', '|', '?', '*', '+', '@', '\'', '"', ';', '\n', '\t', '<',
        '>', '^',
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
    // TODO: remake some of thests now that >> > < are valid in any expression
    use crate::{
        ast::{Application, Fanction, GoThrough, If, Unless, Until, Varidiac},
        lexer::{parse_umpl, Boolean, UMPL2Expr},
    };

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
    //             vec![UMPL2Expr::Ident("ab/".into())]
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
    //             UMPL2Expr::Application(Application::new(vec![
    //                 UMPL2Expr::Ident("tree".into()),
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
    //         UMPL2Expr::ContiueDoing(vec![UMPL2Expr::Ident("lg`".into())])
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
    //             vec![UMPL2Expr::Ident("l".into())]
    //         ))
    //     );
    // }

    #[test]
    fn umpl_ident() {
        let test_result = parse_umpl("a===a");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), UMPL2Expr::Ident("a===a".into()));
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

    #[test]
    fn umpl_application() {
        let test_result = parse_umpl("{mul 5 0x10 ]>>");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            UMPL2Expr::Application(Application::new(vec![
                UMPL2Expr::Ident("mul".into()),
                UMPL2Expr::Number(5.0),
                UMPL2Expr::Number(16.0)
            ],))
        );
    }

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
