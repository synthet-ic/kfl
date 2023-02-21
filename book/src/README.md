# KFL

- Nominal Typing
- Trait-Based
- Two-Level

## KDL

- KDL as token tree (≠ AST, ≠ token stream)
  - KDL is new string, new literal witch has more structure
- KDL as algebraic data type with exponential

## TODO

- [ ] Make AST span free, accomodate span in context
- [ ] Use previous version of KFL itself when testing grammar, instead of serde/serde_json
- [ ] `-`
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories, reconsider `Context` in the presence of `DecodePartial`
- [ ] Test organisation, provide utility macros
- [ ] `Encode`
  - [ ] `EncodePartial` as an analogous to `skip_serializing_none`
- [ ] Compare TokenStream with Scalar, TokenTree with Node
- [ ] property enum or union
