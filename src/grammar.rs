use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::{BTreeSet, BTreeMap},
    string::String,
    vec::Vec
};
use core::fmt::{Debug, Pointer};

use chumsky::{
    extra::Full,
    input::Input,
    prelude::*,
};

use crate::{
    ast::{Node, Scalar},
    context::Context,
    errors::{ParseError, TokenFormat},
    span::Span
};

type I<'a> = &'a str;
type Extra = Full<ParseError, Context, ()>;

fn begin_comment<'a>(which: char) -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    just('/')
    .map_err(|e: ParseError| e.with_no_expected())
    .ignore_then(just(which).ignored())
}

fn newline<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    just('\r')
        .or_not()
        .ignore_then(just('\n'))
        .or(just('\r'))  // Carriage return
        .or(just('\x0C'))  // Form feed
        .or(just('\u{0085}'))  // Next line
        .or(just('\u{2028}'))  // Line separator
        .or(just('\u{2029}'))  // Paragraph separator
        .ignored()
    .map_err(|e: ParseError| e.with_expected_kind("newline"))
}

fn ws_char<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    any().filter(|c| matches!(c,
        '\t' | ' ' | '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200A}' |
        '\u{202F}' | '\u{205F}' | '\u{3000}' |
        '\u{FEFF}'
    ))
    .ignored()
}

fn id_char<'a>() -> impl Parser<'a, I<'a>, char, Extra> + Clone {
    any().filter(|c| !matches!(c,
        '\u{0000}'..='\u{0021}' |
        '\\'|'/'|'('|')'|'{'|'}'|'<'|'>'|';'|'['|']'|'='|','|'"' |
        // whitespace, excluding 0x20
        '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200A}' |
        '\u{202F}' | '\u{205F}' | '\u{3000}' |
        // newline (excluding <= 0x20)
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    ))
    .map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn id_sans_dig<'a>() -> impl Parser<'a, I<'a>, char, Extra> + Clone {
    any().filter(|c| !matches!(c,
        '0'..='9' |
        '\u{0000}'..='\u{0020}' |
        '\\'|'/'|'('|')'|'{'|'}'|'<'|'>'|';'|'['|']'|'='|','|'"' |
        // whitespace, excluding 0x20
        '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200A}' |
        '\u{202F}' | '\u{205F}' | '\u{3000}' |
        // newline (excluding <= 0x20)
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    ))
    .map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn id_sans_sign_dig<'a>() -> impl Parser<'a, I<'a>, char, Extra> + Clone {
    any().filter(|c| !matches!(c,
        '-'| '+' | '0'..='9' |
        '\u{0000}'..='\u{0020}' |
        '\\'|'/'|'('|')'|'{'|'}'|'<'|'>'|';'|'['|']'|'='|','|'"' |
        // whitespace, excluding 0x20
        '\u{00a0}' | '\u{1680}' |
        '\u{2000}'..='\u{200A}' |
        '\u{202F}' | '\u{205F}' | '\u{3000}' |
        // newline (excluding <= 0x20)
        '\u{0085}' | '\u{2028}' | '\u{2029}'
    ))
    .map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn ws<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    ws_char().repeated().at_least(1).ignored().or(ml_comment())
    .map_err(|e| e.with_expected_kind("whitespace"))
}

fn comment<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    begin_comment('/')
    .then(any().repeated().then(newline().or(end())))  // take_until
    .ignored()
}

fn ml_comment<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    recursive(|comment| {
        choice((
            comment,
            none_of('*').ignored(),
            just('*').then_ignore(none_of('/').rewind()).ignored(),
        )).repeated().ignored()
        .delimited_by(begin_comment('*'), just("*/"))
    })
    .map_err_with_span(|err, span| {
        let span = Span::from(span);
        if matches!(&err, ParseError::Unexpected { found: TokenFormat::Eoi, .. }) &&
           span.len() > 2
        {
            err.merge(ParseError::Unclosed {
                label: "comment",
                opened_at: span.at_start(2),
                opened: "/*".into(),
                expected_at: span.at_end(),
                expected: "*/".into(),
                found: None.into(),
            })
        } else {
            // otherwise opening /* is not matched
            err
        }
    })
}

// TODO(rnarkk) `then_with` method has been removed. We have to compensate it with either `then_with_ctx`, `with_ctx`, `map_ctx`, `configure`, or combination of them. https://github.com/zesterer/chumsky/pull/269
// fn raw_string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
//     just('r')
//     .ignore_then(just('#').repeated().map_slice(str::len))
//     .then_ignore(just('"'))
//     .then_with(|sharp_num|
//         take_until(
//             just('"')
//             .ignore_then(just('#').repeated().exactly(sharp_num).ignored()))
//         .map_slice(|v: &str| v.chars().collect::<String>().into())
//         .map_err_with_span(move |e: ParseError, span| {
//             let span = Span::from(span);
//             if matches!(&e, ParseError::Unexpected { found: TokenFormat::Eoi, .. }) {
//                 e.merge(ParseError::Unclosed {
//                     label: "raw string",
//                     opened_at: span.before_start(sharp_num + 2),
//                     opened: TokenFormat::OpenRaw(sharp_num),
//                     expected_at: span.at_end(),
//                     expected: TokenFormat::CloseRaw(sharp_num),
//                     found: None.into(),
//                 })
//             } else {
//                 e
//             }
//         })
//     )
// }

fn string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    // TODO(rnarkk) recover this
    // raw_string().or(escaped_string())
    escaped_string()
}

fn expected_kind(s: &'static str) -> BTreeSet<TokenFormat> {
    [TokenFormat::Kind(s)].into_iter().collect()
}

fn esc_char<'a>() -> impl Parser<'a, I<'a>, char, Extra> + Clone {
    any().try_map(|c, span: <I as Input>::Span| match c {
        '"'|'\\'|'/' => Ok(c),
        'b' => Ok('\u{0008}'),
        'f' => Ok('\u{000C}'),
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        c => {
            Err(ParseError::Unexpected {
            label: Some("invalid escape char"),
            span: Span(span.start, span.end),
            found: c.into(),
            expected: "\"\\/bfnrt".chars().map(|c| c.into()).collect(),
        })}
    })
    // TODO(rnarkk)
    // .or(
    //     just('u')
    //     .ignore_then(
    //         any()
    //         .try_map(|c: char, span: <I as Input>::Span|
    //             c.is_digit(16).then(|| c)
    //             .ok_or_else(|| {
    //                 ParseError::Unexpected {
    //                 label: Some("unexpected character"),
    //                 span: Span::from(span),
    //                 found: c.into(),
    //                 expected: expected_kind("hexadecimal digit"),
    //             }}))
    //         .repeated()
    //         .at_least(1)
    //         .at_most(6)
    //         .delimited_by(just('{'), just('}'))
    //         .map_slice(|v: &str| v)
    //         .try_map(|hex_chars, span: <I as Input>::Span| {
    //             let s = hex_chars.chars().collect::<String>();
    //             let c =
    //                 u32::from_str_radix(&s, 16).map_err(|e| e.to_string())
    //                 .and_then(|n| char::try_from(n).map_err(|e| e.to_string()))
    //                 .map_err(|e| ParseError::Message {
    //                     label: Some("invalid character code"),
    //                     span: Span(span.start, span.end),
    //                     message: e.to_string(),
    //                 })?;
    //             Ok(c)
    //         })
    //         .recover_with(skip_until(one_of(['}', '"', '\\']).map(|_| '\0')))))
}

fn escaped_string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    just('"')
    .ignore_then(
        any().filter(|&c| c != '"' && c != '\\')
        .or(just('\\').ignore_then(esc_char()))
        .repeated().map_slice(|v| v.to_owned().into_boxed_str()))
    .then_ignore(just('"'))
    .map_err_with_span(|err: ParseError, span| {
        if matches!(&err, ParseError::Unexpected { found: TokenFormat::Eoi, .. })
        {
            err.merge(ParseError::Unclosed {
                label: "string",
                opened_at: Span(span.start, span.start + 1),  //span.before_start(1),
                opened: '"'.into(),
                expected_at: Span(span.end, span.end),
                expected: '"'.into(),
                found: None.into(),
            })
        } else {
            err
        }
    })
}

// TODO(ranrkk)
fn bare_ident<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    let sign = just('+').or(just('-'));
    choice((
        sign.then(id_sans_dig().then(id_char().repeated())).map_slice(|v| v),
        sign.repeated().exactly(1).map_slice(|v| v),
        sign.repeated().then(id_sans_sign_dig()).then(id_char().repeated()).map_slice(|v| v)
    ))
    .map_slice(|s| s.to_owned().into_boxed_str())
}

fn ident<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    bare_ident().or(string())
}

fn literal<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    string()
    .or(any().filter(|c| c != &' ' && c != &'{' && c != &'}' && c != &'\n' && c != &'(' && c != &')' && c != &'\\' && c != &'=' && c != &'"').repeated().at_least(1).map_slice(|v: &str| v.chars().collect::<String>().into()))
}

fn type_name<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> + Clone {
    ident().delimited_by(just('('), just(')'))
}

fn spanned<'a, T, P>(p: P) -> impl Parser<'a, I<'a>, T, Extra> + Clone 
    where T: Pointer + Debug,
          P: Parser<'a, I<'a>, T, Extra> + Clone,
{
    p.map_with_state(|value, span, ctx| {
        ctx.set_span(&value, span.into());
        value
    })
}

fn esc_line<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    just('\\')
        .ignore_then(ws().repeated())
        .ignore_then(comment().or(newline()))
}

fn node_space<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    ws().or(esc_line())
}

fn node_terminator<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    choice((newline(), comment(), just(';').ignored(), end()))
}

#[derive(Debug)]
enum PropOrArg {
    Prop(Box<str>, Scalar),
    Arg(Scalar),
    Ignore,
}

fn type_name_value<'a>() -> impl Parser<'a, I<'a>, Scalar, Extra> + Clone {
    type_name().then(literal())
    .map(|(type_name, literal)| Scalar { type_name: Some(type_name), literal })
}

fn scalar<'a>() -> impl Parser<'a, I<'a>, Scalar, Extra> + Clone {
    type_name_value()
    .or(literal().map(|literal| Scalar { type_name: None, literal }))
}

fn prop_or_arg_inner<'a>() -> impl Parser<'a, I<'a>, PropOrArg, Extra> + Clone {
    use PropOrArg::*;
    choice((
        bare_ident().then(just('=').ignore_then(scalar()))
            .map(|(name, scalar)| Prop(name, scalar)),
        string().then(just('=').ignore_then(scalar())).map(
            |(name, scalar)| Prop(name, scalar)),
        scalar().map(Arg),
    ))
}

fn prop_or_arg<'a>() -> impl Parser<'a, I<'a>, PropOrArg, Extra> + Clone {
    begin_comment('-')
        .ignore_then(node_space().repeated())
        .ignore_then(prop_or_arg_inner())
        .map(|_| PropOrArg::Ignore)
    .or(prop_or_arg_inner())
}

fn line_space<'a>() -> impl Parser<'a, I<'a>, (), Extra> + Clone {
    newline().or(ws()).or(comment())
}

fn nodes<'a>() -> impl Parser<'a, I<'a>, Vec<Node>, Extra> {
    use PropOrArg::*;
    recursive(|nodes| {
        let braced_nodes =
            just('{')
            .ignore_then(nodes
                .then_ignore(just('}'))
                .map_err_with_span(|err, span| {
                    let span = Span::from(span);
                    if matches!(&err, ParseError::Unexpected {
                        found: TokenFormat::Eoi, .. })
                    {
                        err.merge(ParseError::Unclosed {
                            label: "curly braces",
                            // we know it's `{` at the start of the span
                            opened_at: span.before_start(1),
                            opened: '{'.into(),
                            expected_at: span.at_end(),
                            expected: '}'.into(),
                            found: None.into(),
                        })
                    } else {
                        err
                    }
                }));

        let node
            // type_name
            = ident().delimited_by(just('('), just(')')).or_not()
            // node_name
            .then(ident())
            // line_items
            .then(
                node_space()
                .repeated().at_least(1)
                .ignore_then(prop_or_arg())
                .repeated()
                .collect::<Vec<PropOrArg>>()
            )
            // opt_children
            .then(node_space().repeated()
                  .ignore_then(begin_comment('-')
                               .then_ignore(node_space().repeated())
                               .or_not())
                  .then(braced_nodes)  // spanned(braced_nodes)
                  .or_not())
            .then_ignore(node_space().repeated().then(node_terminator()))
            .map(|(((type_name, node_name), line_items), opt_children)| {
                let mut node = Node {
                    type_name,
                    node_name,
                    properties: BTreeMap::new(),
                    arguments: Vec::new(),
                    children: match opt_children {
                        Some((Some(_comment), _)) => None,
                        Some((None, children)) => Some(children),
                        None => None,
                    },
                };
                for item in line_items {
                    match item {
                        Prop(name, scalar) => {
                            node.properties.insert(name, scalar);
                        }
                        Arg(scalar) => {
                            node.arguments.push(scalar);
                        }
                        Ignore => {}
                    }
                }
                node
            });

        // comment
        begin_comment('-').then_ignore(node_space().repeated()).or_not()
        // node
        .then(spanned(node))
            .separated_by(line_space().repeated())
            .allow_leading().allow_trailing()
            .collect::<Vec<(Option<()>, Node)>>()
            .map(|vec| vec.into_iter().filter_map(|(comment, node)| {
                if comment.is_none() {
                    Some(node)
                } else {
                    None
                }
            }).collect())
    })
}

pub(crate) fn document<'a>() -> impl Parser<'a, I<'a>, Vec<Node>, Extra> {
    nodes()
}

// TODO(rnarkk) tests which need span info are comment-outed
#[cfg(test)]
mod test {
    extern crate std;
    use alloc::{borrow::ToOwned, string::String, vec::Vec};
    use chumsky::{
        prelude::*,
        extra::Full
    };
    use miette::NamedSource;
    use crate::ast::Scalar;
    use crate::context::Context;
    use crate::errors::{Error, ParseError};
    use super::{ws, comment, ml_comment, string, ident, bare_ident, literal, type_name, type_name_value, prop_or_arg_inner};
    use super::{nodes};

    type Extra = Full<ParseError, Context, ()>;

    macro_rules! err_eq {
        ($left:expr, $right:expr) => {
            let left = $left.unwrap_err();
            let left: serde_json::Value = serde_json::from_str(&left).unwrap();
            let right: serde_json::Value =
                serde_json::from_str($right).unwrap();
            assert_json_diff::assert_json_include!(
                actual: left, expected: right);
            //assert_json_diff::assert_json_eq!(left, right);
        }
    }

    fn parse<'a, P, T>(p: P, input: &'a str) -> Result<T, String>
        where P: Parser<'a, &'a str, T, Extra>
    {
        p.then_ignore(end())
        .parse(input).into_result().map_err(|errors| {
            let source = input.to_owned() + " ";
            let e = Error {
                source_code: NamedSource::new("<test>", source),
                errors: errors.into_iter().map(Into::into).collect(),
            };
            let mut buf = String::with_capacity(512);
            miette::GraphicalReportHandler::new()
                .render_report(&mut buf, &e).unwrap();
            std::println!("{}", buf);
            buf.truncate(0);
            miette::JSONReportHandler::new()
                .render_report(&mut buf, &e).unwrap();
            return buf;
        })
    }

    #[test]
    fn parse_ws() {
        parse(ws(), "   ").unwrap();
        parse(ws(), "text").unwrap_err();
    }

    #[test]
    fn parse_comments() {
        parse(comment(), "//hello").unwrap();
        parse(comment(), "//hello\n").unwrap();
        parse(ml_comment(), "/*nothing*/").unwrap();
        parse(ml_comment(), "/*nothing**/").unwrap();
        parse(ml_comment(), "/*no*thing*/").unwrap();
        parse(ml_comment(), "/*no/**/thing*/").unwrap();
        parse(ml_comment(), "/*no/*/**/*/thing*/").unwrap();
        parse(ws().then(comment()), "   // hello").unwrap();
        parse(ws().then(comment()).then(ws()).then(comment()),
              "   // hello\n   //world").unwrap();
    }

    #[test]
    fn parse_comment_err() {
        err_eq!(parse(ws(), r#"/* comment"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed comment `/*`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 2}},
                    {"label": "expected `*/`",
                    "span": {"offset": 10, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(ws(), r#"/* com/*ment *"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed comment `/*`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 2}},
                    {"label": "expected `*/`",
                    "span": {"offset": 14, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(ws(), r#"/* com/*me*/nt *"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed comment `/*`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 2}},
                    {"label": "expected `*/`",
                    "span": {"offset": 16, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(ws(), r#"/* comment *"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed comment `/*`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 2}},
                    {"label": "expected `*/`",
                    "span": {"offset": 12, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(ws(), r#"/*/"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed comment `/*`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 2}},
                    {"label": "expected `*/`",
                    "span": {"offset": 3, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        // // nothing is expected for comment or whitespace
        // err_eq!(parse(ws(), r#"xxx"#), r#"{
        //     "message": "error parsing KDL",
        //     "severity": "error",
        //     "labels": [],
        //     "related": [{
        //         "message": "found `x`, expected whitespace",
        //         "severity": "error",
        //         "filename": "<test>",
        //         "labels": [
        //             {"label": "unexpected token",
        //             "span": {"offset": 0, "length": 1}}
        //         ],
        //         "related": []
        //     }]
        // }"#);
    }

    #[test]
    fn parse_str() {
        assert_eq!(&*parse(string(), r#""hello""#).unwrap(), "hello");
        assert_eq!(&*parse(string(), r#""""#).unwrap(), "");
//         assert_eq!(&*parse(string(), r#""hel\"lo""#).unwrap(), "hel\"lo");
//         assert_eq!(&*parse(string(), r#""hello\nworld!""#).unwrap(),
//                    "hello\nworld!");
        // assert_eq!(&*parse(string(), r#""\u{1F680}""#).unwrap(), "🚀");
    }

    // #[test]
    // fn parse_raw_str() {
    //     assert_eq!(&*parse(string(), r#"r"hello""#).unwrap(), "hello");
    //     assert_eq!(&*parse(string(), r##"r#"world"#"##).unwrap(), "world");
    //     assert_eq!(&*parse(string(), r##"r#"world"#"##).unwrap(), "world");
    //     assert_eq!(&*parse(string(), r####"r###"a\n"##b"###"####).unwrap(),
    //                "a\\n\"##b");
    // }

    #[test]
    fn parse_str_err() {
        err_eq!(parse(string(), r#""hello"#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "unclosed string `\"`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "opened here",
                    "span": {"offset": 0, "length": 1}},
                    {"label": "expected `\"`",
                    "span": {"offset": 6, "length": 0}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(string(), r#""he\u{FFFFFF}llo""#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "invalid digit found in string",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "invalid character code",
                    "span": {"offset": 5, "length": 8}}
                ],
                "related": []
            }]
        }"#);
        err_eq!(parse(string(), r#""he\u{1234567}llo""#), r#"{
            "message": "error parsing KDL",
            "severity": "error",
            "labels": [],
            "related": [{
                "message": "found `7`, expected `}`",
                "severity": "error",
                "filename": "<test>",
                "labels": [
                    {"label": "unexpected token",
                    "span": {"offset": 12, "length": 1}}
                ],
                "related": []
            }]
        }"#);
        // err_eq!(parse(string(), r#""he\u{1gh}llo""#), r#"{
        //     "message": "error parsing KDL",
        //     "severity": "error",
        //     "labels": [],
        //     "related": [{
        //         "message": "found `g`, expected `}` or hexadecimal digit",
        //         "severity": "error",
        //         "filename": "<test>",
        //         "labels": [
        //             {"label": "unexpected token",
        //             "span": {"offset": 7, "length": 1}}
        //         ],
        //         "related": []
        //     }]
        // }"#);
        // err_eq!(parse(string(), r#""he\x01llo""#), r#"{
        //     "message": "error parsing KDL",
        //     "severity": "error",
        //     "labels": [],
        //     "related": [{
        //         "message":
        //             "found `x`, expected `\"`, `/`, `\\`, `b`, `f`, `n`, `r`, `t` or `u`",
        //         "severity": "error",
        //         "filename": "<test>",
        //         "labels": [
        //             {"label": "invalid escape char",
        //             "span": {"offset": 4, "length": 1}}
        //         ],
        //         "related": []
        //     }]
        // }"#);
    //     // Tests error recovery
    //     err_eq!(parse(string(), r#""he\u{FFFFFF}l\!lo""#), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "converted integer out of range for `char`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "invalid character code",
    //                 "span": {"offset": 5, "length": 8}}
    //             ],
    //             "related": []
    //         }, {
    //             "message":
    //                 "found `!`, expected `\"`, `/`, `\\`, `b`, `f`, `n`, `r`, `t` or `u`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "invalid escape char",
    //                 "span": {"offset": 15, "length": 1}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);
    }

    // TODO(rnarkk) `then_with`
    // #[test]
    // fn parse_raw_str_err() {
    //     err_eq!(parse(string(), r#"r"hello"#),  r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "unclosed raw string `r\"`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 0, "length": 2}},
    //                 {"label": "expected `\"`",
    //                 "span": {"offset": 7, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);
    //     err_eq!(parse(string(), r###"r#"hello""###), r###"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "unclosed raw string `r#\"`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 0, "length": 3}},
    //                 {"label": "expected `\"#`",
    //                 "span": {"offset": 9, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"###);
    //     err_eq!(parse(string(), r####"r###"hello"####), r####"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "unclosed raw string `r###\"`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 0, "length": 5}},
    //                 {"label": "expected `\"###`",
    //                 "span": {"offset": 10, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"####);
    //     err_eq!(parse(string(), r####"r###"hello"#world"####), r####"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "unclosed raw string `r###\"`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 0, "length": 5}},
    //                 {"label": "expected `\"###`",
    //                 "span": {"offset": 17, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"####);
    // }

    #[test]
    fn parse_ident() {
        assert_eq!(&*parse(ident(), "abcdef").unwrap(), "abcdef");
        assert_eq!(&*parse(ident(), "xx_cd$yy").unwrap(), "xx_cd$yy");
        assert_eq!(&*parse(ident(), "-").unwrap(), "-");
        assert_eq!(&*parse(ident(), "--hello").unwrap(), "--hello");
        assert_eq!(&*parse(ident(), "--hello1234").unwrap(), "--hello1234");
        assert_eq!(&*parse(ident(), "--1").unwrap(), "--1");
        assert_eq!(&*parse(ident(), "++1").unwrap(), "++1");
        assert_eq!(&*parse(ident(), "-hello").unwrap(), "-hello");
        assert_eq!(&*parse(ident(), "+hello").unwrap(), "+hello");
        assert_eq!(&*parse(ident(), "-A").unwrap(), "-A");
        assert_eq!(&*parse(ident(), "+b").unwrap(), "+b");
        assert_eq!(&*parse(ident().then_ignore(ws()), "adef   ").unwrap(),
                   "adef");
        assert_eq!(&*parse(ident().then_ignore(ws()), "a123@   ").unwrap(),
                   "a123@");
        parse(ident(), "1abc").unwrap_err();
        parse(ident(), "-1").unwrap_err();
        parse(ident(), "-1test").unwrap_err();
        parse(ident(), "+1").unwrap_err();
    }

    #[test]
    fn parse_literal() {
        parse(literal(), "true").unwrap();
        parse(literal(), "false").unwrap();
        parse(literal(), "null").unwrap();
        parse(literal(), "12").unwrap();
        parse(literal(), "012").unwrap();
        parse(literal(), "0").unwrap();
        parse(literal(), "-012").unwrap();
        parse(literal(), "+0").unwrap();
        parse(literal(), "123_555").unwrap();
        parse(literal(), "123.555").unwrap();
        parse(literal(), "+1_23.5_55E-17").unwrap();
        parse(literal(), "123e+555").unwrap();
        parse(literal(), "0x12").unwrap();
        parse(literal(), "0xab_12").unwrap();
        parse(literal(), "-0xab_12").unwrap();
        parse(literal(), "0o17").unwrap();
        parse(literal(), "+0o17").unwrap();
        parse(literal(), "0b1010_101").unwrap();
        parse(literal(), "2023-2-27").unwrap();
        parse(literal(), "127.0.0.1:80").unwrap();
        parse(literal(), "/path/to/file").unwrap();
    }

    // #[test]
    // fn exclude_keywords() {
    //     parse(nodes(), "item true").unwrap();

    //     err_eq!(parse(nodes(), "true \"item\""), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message":
    //                 "found `true`, expected identifier",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "keyword",
    //                 "span": {"offset": 0, "length": 4}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(nodes(), "item false=true"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message":
    //                 "found keyword, expected identifier or string",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected keyword",
    //                 "span": {"offset": 5, "length": 5}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(nodes(), "item 2=2"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "numbers cannot be used as property names",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected number",
    //                 "span": {"offset": 5, "length": 1}}
    //             ],
    //             "help": "consider enclosing in double quotes \"..\"",
    //             "related": []
    //         }]
    //     }"#);
    // }

    #[test]
    fn parse_type() {
        assert_eq!(parse(type_name(), "(abcdef)").unwrap(),
                   "abcdef".into());
        assert_eq!(parse(type_name(), "(xx_cd$yy)").unwrap(),
                   "xx_cd$yy".into());
        parse(type_name(), "(1abc)").unwrap_err();
        parse(type_name(), "( abc)").unwrap_err();
        parse(type_name(), "(abc )").unwrap_err();
    }

    // #[test]
    // fn parse_type_err() {
    //     err_eq!(parse(type_name(), "(123)"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "found number, expected identifier",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected number",
    //                 "span": {"offset": 1, "length": 3}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(type_name(), "(-1)"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "found number, expected identifier",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected number",
    //                 "span": {"offset": 1, "length": 2}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);
    // }

    #[test]
    fn parse_type_name_value() {
        assert_eq!(parse(type_name_value(), "(abcdef)\"hello\"").unwrap(),
                   Scalar { type_name: Some("abcdef".into()), literal: "hello".into() });
        // assert_eq!(parse(type_name_value(), "(xx_cd$yy)\"hello\"").unwrap(),
        //            "xx_cd$yy".into());
        // parse(type_name_value(), "(1abc)\"hello\"").unwrap_err();
        // parse(type_name_value(), "( abc)\"hello\"").unwrap_err();
        // parse(type_name_value(), "(abc )\"hello\"").unwrap_err();
    }

    fn single<T, E: core::fmt::Debug>(r: Result<Vec<T>, E>) -> T {
        let mut v = r.unwrap();
        assert_eq!(v.len(), 1);
        v.remove(0)
    }

    #[test]
    fn parse_prop_or_arg() {
        parse(bare_ident(), "--x").unwrap();
        parse(literal(), "2").unwrap();
        parse(prop_or_arg_inner(), "--x=2").unwrap();
    }

    #[test]
    fn parse_node() {
        let nval = single(parse(nodes(), "hello"));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);

        // let nval = single(parse(nodes(), "\"123\""));
        // assert_eq!(nval.node_name.as_ref(), "123");
        // assert_eq!(nval.type_name.as_ref(), None);

        let nval = single(parse(nodes(), "(typ)other"));
        assert_eq!(nval.node_name.as_ref(), "other");
        assert_eq!(nval.type_name.as_ref().map(|x| x.as_ref()), Some("typ"));

        let nval = single(parse(nodes(), "(\"std::duration\")\"timeout\""));
        assert_eq!(nval.node_name.as_ref(), "timeout");
        assert_eq!(nval.type_name.as_ref().map(|x| x.as_ref()),
                   Some("std::duration"));

        let nval = single(parse(nodes(), "hello \"arg1\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.properties.len(), 0);
        assert_eq!(&nval.arguments[0].literal,
                   &"arg1".into());

        let nval = single(parse(nodes(), "node \"true\""));
        assert_eq!(nval.node_name.as_ref(), "node");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.properties.len(), 0);
        assert_eq!(&nval.arguments[0].literal,
                   &"true".into());

        // let nval = single(parse(nodes(), "hello (string)\"arg1\""));
        // assert_eq!(nval.node_name.as_ref(), "hello");
        // assert_eq!(nval.type_name.as_ref(), None);
        // assert_eq!(nval.arguments.len(), 1);
        // assert_eq!(nval.properties.len(), 0);
        // assert_eq!(&**nval.arguments[0].type_name.as_ref().unwrap(),
        //            "string");
        // assert_eq!(&nval.arguments[0].literal,
        //            &"arg1".into());

        let nval = single(parse(nodes(), "hello key=(string)\"arg1\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 0);
        assert_eq!(nval.properties.len(), 1);
        assert_eq!(&**nval.properties.get("key").unwrap()
                   .type_name.as_ref().unwrap(),
                   "string");
        assert_eq!(&nval.properties.get("key").unwrap().literal,
                   &"arg1".into());

        let nval = single(parse(nodes(), "hello key=\"arg1\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 0);
        assert_eq!(nval.properties.len(), 1);
        assert_eq!(&nval.properties.get("key").unwrap().literal,
                   &"arg1".into());

        let nval = single(parse(nodes(), "parent {\nchild\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.children().len(), 1);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child");

        let nval = single(parse(nodes(), "parent {\nchild1\nchild2\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.children().len(), 2);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child1");
        assert_eq!(nval.children.as_ref().unwrap()[1].node_name.as_ref(),
                   "child2");

        let nval = single(parse(nodes(), "parent{\nchild3\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.children().len(), 1);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child3");

        let nval = single(parse(nodes(), "parent \"x\"=1 {\nchild4\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.properties.len(), 1);
        assert_eq!(nval.children().len(), 1);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child4");

        let nval = single(parse(nodes(), "parent \"x\" {\nchild4\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.children().len(), 1);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child4");

        let nval = single(parse(nodes(), "parent \"x\"{\nchild5\n}"));
        assert_eq!(nval.node_name.as_ref(), "parent");
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.children().len(), 1);
        assert_eq!(nval.children.as_ref().unwrap()[0].node_name.as_ref(),
                   "child5");

        let nval = single(parse(nodes(), "hello /-\"skip_arg\" \"arg2\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.properties.len(), 0);
        assert_eq!(&nval.arguments[0].literal,
                   &"arg2".into());

        let nval = single(parse(nodes(), "hello /- \"skip_arg\" \"arg2\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 1);
        assert_eq!(nval.properties.len(), 0);
        assert_eq!(&nval.arguments[0].literal,
                   &"arg2".into());

        let nval = single(parse(nodes(), "hello prop1=\"1\" /-prop1=\"2\""));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
        assert_eq!(nval.arguments.len(), 0);
        assert_eq!(nval.properties.len(), 1);
        assert_eq!(&nval.properties.get("prop1").unwrap().literal,
                   &"1".into());

        // let nval = single(parse(nodes(), "parent /-{\nchild\n}"));
        // assert_eq!(nval.node_name.as_ref(), "parent");
        // assert_eq!(nval.children().len(), 0);
    }

    #[test]
    fn parse_node_whitespace() {
        let nval = single(parse(nodes(), "hello  {   }"));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);

        let nval = single(parse(nodes(), "hello  {   }  "));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);

        let nval = single(parse(nodes(), "hello "));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);

        let nval = single(parse(nodes(), "hello   "));
        assert_eq!(nval.node_name.as_ref(), "hello");
        assert_eq!(nval.type_name.as_ref(), None);
    }

    #[test]
    fn parse_node_err() {
    //     err_eq!(parse(nodes(), "hello{"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "unclosed curly braces `{`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 5, "length": 1}},
    //                 {"label": "expected `}`",
    //                 "span": {"offset": 6, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);
    //     err_eq!(parse(nodes(), "hello world"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "identifiers cannot be used as arguments",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected identifier",
    //                 "span": {"offset": 6, "length": 5}}
    //             ],
    //             "help": "consider enclosing in double quotes \"..\"",
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(nodes(), "hello world {"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "identifiers cannot be used as arguments",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected identifier",
    //                 "span": {"offset": 6, "length": 5}}
    //             ],
    //             "help": "consider enclosing in double quotes \"..\"",
    //             "related": []
    //         }, {
    //             "message": "unclosed curly braces `{`",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "opened here",
    //                 "span": {"offset": 12, "length": 1}},
    //                 {"label": "expected `}`",
    //                 "span": {"offset": 13, "length": 0}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(nodes(), "1 + 2"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "found number, expected identifier",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected number",
    //                 "span": {"offset": 0, "length": 1}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);

    //     err_eq!(parse(nodes(), "-1 +2"), r#"{
    //         "message": "error parsing KDL",
    //         "severity": "error",
    //         "labels": [],
    //         "related": [{
    //             "message": "found number, expected identifier",
    //             "severity": "error",
    //             "filename": "<test>",
    //             "labels": [
    //                 {"label": "unexpected number",
    //                 "span": {"offset": 0, "length": 2}}
    //             ],
    //             "related": []
    //         }]
    //     }"#);
    }

    #[test]
    fn parse_nodes() {
        let nval = parse(nodes(), "parent {\n/-  child\n}").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].node_name.as_ref(), "parent");
        assert_eq!(nval[0].children().len(), 0);

        let nval = parse(nodes(), "/-parent {\n  child\n}\nsecond").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].node_name.as_ref(), "second");
        assert_eq!(nval[0].children().len(), 0);

    }

    #[test]
    fn parse_dashes() {
        let nval = parse(nodes(), "-").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].node_name.as_ref(), "-");
        assert_eq!(nval[0].children().len(), 0);

        let nval = parse(nodes(), "--").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].node_name.as_ref(), "--");
        assert_eq!(nval[0].children().len(), 0);

        let nval = parse(nodes(), "--1").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].node_name.as_ref(), "--1");
        assert_eq!(nval[0].children().len(), 0);

        let nval = parse(nodes(), "-\n-").unwrap();
        assert_eq!(nval.len(), 2);
        assert_eq!(nval[0].node_name.as_ref(), "-");
        assert_eq!(nval[0].children().len(), 0);
        assert_eq!(nval[1].node_name.as_ref(), "-");
        assert_eq!(nval[1].children().len(), 0);

        let nval = parse(nodes(), "node -1 --x=2").unwrap();
        assert_eq!(nval.len(), 1);
        assert_eq!(nval[0].arguments.len(), 1);
        assert_eq!(nval[0].properties.len(), 1);
        // assert_eq!(&nval[0].arguments[0].literal,
        //            &Integer(10, "-1".into()));
        // assert_eq!(&nval[0].properties.get("--x").unwrap().literal,
        //            &Integer(10, "2".into()));
    }
}
