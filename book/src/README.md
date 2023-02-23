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

### Designing Basics

- [ ] Delete `DecodeChildren`
- `flatten`
  - [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well?
  - [ ] flatten for enums
- [ ] Detect name conflicts between fields in the same struct
- [ ] property enum or union?

### Documentation

### Source Code Quality

### Testing

- [ ] Span
  - [ ] Should wrap Box<str>?
- [ ] Understand error categories
- [ ] Rename `parse_` to `decode_` and `print_` to `encode_` in tests?
- [ ] Use previous version of KFL itself when testing grammar, instead of serde/serde_json

### Encoding

- [ ] 
- [ ] Encoding should have ways of recovering quatiented styling/formatting

### Scalars

- Instad of predetermining strings or patterns of chars valid for scalar values, determine syntactical separation condition between meta structure and scalar representations. This would give us possible scope of scalar notations, more relaxed ones if successful, and ability for scalar objects, in AST, to hold raw strings or slices to delay parsing them.
- Eliminate `FromStr`, `ToString`, `Display` (as primary interfaces, as least)
  - `#![no_std]`
- [ ] Compare TokenStream with Scalar, TokenTree with Node

### Specials

- [ ] `-`
