use proc_macro2::TokenStream;

mod definition;
mod kw;
mod node;
mod scalar;
mod variants;

use definition::Definition;
use scalar::{Scalar, emit_decode_scalar, emit_encode_scalar};

fn emit_decode(def: &Definition) -> syn::Result<TokenStream> {
    match def {
        Definition::Struct(s) => node::emit_decode_struct(s, true, false),
        Definition::NewType(s) => node::emit_new_type(s),
        Definition::TupleStruct(s) => node::emit_decode_struct(s, false, false),
        Definition::UnitStruct(s) => node::emit_decode_struct(s, true, false),
        Definition::Enum(e) => variants::emit_decode_enum(e),
    }
}

fn emit_decode_partial(def: &Definition) -> syn::Result<TokenStream> {
    match def {
        Definition::Struct(s) => node::emit_decode_struct(s, true, true),
        Definition::NewType(_) => todo!(),
        Definition::TupleStruct(s) => node::emit_decode_struct(s, false, true),
        Definition::UnitStruct(s) => node::emit_decode_struct(s, true, true),
        Definition::Enum(_) => todo!(),
    }
}

fn emit_encode(def: &Definition) -> syn::Result<TokenStream> {
    match def {
        Definition::Struct(s) => node::emit_encode_struct(s, false),
        Definition::NewType(_) => todo!(),
        Definition::TupleStruct(s) => node::emit_encode_struct(s, false),
        Definition::UnitStruct(s) => node::emit_encode_struct(s, false),
        Definition::Enum(e) => variants::emit_encode_enum(e),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Decode, attributes(kfl))]
// #[doc = include_str!("../derive_decode.md")]
pub fn decode_derive(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let item = syn::parse_macro_input!(input as Definition);
    match emit_decode(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(DecodePartial, attributes(kfl))]
pub fn decode_partial_derive(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let item = syn::parse_macro_input!(input as Definition);
    match emit_decode_partial(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(DecodeScalar, attributes(kfl))]
pub fn decode_scalar_derive(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let item = syn::parse_macro_input!(input as Scalar);
    match emit_decode_scalar(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Encode, attributes(kfl))]
pub fn encode_derive(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let item = syn::parse_macro_input!(input as Definition);
    match emit_encode(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(EncodeScalar, attributes(kfl))]
pub fn encode_scalar_derive(input: proc_macro::TokenStream)
    -> proc_macro::TokenStream
{
    let item = syn::parse_macro_input!(input as Scalar);
    match emit_encode_scalar(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
