# knuffel

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

# 
