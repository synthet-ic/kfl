use std::{
    net::SocketAddr,
    path::PathBuf,
    str::FromStr
};

use crate::{
    ast::{Literal, Integer, Decimal, Radix, BuiltinType},
    decode::Context,
    errors::{DecodeError, ExpectedType},
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
            fn decode(value: &crate::ast::Value<S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = value.type_name.as_ref() {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        return Err(DecodeError::TypeName {
                            span: typ.span().clone(),
                            found: Some(typ.value.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($typ),
                        });
                    }
                }
                match &*value.literal {
                    Literal::Int(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                            &value.literal, err)),
                    _ => Err(DecodeError::scalar_kind("string",
                             &value.literal))
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
            fn decode(value: &crate::ast::Value<S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = value.type_name.as_ref() {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        return Err(DecodeError::TypeName {
                            span: typ.span().clone(),
                            found: Some(typ.value.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($typ),
                        });
                    }
                }
                match &*value.literal {
                    Literal::Int(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                            &value.literal, err)),
                    Literal::Decimal(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                            &value.literal, err)),
                    _ => Err(DecodeError::scalar_kind("string",
                             &value.literal))
                }
            }
        }
    }
}

impl_decimal!(f32, F32);
impl_decimal!(f64, F64);

impl<S: ErrorSpan> DecodeScalar<S> for String {
    fn decode(value: &crate::ast::Value<S>, _: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        if let Some(typ) = value.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: typ.span().clone(),
                found: Some(typ.value.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "String",
            });
        }
        match &*value.literal {
            Literal::String(ref s) => Ok(s.clone().into()),
            _ => Err(DecodeError::scalar_kind("string", &value.literal))
        }
    }
}

macro_rules! impl_from_str {
    ($ty:ty, $display:literal) => {
        impl<S: ErrorSpan> DecodeScalar<S> for $ty {
            fn decode(value: &crate::ast::Value<S>, _: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = value.type_name.as_ref() {
                    return Err(DecodeError::TypeName {
                        span: typ.span().clone(),
                        found: Some(typ.value.clone()),
                        expected: ExpectedType::no_type(),
                        rust_type: $display,
                    });
                }
                match &*value.literal {
                    Literal::String(ref s) => <$ty>::from_str(&s)
                        .map_err(|err| DecodeError::conversion(
                                 &value.literal, err)),
                    _ => Err(DecodeError::scalar_kind("string",
                             &value.literal))
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
    fn decode(value: &crate::ast::Value<S>, _: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        if let Some(typ) = value.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: typ.span().clone(),
                found: Some(typ.value.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "bool",
            });
        }
        match &*value.literal {
            Literal::Bool(v) => Ok(*v),
            _ => Err(DecodeError::scalar_kind("boolean", &value.literal))
        }
    }
}
