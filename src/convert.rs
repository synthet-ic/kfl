use std::{
    net::SocketAddr,
    path::PathBuf,
    str::FromStr
};

use crate::{
    ast::{Scalar, Literal, Integer, Decimal, Radix, BuiltinType},
    decode::Context,
    errors::{DecodeError, ExpectedType, EncodeError},
    traits::{ErrorSpan, DecodeScalar, EncodeScalar}
};

macro_rules! impl_integer {
    ($ty:ident, $marker:ident) => {
        impl TryFrom<&Integer> for $ty {
            type Error = <$ty as FromStr>::Err;
            fn try_from(val: &Integer) -> Result<$ty, <$ty as FromStr>::Err>
            {
                match val.0 {
                    Radix::Bin => <$ty>::from_str_radix(&val.1, 2),
                    Radix::Oct => <$ty>::from_str_radix(&val.1, 8),
                    Radix::Dec => <$ty>::from_str(&val.1),
                    Radix::Hex => <$ty>::from_str_radix(&val.1, 16),
                }
            }
        }

        impl<S: ErrorSpan> DecodeScalar<S> for $ty {
            fn decode(scalar: &Scalar, ctx: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        return Err(DecodeError::TypeName {
                            span: ctx.span(&typ),
                            found: Some(typ.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($ty),
                        });
                    }
                }
                match &scalar.literal {
                    Literal::Int(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                                 ctx.span(&scalar), err)),
                    _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                             &scalar.literal))
                }
            }
        }

        impl TryFrom<&$ty> for Integer {
            type Error = <$ty as FromStr>::Err;
            fn try_from(val: &$ty) -> Result<Integer, <$ty as FromStr>::Err>
            {
                Ok(Integer(
                    Radix::Oct,
                    val.to_string().into()
                ))
            }
        }

        impl<S: ErrorSpan> EncodeScalar<S> for $ty {
            fn encode(&self, _: &mut Context<S>)
                -> Result<Scalar, EncodeError<S>>
            {
                let literal = Literal::Int(Integer::try_from(self).unwrap());
                Ok(Scalar {
                    type_name: None,
                    literal: literal.into()
                })
                
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
    ($ty:ident, $marker:ident) => {
        impl TryFrom<&Integer> for $ty {
            type Error = <$ty as FromStr>::Err;
            fn try_from(val: &Integer) -> Result<$ty, <$ty as FromStr>::Err>
            {
                <$ty>::from_str(&val.1)
            }
        }

        impl TryFrom<&Decimal> for $ty {
            type Error = <$ty as FromStr>::Err;
            fn try_from(val: &Decimal) -> Result<$ty, <$ty as FromStr>::Err>
            {
                <$ty>::from_str(&val.0)
            }
        }

        impl<S: ErrorSpan> DecodeScalar<S> for $ty {
            fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    if typ.as_builtin() != Some(&BuiltinType::$marker) {
                        return Err(DecodeError::TypeName {
                            span: ctx.span(&typ).clone(),
                            found: Some(typ.clone()),
                            expected: ExpectedType::optional(
                                BuiltinType::$marker),
                            rust_type: stringify!($ty),
                        });
                    }
                }
                match &scalar.literal {
                    Literal::Int(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                                 ctx.span(&scalar), err)),
                    Literal::Decimal(ref v) => v.try_into()
                        .map_err(|err| DecodeError::conversion(
                                 ctx.span(&scalar), err)),
                    _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                             &scalar.literal))
                }
            }
        }
    }
}

impl_decimal!(f32, F32);
impl_decimal!(f64, F64);

impl<S: ErrorSpan> DecodeScalar<S> for String {
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        if let Some(typ) = scalar.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: ctx.span(&typ),
                found: Some(typ.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "String",
            });
        }
        match &scalar.literal {
            Literal::String(ref s) => Ok(s.clone().into()),
            _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                                              &scalar.literal))
        }
    }
}

macro_rules! impl_from_str {
    ($ty:ty, $display:literal) => {
        impl<S: ErrorSpan> DecodeScalar<S> for $ty {
            fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
                -> Result<Self, DecodeError<S>>
            {
                if let Some(typ) = scalar.type_name.as_ref() {
                    return Err(DecodeError::TypeName {
                        span: ctx.span(&typ),
                        found: Some(typ.clone()),
                        expected: ExpectedType::no_type(),
                        rust_type: $display,
                    });
                }
                match &scalar.literal {
                    Literal::String(ref s) => <$ty>::from_str(&s)
                        .map_err(|err| DecodeError::conversion(
                                 ctx.span(&scalar), err)),
                    _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                             &scalar.literal))
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
    fn decode(scalar: &crate::ast::Scalar, ctx: &mut Context<S>)
        -> Result<Self, DecodeError<S>>
    {
        if let Some(typ) = scalar.type_name.as_ref() {
            return Err(DecodeError::TypeName {
                span: ctx.span(&type),
                found: Some(typ.clone()),
                expected: ExpectedType::no_type(),
                rust_type: "bool",
            });
        }
        match &scalar.literal {
            Literal::Bool(v) => Ok(*v),
            _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "boolean", &scalar.literal))
        }
    }
}
