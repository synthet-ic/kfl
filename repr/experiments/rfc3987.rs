//! <https://www.rfc-editor.org/rfc/rfc3987>

pub use repr::{
    consts::{EMPTY, DIGIT, SPACE, WORD as ALPHA},
    pat::Pat
};

pub const SUB_DELIMS: Pat
    = EMPTY & '!' | '$' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | ';' | '=';
pub const PCT_ENCODED: Pat = '%' & HEXDIG & HEXDIG;
// pub const UCSCHAR: Pat
//     = %xA0..D7FF | %xF900..FDCF | %xFDF0..FFEF
//     | %x10000..1FFFD | %x20000..2FFFD | %x30000..3FFFD
//     | %x40000..4FFFD | %x50000..5FFFD | %x60000..6FFFD
//     | %x70000..7FFFD | %x80000..8FFFD | %x90000..9FFFD
//     | %xA0000..AFFFD | %xB0000..BFFFD | %xC0000..CFFFD
//     | %xD0000..DFFFD | %xE1000..EFFFD;
pub const UNRESERVED: Pat
    = ALPHA | DIGIT | '-' | '.' | '_' | '~' | UCSCHAR;
pub const USERINFO: Pat = (UNRESERVED | PCT_ENCODED | SUB_DELIMS | ':') * ..;
pub const AUTHORITY: Pat = (USERINFO & '@')? & ihost & (':' & port);
pub const HIER_PART: Pat
    = "//" & AUTHORITY & ipath_abempty
    | ipath_absolute
    | ipath_rootless
    | ipath_empty;
pub const SCHEME: Pat = ALPHA & (ALPHA | DIGIT | '+' | '-' | '.') * ..;
pub const IRI: Pat
    = SCHEME & ':' & HIER_PART & ('?' & iquery)? & ('#' & ifragment)?;
