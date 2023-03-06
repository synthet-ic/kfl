//! <https://www.apple.com/DTDs/PropertyList-1.0.dtd>

use chrono::NaiveDateTime;
use kfl::{Decode, DecodePartial};

#[derive(Debug, Decode)]
pub struct PList {
    #[kfl(property)]
    pub version: String,
    #[kfl(children)]
    pub elements: Vec<Element>
}

#[derive(Debug, Decode)]
pub enum Element {
    Array(#[kfl(children)] Vec<Element>),
    Data(#[kfl(argument)] Vec<u8>),
    Date(#[kfl(argument)] NaiveDateTime),
    Dict(#[kfl(flatten)] Dict),
    Real(#[kfl(argument, default)] f32),
    Integer(#[kfl(argument, default)] i32),
    String(#[kfl(argument, default)] String),
    True,
    False
}

#[derive(Debug, DecodePartial, Default)]
pub struct Dict {
    #[kfl(children)]
    pub keys: Vec<Key>,
    #[kfl(children)]
    pub values: Vec<Element>
}

#[derive(Debug, Decode)]
pub struct Key(#[kfl(argument)] pub String);
