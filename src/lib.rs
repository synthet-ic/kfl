#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

mod containers;
mod convert;
mod convert_ast;
mod grammar;
mod wrappers;

pub mod ast;
pub mod context;
pub mod errors;
pub mod span;
pub mod traits;

#[cfg(feature = "derive")]
pub use kfl_derive::{Decode, DecodePartial, DecodeScalar};
#[cfg(feature = "derive")]
pub use kfl_derive::{Encode, /* EncodePartial,*/ EncodeScalar};

pub use wrappers::{decode, decode_children, decode_with_context, parse};
pub use wrappers::{encode, print};
pub use traits::{Decode, DecodePartial, DecodeScalar, DecodeChildren};
pub use traits::{Encode, EncodePartial, EncodeScalar, EncodeChildren};
pub use errors::Error;
