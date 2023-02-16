# KFL

- Nominal Typing
- Trait-Based

# TODO

- [ ] `-`
- [ ] Struct variants and tuple variants
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories, reconsider `Context` in the presence of `DecodePartial`
- [ ] Test organisation, provide utility macros
- [ ] `Encode`
  - [ ] `EncodePartial` as an analogous to `skip_serializing_none`
- [ ] Compare TokenStream with Scalar, TokenTree with Node
