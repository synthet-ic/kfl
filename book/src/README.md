# KFL

- Nominal Typing
- Trait-Based
- Two-Level
- Decode-Encode Duality
- Query System

## KDL

- KDL as token tree (≠ AST, ≠ token stream)
  - KDL is new string, new literal. Nodes are static just like string literals but have more structure than old literals
  - Those raw literals (nodes) can be exposed to your final structures
    - By that, it's not 'multi-staged'
- KDL as algebraic data type with exponential

## TODO

- Replace Rust's `FromStr`, `ToString`, `Display` (as primary interfaces, as least)
  - `#![no_std]`
- [ ] Should wrap Box<str>?
- [ ] Use previous version of KFL itself when testing grammar, instead of serde/serde_json
- [ ] `-`
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well?
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories
- [ ] `Encode`
  - [ ] `EncodePartial` as an analogous to `skip_serializing_none`
  - [ ] This should have means of recovering quatiented styling/formatting
- [ ] Compare TokenStream with Scalar, TokenTree with Node
- [ ] property enum or union?
