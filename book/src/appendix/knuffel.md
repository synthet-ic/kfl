# knuffel

## Misconceptions

- Children
  - Not eble to use enum as a single child
- Nominal
  - Flatten properties (analogous to serde)
    - Not implemented `DecodeScalar` for structs
  - NodeName
  - TypeName
    - hence type annotations for nodes and scalars are different
  - bool child
- Option vs Default
  - benefit: form of type path doesn't matter 
- unwrap
- `NewType` for structs and `Nested` for enums
  - Every data in KDL appears either as a scalar wrapped in a node or a node itself. Therefore every type can exsist only in an already `newtype`ed form.
- `str` and `bytes` → simply implement `DecodeScalar` for `SocketAddr` and `Vec<u8>`

# Implementation

# Change List

- Remove `DecodeMode`
- `DecodeScalar`
  - Separation of `check_type` and `raw_decode` is inefficient when implementing `bytes` for `Vec<u8>`
  - You have to return default values
