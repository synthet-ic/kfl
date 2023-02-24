#![no_std]
extern crate alloc;

mod definition;
mod kw;
mod new_type;
mod node;
mod scalar;
mod variants;

use proc_macro::TokenStream;
use alloc::{format, string::String};
use definition::Definition;
use scalar::{Scalar, emit_decode_scalar, emit_encode_scalar};

type EmitResult = syn::Result<proc_macro2::TokenStream>;

fn emit_decode(def: &Definition) -> EmitResult {
    match def {
        Definition::Struct(s) => node::emit_decode_struct(s, true, false),
        Definition::NewType(s) => new_type::emit_new_type(s),
        Definition::TupleStruct(s) => node::emit_decode_struct(s, false, false),
        Definition::UnitStruct(s) => node::emit_decode_struct(s, true, false),
        Definition::Enum(e) => variants::emit_decode_enum(e),
    }
}

fn emit_decode_partial(def: &Definition) -> EmitResult {
    match def {
        Definition::Struct(s) => node::emit_decode_struct(s, true, true),
        Definition::NewType(_) => todo!(),
        Definition::TupleStruct(s) => node::emit_decode_struct(s, false, true),
        Definition::UnitStruct(s) => node::emit_decode_struct(s, true, true),
        Definition::Enum(_) => todo!(),
    }
}

fn emit_encode(def: &Definition) -> EmitResult {
    match def {
        Definition::Struct(s) => node::emit_encode_struct(s, false),
        Definition::NewType(_) => todo!(),
        Definition::TupleStruct(s) => node::emit_encode_struct(s, false),
        Definition::UnitStruct(s) => node::emit_encode_struct(s, false),
        Definition::Enum(e) => variants::emit_encode_enum(e),
    }
}

fn emit_encode_partial(def: &Definition) -> EmitResult {
    match def {
        Definition::Struct(s) => node::emit_encode_struct(s, true),
        Definition::NewType(_) => todo!(),
        Definition::TupleStruct(s) => node::emit_encode_struct(s, true),
        Definition::UnitStruct(s) => node::emit_encode_struct(s, true),
        Definition::Enum(_) => todo!(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Decode, attributes(kfl))]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Definition);
    match emit_decode(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(DecodePartial, attributes(kfl))]
pub fn decode_partial_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Definition);
    match emit_decode_partial(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(DecodeScalar, attributes(kfl))]
pub fn decode_scalar_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Scalar);
    match emit_decode_scalar(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Encode, attributes(kfl))]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Definition);
    match emit_encode(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(EncodePartial, attributes(kfl))]
pub fn encode_partial_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Definition);
    match emit_encode_partial(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

// #[proc_macro_error::proc_macro_error]
#[proc_macro_derive(EncodeScalar, attributes(kfl))]
pub fn encode_scalar_derive(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as Scalar);
    match emit_encode_scalar(&item) {
        Ok(stream) => stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

pub(crate) fn to_kebab_case(ident: &syn::Ident) -> String {
    heck::ToKebabCase::to_kebab_case(format!("{}", ident).as_str())
}
