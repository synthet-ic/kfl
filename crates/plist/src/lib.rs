/*!
<https://www.apple.com/DTDs/PropertyList-1.0.dtd>
*/

use chrono::NaiveDateTime;
use kfl::Decode;

#[derive(Debug, Decode)]
pub struct PList {
    #[kfl(property)]
    pub version: String,
    #[kfl(children)]
    pub elements: Vec<Element>
}

#[derive(Debug, Decode)]
pub enum Element {
    Array(#[kfl(children)] pub Vec<Box<Element>>),
    Data(#[kfl(argument, bytes)] pub Vec<u8>),
    Date(#[kfl(argument, str)] pub NaiveDateTime),
    Dict(pub Box<Dict>),
    Real(#[kfl(argument, default)] pub f32),
    Integer(#[kfl(argument, default)] pub i32),
    String(#[kfl(argument, default)] pub String),
    Bool(#[kfl(argument)] bool)
}

#[derive(Debug, Decode)]
pub struct Dict {
    #[kfl(children)]
    pub keys: Vec<Key>,
    #[kfl(children)]
    pub values: Vec<Element>
}

#[derive(Debug, Decode)]
pub struct Key(#[kfl(argument)] pub String);
