use std::{
    net::SocketAddr,
    path::PathBuf,
    str::FromStr
};

use crate::{
    ast::{Literal, Integer, Decimal, Radix, TypeName, BuiltinType},
    decode::{Context, Kind},
    errors::{DecodeError, ExpectedType},
    span::Spanned,
    traits::{ErrorSpan, DecodeScalar}
};

macro_rules! impl_integer {
    ($typ:ident, $marker:ident) => {
        impl TryFrom<&Integer> for $typ {
            type Error = <$typ as FromStr>::Err;
            fn try_from(val: &Integer) -> Result<$typ, <$typ as FromStr>::Err>
            {
                match val.0 {
                    Radix::Bin => <$typ>::from_str_radix(&val.1, 2),
                    Radix::Oct => <$typ>::from_str_radix(&val.1, 8),
                    Radix::Dec => <$typ>::from_str(&val.1),
                    Radix::Hex => <$typ>::from_str_radix(&val.1, 16),
                }
            }
        }

        impl<S: ErrorSpan> DecodeScalar<S> for $typ {
            fn raw_decode(val: &Spanned<Literal, S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                match &**val {
                    Literal::Int(ref value) => value.try_into()
                        .map_err(|err| DecodeError::conversion(val, err)),
                    _ => Err(DecodeError::scalar_kind(Kind::String, val))
                }
            }
            fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                          ctx: &mut Context<S>)
            {
                if let Some(typ) = type_name {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        ctx.emit_error(DecodeError::TypeName {
                            span: typ.span().clone(),
                            found: Some(typ.value.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($typ),
                        });
                    }
                }
            }
        }
    }
}

impl_integer!(i8, I8);
impl_integer!(u8, U8);
impl_integer!(i16, I16);
impl_integer!(u16, U16);
impl_integer!(i32, I32);
impl_integer!(u32, U32);
impl_integer!(i64, I64);
impl_integer!(u64, U64);
impl_integer!(isize, Isize);
impl_integer!(usize, Usize);

macro_rules! impl_decimal {
    ($typ:ident, $marker:ident) => {
        impl TryFrom<&Integer> for $typ {
            type Error = <$typ as FromStr>::Err;
            fn try_from(val: &Integer) -> Result<$typ, <$typ as FromStr>::Err>
            {
                <$typ>::from_str(&val.1)
            }
        }

        impl TryFrom<&Decimal> for $typ {
            type Error = <$typ as FromStr>::Err;
            fn try_from(val: &Decimal) -> Result<$typ, <$typ as FromStr>::Err>
            {
                <$typ>::from_str(&val.0)
            }
        }

        impl<S: ErrorSpan> DecodeScalar<S> for $typ {
            fn raw_decode(val: &Spanned<Literal, S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                match &**val {
                    Literal::Int(ref value) => value.try_into()
                        .map_err(|err| DecodeError::conversion(val, err)),
                    Literal::Decimal(ref value) => value.try_into()
                        .map_err(|err| DecodeError::conversion(val, err)),
                    _ => Err(DecodeError::scalar_kind(Kind::String, val))
                }
            }
            fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                          ctx: &mut Context<S>)
            {
                if let Some(typ) = type_name {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        ctx.emit_error(DecodeError::TypeName {
                            span: typ.span().clone(),
                            found: Some(typ.value.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($typ),
                        });
                    }
                }
            }
        }
    }
}

impl_decimal!(f32, F32);
impl_decimal!(f64, F64);

impl<S: ErrorSpan> DecodeScalar<S> for String {
    fn raw_decode(value: &Spanned<Literal, S>, _: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        match &**value {
            Literal::String(ref s) => Ok(s.clone().into()),
            _ => Err(DecodeError::scalar_kind(Kind::String, value))
        }
    }
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>)
    {
        if let Some(typ) = type_name {
            ctx.emit_error(DecodeError::TypeName {
                span: typ.span().clone(),
                found: Some(typ.value.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "String",
            });
        }
    }
}

macro_rules! impl_from_str {
    ($ty:ty, $display:literal) => {
        impl<S: ErrorSpan> DecodeScalar<S> for $ty {
            fn raw_decode(value: &Spanned<Literal, S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                match &**value {
                    Literal::String(ref s) => <$ty>::from_str(&s)
                        .map_err(|err| DecodeError::conversion(value, err)),
                    _ => Err(DecodeError::scalar_kind(Kind::String, value))
                }
            }
            fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                          ctx: &mut Context<S>)
            {
                if let Some(type_name) = type_name {
                    ctx.emit_error(DecodeError::TypeName {
                        span: type_name.span().clone(),
                        found: Some(type_name.value.clone()),
                        expected: ExpectedType::no_type(),
                        rust_type: $display,
                    });
                }
            }
        }
    }
}

impl_from_str!(PathBuf, "PathBuf");
impl_from_str!(SocketAddr, "SocketAddr");
#[cfg(feature = "chrono")]
impl_from_str!(chrono::NaiveDateTime, "NaiveDateTime");

impl<S: ErrorSpan> DecodeScalar<S> for bool {
    fn raw_decode(val: &Spanned<Literal, S>, _: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        match &**val {
            Literal::Bool(value) => Ok(*value),
            _ => Err(DecodeError::scalar_kind(Kind::Bool, &val))
        }
    }
    fn type_check(type_name: &Option<Spanned<TypeName, S>>,
                  ctx: &mut Context<S>)
    {
        if let Some(typ) = type_name {
            ctx.emit_error(DecodeError::TypeName {
                span: typ.span().clone(),
                found: Some(typ.value.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "bool",
            });
        }
    }
}
