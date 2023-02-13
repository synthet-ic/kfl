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
- Option vs Default
- unwrap
- new type

# TODO

- [ ] Implement `DecodeScalar` for struct as the replacement of `flat(properties)`
- [ ] Allow distinguish variant nodes by `(type-name)`
- [ ] Span
