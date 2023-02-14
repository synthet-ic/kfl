# KFL

- Nominal Typing
- Trait-Specific

## Misconceptions

- Children
  - Not eble to use enum as a single child
- Nominal
  - Flatten properties (analogous to serde)
    - Not implemented `DecodeScalar` for structs
    - Unify `DecodePartial` and `DecodeChildren`
  - NodeName
  - TypeName
  - bool child
- Option vs Default
  - benefit: form of type path doesn't matter 
- unwrap
- `NewType` for structs and `Nested` for enums

# TODO

- [ ] `DecodePartial` compatibility cannot automatically be detected
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories, reconsider `Context` together with `DecodePartial`
- [ ] Test organisation, provide utility macros
- [ ] Encode
