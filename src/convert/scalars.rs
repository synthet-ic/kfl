//! Convert built-in scalar types.

use alloc::{
    boxed::Box,
    format,
    string::String
};
use core::str::FromStr;

use chumsky::{
    extra::Full,
    prelude::*,
};

type I<'a> = &'a str;
type Extra = Full<ParseError, Context, ()>;

use crate::{
    ast::Scalar,
    context::Context,
    errors::{DecodeError, ExpectedType, EncodeError, ParseError},
    traits::{DecodeScalar, EncodeScalar}
};

fn digit<'a>(radix: u32) -> impl Parser<'a, I<'a>, char, Extra> {
    any().filter(move |c: &char| c.is_digit(radix))
}

fn digits<'a>(radix: u32) -> impl Parser<'a, I<'a>, &'a str, Extra> {
    any().filter(move |c: &char| c == &'_' || c.is_digit(radix)).repeated().map_slice(|x| x)
}

fn decimal_number<'a>() -> impl Parser<'a, I<'a>, (u32, Box<str>), Extra> {
    just('-').or(just('+')).or_not()
    .then(digit(10)).then(digits(10))
    .then(just('.').then(digit(10)).then(digits(10)).or_not())
    .then(just('e').or(just('E'))
           .then(just('-').or(just('+')).or_not())
           .then(digits(10)).or_not())
    .map_slice(|v|
        (10, v.chars().filter(|c| c != &'_').collect::<String>().into()))
}

fn radix_number<'a>() -> impl Parser<'a, I<'a>, (u32, Box<str>), Extra> {
    // sign
    just('-').or(just('+')).or_not()
    .then_ignore(just('0'))
    .then(choice((
        just('b').ignore_then(
            digit(2).then(digits(2)).map_slice(|s| (2, s))),
        just('o').ignore_then(
            digit(8).then(digits(8)).map_slice(|s| (10, s))),
        just('x').ignore_then(
            digit(16).then(digits(16)).map_slice(|s| (16, s))),
    )))
    .map(|(sign, (radix, value))| {
        let mut s = String::with_capacity(value.len() + sign.map_or(0, |_| 1));
        sign.map(|c| s.push(c));
        s.extend(value.chars().filter(|&c| c != '_'));
        (radix, s.into())
    })
}

fn number<'a>() -> impl Parser<'a, I<'a>, (u32, Box<str>), Extra> {
    radix_number().or(decimal_number())
}

macro_rules! impl_integer {
    ($ty:ident) => {
        impl DecodeScalar for $ty {
            fn decode(scalar: &Scalar, ctx: &mut Context)
                -> Result<Self, DecodeError>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    if typ.as_ref() != stringify!($ty) {
                        return Err(DecodeError::TypeName {
                            span: ctx.span(&typ),
                            found: Some(typ.clone()),
                            expected: ExpectedType::optional(stringify!($ty)),
                            rust_type: stringify!($ty),
                        });
                    }
                }
                match number().parse_with_state(scalar.literal.as_ref(), ctx)
                    .into_result()
                {
                    Ok((radix, value)) => <$ty>::from_str_radix(&value, radix).map_err(|err| DecodeError::conversion(ctx.span(&scalar), err)),
                    Err(_) => Err(DecodeError::scalar_kind(ctx.span(&scalar), "integer", scalar.literal.clone()))  // TODO(rnarkk)
                }
            }
        }

        impl EncodeScalar for $ty {
            fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
                let literal = format!("{}", self);
                Ok(Scalar { type_name: None, literal: literal.into() })
            }
        }
    }
}

impl_integer!(i8);
impl_integer!(u8);
impl_integer!(i16);
impl_integer!(u16);
impl_integer!(i32);
impl_integer!(u32);
impl_integer!(i64);
impl_integer!(u64);
impl_integer!(isize);
impl_integer!(usize);

macro_rules! impl_decimal {
    ($ty:ident) => {
        impl DecodeScalar for $ty {
            fn decode(scalar: &Scalar, ctx: &mut Context)
                -> Result<Self, DecodeError>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    if typ.as_ref() != stringify!($ty) {
                        return Err(DecodeError::TypeName {
                            span: ctx.span(&typ),
                            found: Some(typ.clone()),
                            expected: ExpectedType::optional(stringify!($ty)),
                            rust_type: stringify!($ty),
                        });
                    }
                }
                match number().parse_with_state(scalar.literal.as_ref(), ctx).into_result() {
                    Ok((10, value)) => <$ty>::from_str(value.as_ref()).map_err(|err| DecodeError::conversion(ctx.span(&scalar), err)),
                    Ok(_) => Err(DecodeError::unexpected(ctx.span(&scalar), "radix", "radix other than 10 (decimal) is not implemented")),
                    Err(_) => Err(DecodeError::scalar_kind(ctx.span(&scalar), "decimal", scalar.literal.clone()))
                }
                // <$ty>::from_str(scalar.literal.as_ref())
            }
        }

        impl EncodeScalar for $ty {
            fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
                let literal = format!("{}", self);
                Ok(Scalar { type_name: None, literal: literal.into() })
            }
        }
    }
}

impl_decimal!(f32);
impl_decimal!(f64);

impl DecodeScalar for String {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        if let Some(typ) = scalar.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: ctx.span(&typ),
                found: Some(typ.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "String",
            });
        }
        Ok(scalar.literal.clone().into())
    }
}
impl EncodeScalar for String {
    fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
        let literal = format!("{:?}", self);
        Ok(Scalar { type_name: None, literal: literal.into() })
    }
}

macro_rules! impl_from_str {
    ($ty:ty) => {
        impl DecodeScalar for $ty {
            fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context)
                -> Result<Self, DecodeError>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    return Err(DecodeError::TypeName {
                        span: ctx.span(&typ),
                        found: Some(typ.clone()),
                        expected: ExpectedType::no_type(),
                        rust_type: stringify!($ty),
                    });
                }
                <$ty>::from_str(scalar.literal.as_ref())
                        .map_err(|err| DecodeError::conversion(
                                 ctx.span(&scalar), err))
            }
        }
    }
}

#[cfg(feature = "std")]
mod _std {
    extern crate std;
    use std::path::PathBuf;
    use std::net::SocketAddr;
    use super::*;

    impl_from_str!(PathBuf);
    impl EncodeScalar for PathBuf {
        fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
            let string = format!("{}", self.display());
            Ok(Scalar {
                type_name: None,
                literal: string.into_boxed_str()
            })
        }
    }

    impl_from_str!(SocketAddr);
    impl EncodeScalar for SocketAddr {
        fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
            let string = format!("{}", self);
            Ok(Scalar {
                type_name: None,
                literal: string.into_boxed_str()
            })
        }
    }
}

#[cfg(feature = "chrono")]
mod _chrono {
    use chrono::NaiveDateTime;
    use super::*;
    impl_from_str!(NaiveDateTime);
    impl EncodeScalar for NaiveDateTime {
        fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
            let string = format!("{}", self);
            Ok(Scalar {
                type_name: None,
                literal: string.into_boxed_str()
            })
        }
    }
}

#[cfg(feature = "http")]
mod _http {
    use http::Uri;
    use super::*;
    impl_from_str!(Uri);
    impl EncodeScalar for Uri {
        fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
            let string = format!("{}", self);
            Ok(Scalar {
                type_name: None,
                literal: string.into_boxed_str()
            })
        }
    }
}

impl DecodeScalar for bool {
    fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError> {
        if let Some(typ) = scalar.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: ctx.span(&typ),
                found: Some(typ.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "bool",
            });
        }
        match scalar.literal.as_ref() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "boolean",
                     scalar.literal.clone()))
        }
    }
}
impl EncodeScalar for bool {
    fn encode(&self, _: &mut Context) -> Result<Scalar, EncodeError> {
        let literal = match self {
            true => "true",
            false => "false"
        };
        Ok(Scalar { type_name: None, literal: literal.into() })
    }
}
