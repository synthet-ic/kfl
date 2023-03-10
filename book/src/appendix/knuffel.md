# knuffel

## Misconceptions

- Children
  - Not able to use enum as a single child
- Nominal
  - Flatten properties (analogous to serde)
  - NodeName
  - TypeName
    - type annotations for nodes and scalars are different
  - bool child
- `DecodeChildren`
- Option vs Default
  - form of type path doesn't matter 
- unwrap
- `NewType` for structs and `Nested` for enums
  - Every data in KDL appears either as a scalar wrapped in a node or a node itself. Therefore every type can exsist only in an already `newtype`ed form.
- `str` and `bytes` → simply implement `DecodeScalar` for `SocketAddr` and `Vec<u8>`

# Implementation

- floating-point
- `Encode`

# Change List

- Struct variants and tuple variants
- `DecodePartial` compatibility cannot automatically be detected (except `Unit` useless case)
  - Because we discard `option` parameter and also switch to manually implementing it for `Option<T>` and `Vec<T>`
  - Beautiful fact is that default `Decode` behaviour anyway coerce `T` into `Option<T>` when `ChildMode::Normal` or `Vec<T>` when `ChildMode::Multi` at decoding (and then unwrap or into_iter.collect)
- Removed `DecodeMode`
- `DecodeScalar`
  - Separation of `check_type` and `raw_decode` is inefficient when implementing `bytes` for `Vec<u8>`
    - We need to return default values
  - We can expect more that one kind of value, so DecodeError::ScalarKind cannot expect one fixed Kind
- Removed `DecodeChildren`, why?
  - If top level nodes should be considered as a set, they should be accomodated in a single node.
  - If not, they can always be decoded individually (partially).
- Removed `Document` struct as root of nodes
- Removed `Spanned`, now `Context` has spans map, prepared at parsing
  - To avoid spreading `DUMMY_SP` kind everywhere when implement`Encode`
- `Span` now our AST is span-free, spans are accomodated in context
  - `Span` is to protect from being dereferenced `AsRef` to hold pointer identity
  - Instead of `Spanned<Box<str>, S>`, just `Span` (with `Context` including the input text) suffices. Another beautiful fact.
  - chumsky now operates on its own Input trait that has an associated type `Span`, so we cannot be passive about `S` expecting only some specific set of traits consisting of third party's ones (`Into<miette::SourceSpan>`).
    - In addition to that, each span type from different crates anyway resembles at all and conversion is direct as well. So let's be bold to carry around our own.
  - Now that users can get spans from `Context` returned from `decode`, removed `span` and `span_type` directives and correspoinding fields from ast.
  - Now `DecodePatial` compatibility and `DecodeChilden` compatibility coincide.
  - `Context` is not primarily for stacking erros, the need is partially met by `DecodePartial`
- Removed `Literal` and `TypeName`
  - Now no string of literal can be decoded without specifing type; `null`, `true`, `false` for example do not inherently correspond to Rust's `None`, `true`, `false` respectively
