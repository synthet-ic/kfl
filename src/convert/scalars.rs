//! Convert built-in scalar types.

use alloc::{
    // boxed::Box,
    format,
    string::String
};
use core::str::FromStr;

use repr::{Pat, char::CharExt};

use crate::{
    ast::Scalar,
    context::Context,
    errors::{DecodeError, ExpectedType, EncodeError, ParseError},
    traits::{DecodeScalar, EncodeScalar}
};

fn digit<'a>(radix: u32) -> Pat {
    match radix {
        2 => p!(0..1),
        8 => p!(0..7),
        10 => p!(0..9),
        16 => p!(0..9|A..F|a..f),
        _ => panic!()
    }
}

fn digits<'a>(radix: u32) -> Pat {
    repeat!(i!(_) | digit())
}

fn decimal_number<'a>() -> Pat {
    p!(-|+)?
    & digit(10) & digits(10)
    & (p!(.) & digit(10) & digits(10))?
    & (p!(e|E) & p!(-|+)? & digits(10))?
    .map_slice(|s| (10, s.to_owned().into_boxed_str()))
}

fn radix_number<'a>() -> Pat {
    // sign
    p!(-|+)?
    & i!(0)
    & (i!(b) & (digit(2) & digits(2)).map(|s| (2, s))
    | i!(o) & (digit(8) & digits(8)).map(|s| (10, s))
    | i!(x) & (digit(16) & digits(16)).map(|s| (16, s))
    ).map(|(sign, (radix, value))| {
        let mut s = String::with_capacity(value.len() + sign.map_or(0, |_| 1));
        sign.map(|c| s.push(c));
        s.extend(value);
        (radix, s.into())
    })
}

fn number<'a>() -> Pat {
    radix_number() | decimal_number()
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
