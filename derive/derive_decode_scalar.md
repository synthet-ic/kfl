Currently `DecodeScalar` derive is only implemented for enums

# Enums

Only enums that contain no data are supported:

```rust
use kfl::DecodeScalar;

#[derive(DecodeScalar)]
enum ColourKind {
    Red,
    Blue,
    Green,
    InfraRed,
}
```

This will match scalar values in `kebab-case`. For example, this node decoder:

```rust
# use kfl::{Decode, DecodeScalar};
# #[derive(DecodeScalar)]
# enum Colour { Red, Blue, Green, InfraRed }
#[derive(Decode)]
struct Document {
    #[kfl(children)]
    colour: Vec<Colour>,
}

#[derive(Decode)]
struct Colour(#[kfl(argument)] Colour);
```

Can be populated from the following text:

```kdl
colour "red"
colour "blue"
colour "green"
colour "infra-red"
```
