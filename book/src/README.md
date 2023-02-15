# KFL

- Nominal Typing
- Trait-Specific

## Misconceptions

- Children
  - Not eble to use enum as a single child
- Nominal
  - Flatten properties (analogous to serde)
    - Not implemented `DecodeScalar` for structs
  - NodeName
  - TypeName
  - bool child
- Option vs Default
  - benefit: form of type path doesn't matter 
- unwrap
- `NewType` for structs and `Nested` for enums
- `str` and `bytes` → simply implement `DecodeScalar` for `SocketAddr` and `Vec<u8>`

# TODO

- [ ] `DecodePartial` compatibility cannot automatically be detected (except `Unit` useless case)
  - Because we discard `option` parameter and also switch to manually implementing it for `Option<T>` and `Vec<T>`
  - Beautiful fact is that default `Decode` behaviour anyway coerce `T` into `Option<T>` when `ChildMode::Normal` or `Vec<T>` when `ChildMode::Multi` at decoding (and then unwrap or into_iter.collect)
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories, reconsider `Context` in the presence of `DecodePartial`
- [ ] Test organisation, provide utility macros
- [ ] `Encode`
  - [ ] `EncodePartial` as an analogous to `skip_serializing_none`
