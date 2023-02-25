use alloc::{format, vec::Vec};
use reflect::*;

reflect::library! {
    extern crate kfl {
        mod ast {
            type Node;
        }
        // mod context {
        //     type Context;
        // }
        // mod errors {
        //     type DecodePartial;
        // }
        mod traits {
            trait Decode {
                fn decode(node: &Node, ctx: &mut Context) -> Result<Self, DecodeError>;
            }
            
            trait DecodeScalar {
                fn decode(scalar: &Scalar, ctx: &mut Context) -> Result<Self, DecodeError>;
            }
        }
    }
}

fn derive_decode(ex: Execution) {
    ex.make_trait_impl(RUNTIME::kfl::traits::Decode, ex.target_type(), |block| {
        block.make_function(RUNTIME::kfl::traits::Decode::decode,
                            |f| decode_decode(ex.target_type(), f));
    });
}

fn derive_decode_scalar(ex: Execution) {
    ex.make_trait_impl(RUNTIME::kfl::traits::DecodeScalar, ex.target_type(), |block| {
        block.make_function(RUNTIME::kfl::traits::DecodeScalar::decode,
                            |f| decode_scalar_decode(ex.target_type(), f));
    });
}

fn decode_decode(t: Type, f: MakeFunction) -> Value {
    let node = f.arg(0);
    let ctx = f.arg(1);
    let type_name = t.get_name();
    match t.data() {
        Data::Struct(t) => match t {
            Struct::Unit(_receiver) => {
                decode_struct(type_name, node, ctx, false, false)
            },
            Struct::Tuple(_receiver) => unimplemented!(),
            Struct::Struct(receiver) => {
                let builder = RUNTIME::std::fmt::Formatter::debug_struct
                    .INVOKE(formatter, type_name)
                    .reference_mut();

                for field in receiver.fields() {
                    RUNTIME::std::fmt::DebugStruct::field.INVOKE(
                        builder,
                        field.get_name(),
                        field.get_value(),
                    );
                }

                RUNTIME::std::fmt::DebugStruct::finish.INVOKE(builder)
            }
        },
        Data::Enum(receiver) => receiver.match_variant(|variant| match variant {
            Variant::Unit(_variant) => unimplemented!(),
            Variant::Tuple(_variant) => unimplemented!(),
            Variant::Struct(_variant) => unimplemented!(),
        }),
    }
}

fn decode_scalar_decode(t: Type, f: MakeFunction) -> Value {
    let scalar = f.arg(0);
    let ctx = f.arg(1);
    // let type_name = t.get_name();
    match t.data() {
        Data::Struct(t) => match t {
            Struct::Unit(_) => unimplemented!(),
            Struct::Tuple(_) => unimplemented!(),
            Struct::Struct(_) => unimplemented!()
        },
        Data::Enum(e) => {
            let variants = e.variants();
            let value_err = if variants.len() <= 3 {
                format!("expected one of {}",
                        variants.iter()
                        .map(|v| format!("`{}`", v.get_name().escape_default()))
                        .collect::<Vec<_>>()
                        .join(", "))
            } else {
                format!("expected `{}`, `{}`, or one of {} others",
                        variants[0].name.escape_default(),
                        variants[1].name.escape_default(),
                        variants.len() - 2)
            };
        }
    }
}

pub fn decode_scalar(scalar: &Scalar, ctx: &mut Context) -> {
    match &scalar.literal {
        Literal::String(ref s) => {
            match s.as_ref() {
                #(#match_branches,)*
                _ => Err(DecodeError::conversion(
                         ctx.span(&scalar.literal), #value_err))
            }
        }
        _ => Err(DecodeError::scalar_kind(ctx.span(&scalar), "string",
                 &scalar.literal))
    }
}

fn encode_encode(t: Type, f: MakeFunction) -> Value {
    let scalar = f.arg(0);
    let ctx = f.arg(1);
    let type_name = t.get_name();
    match t.data() {
        Data::Struct(t) => match t {
            Struct::Unit(_) => unimplemented!(),
            Struct::Tuple(_) => unimplemented!(),
            Struct::Struct(_) => unimplemented!()
        },
        Data::Enum(t) => t.match_variant(|variant| match variant {
            Variant::Unit(_variant) => unimplemented!(),
            Variant::Tuple(_) => unimplemented!(),
            Variant::Struct(_) => unimplemented!(),
        }),
    }
}

fn encode_scalar_encode(t: Type, f: MakeFunction) -> Value {
    let scalar = f.arg(0);
    let ctx = f.arg(1);
    let type_name = t.get_name();
    match t.data() {
        Data::Struct(t) => match t {
            Struct::Unit(_) => unimplemented!(),
            Struct::Tuple(_) => unimplemented!(),
            Struct::Struct(_) => unimplemented!()
        },
        Data::Enum(t) => t.match_variant(|variant| match variant {
            Variant::Unit(_variant) => unimplemented!(),
            Variant::Tuple(_) => unimplemented!(),
            Variant::Struct(_) => unimplemented!(),
        }),
    }
}
