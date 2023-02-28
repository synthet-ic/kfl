use alloc::{
    borrow::ToOwned,
    boxed::Box,
    collections::{BTreeSet, BTreeMap},
    string::{String, ToString},
    vec::Vec
};
use core::fmt::{Debug, Pointer};
use repr::{Pat, char::CharExt};
use chumsky::zero_copy::{
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

fn comment_begin<'a>(which: char)
    -> impl Parser<'a, I<'a>, (), Extra> + Clone
{
    ('/'.map_err(|e: ParseError| e.with_no_expected())
    & which
    ).map_slice(|| ())
}

fn newline<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    ('\r'? & '\n'
    | '\r'  // Carriage return
    | '\x0C'  // Form feed
    | '\u{0085}'  // Next line
    | '\u{2028}'  // Line separator
    | '\u{2029}'  // Paragraph separator
    ).map_slice(|| ())
    .map_err(|e: ParseError| e.with_expected_kind("newline"))
}

fn ws_char<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    !('\t' | ' ' | '\u{00a0}' | '\u{1680}'
    | '\u{2000}'..'\u{200A}'
    | '\u{202F}' | '\u{205F}' | '\u{3000}'
    | '\u{FEFF}'
    ).map(|_| ())
}

fn id_char<'a>() -> impl Parser<'a, I<'a>, char, Extra> {
    !('\u{0000}'..'\u{0021}'
    | '\\' | '/' | '(' | ')' | '{' | '}' | '<' | '>' | ';' | '[' | ']'
    | '=' | ',' | '"'
    // whitespace, excluding 0x20
    | '\u{00a0}' | '\u{1680}'
    | '\u{2000}'..'\u{200A}'
    | '\u{202F}' | '\u{205F}' | '\u{3000}'
    // newline (excluding <= 0x20)
    | '\u{0085}' | '\u{2028}' | '\u{2029}'
    ).map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn id_sans_dig<'a>() -> impl Parser<'a, I<'a>, char, Extra> {
    (id_char() - ('0'..'9'))
    .map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn id_sans_sign_dig<'a>() -> impl Parser<'a, I<'a>, char, Extra> {
    (id_sans_dig() - ('-' | '+'))
    .map_err(|e: ParseError| e.with_expected_kind("letter"))
}

fn ws<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    (ws_char() * (1..) | ml_comment())
    .map_err(|e| e.with_expected_kind("whitespace"))
}

fn comment<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    (comment_begin('/') & take_until(newline() | end())).map(|_| ())
}

fn ml_comment<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    recursive(|comment| {
        comment_begin('*')
        & [comment | !'*' | '*' & ignore((!'/').rewind())]
        & "*/"
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
fn raw_string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    'r'
    & ['#'].map_slice(str::len)
    & '"'
    .then_with(|sharp_num|
        take_until('"' & '#' * sharp_num)
        .map_slice(|s| own!(s))
        .map_err_with_span(move |e: ParseError, span| {
            let span = Span::from(span);
            if matches!(&e, ParseError::Unexpected { found: TokenFormat::Eoi, .. }) {
                e.merge(ParseError::Unclosed {
                    label: "raw string",
                    opened_at: span.before_start(sharp_num + 2),
                    opened: TokenFormat::OpenRaw(sharp_num),
                    expected_at: span.at_end(),
                    expected: TokenFormat::CloseRaw(sharp_num),
                    found: None.into(),
                })
            } else {
                e
            }
        })
    )
}

fn string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    // TODO(rnarkk) recover this
    // raw_string().or(escaped_string())
    escaped_string()
}

fn expected_kind(s: &'static str) -> BTreeSet<TokenFormat> {
    [TokenFormat::Kind(s)].into_iter().collect()
}

fn esc_char<'a>() -> impl Parser<'a, I<'a>, char, Extra> {
    any().try_map(|c, span: <I as Input>::Span| match c {
        '"' | '\\' | '/' => Ok(c),
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
    .or(
        ignore('u')
        & (
            ignore('{')
            & any().try_map(|c: char, span: <I as Input>::Span|
                c.is_digit(16).then(|| c)
                .ok_or_else(|| {
                    ParseError::Unexpected {
                    label: Some("unexpected character"),
                    span: Span::from(span),
                    found: c.into(),
                    expected: expected_kind("hexadecimal digit"),
                }}))
            * (1..6)
            & ignore('}')
            .map_slice(|v: &str| v)
            .try_map(|hex_chars, span: <I as Input>::Span| {
                let s = hex_chars.chars().collect::<String>();
                let c =
                    u32::from_str_radix(&s, 16).map_err(|e| e.to_string())
                    .and_then(|n| char::try_from(n).map_err(|e| e.to_string()))
                    .map_err(|e| ParseError::Message {
                        label: Some("invalid character code"),
                        span: Span(span.start, span.end),
                        message: e.to_string(),
                    })?;
                Ok(c)
            })
            .recover_with(skip_until(('}' | '"' | '\\').map(|_| '\0')))))
}

fn escaped_string<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    (
        '"'
        & [!('"' | '\\') | ignore('\\') & esc_char()]
        & '"'
    )
    .map_slice(|(_, s, _)| own!(s))
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
fn bare_ident<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    let sign = '+' | '-';
    (sign & id_sans_dig() & [id_char()]
    | sign
    | ([sign] & id_sans_sign_dig() & [id_char()])
    ).map_slice(|s| own!(s))
}

fn ident<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    bare_ident() | string()
}

fn literal<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    string()
    | (!(' ' | '{' | '}' | '\n' | '(' | ')' | '\\' | '=' | '"') * (1..))
        .map_slice(|s| own!(s))
}

fn type_name<'a>() -> impl Parser<'a, I<'a>, Box<str>, Extra> {
    ('(' & ident() & ')').map_slice(|(_, s, _)| s)
}

fn spanned<'a, T, P>(p: P) -> impl Parser<'a, I<'a>, T, Extra>
    where T: Pointer + Debug,
          P: Parser<'a, I<'a>, T, Extra>,
{
    p.map_with_state(|value, span, ctx| {
        ctx.set_span(&value, span.into());
        value
    })
}

fn esc_line<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    '\\' & [ws()] & (comment() | newline())
}

fn node_space<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    ws() | esc_line()
}

fn node_terminator<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    newline() | comment() | ignore(';') | end()
}

#[derive(Debug)]
enum PropOrArg {
    Prop(Box<str>, Scalar),
    Arg(Scalar),
    Ignore,
}

use PropOrArg::*;

fn type_name_value<'a>() -> impl Parser<'a, I<'a>, Scalar, Extra> {
    (type_name() & literal()).map(|(ty, lit)| Scalar::new(ty, lit))
}

fn scalar<'a>() -> impl Parser<'a, I<'a>, Scalar, Extra> {
    type_name_value() | literal().map(|lit| Scalar::from(lit))
}

fn prop_or_arg_inner<'a>() -> impl Parser<'a, I<'a>, PropOrArg, Extra>
{
    (bare_ident() & '=' & scalar()).map(|(name, _, scalar)| Prop(name, scalar))
    | (string() & '=' & scalar()).map(|(name, _, scalar)| Prop(name, scalar))
    | scalar().map(Arg)
}

fn prop_or_arg<'a>() -> impl Parser<'a, I<'a>, PropOrArg, Extra> {
    (comment_begin('-') & [node_space()] & prop_or_arg_inner()).map(|_| Ignore)
    | prop_or_arg_inner()
}

fn line_space<'a>() -> impl Parser<'a, I<'a>, (), Extra> {
    newline() | ws() | comment()
}

fn nodes<'a>() -> impl Parser<'a, I<'a>, Vec<Node>, Extra> {
    recursive(|nodes| {
        let braced_nodes =
            ('{' & nodes & '}')
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
            });

        let node
            // type_name
            = ('(' & ident() & ')')?.map_slice(|(_, s, _)| s)
            // node_name
            & ident()
            // line_items
            & (
                [(ignore(node_space() * 1..) & prop_or_arg())]
                .collect::<Vec<PropOrArg>>()
            )
            // opt_children
            & (
                [node_space()]
                & (comment_begin('-') & [node_space()])?
                & braced_nodes  // spanned(braced_nodes)
            )?.map_slice(|(_, _, n)| n)
            & ignore([node_space()] & node_terminator())
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
        (comment_begin('-') & ignore([node_space()]))?
        // node
        .then(spanned(node))
        .separated_by([line_space()])
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
    nodes() & end()
}

// TODO(rnarkk) tests which need span info are comment-outed
#[cfg(test)]
mod test {
    extern crate std;
    use alloc::{borrow::ToOwned, string::String, vec::Vec};
    use chumsky::zero_copy::{
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
