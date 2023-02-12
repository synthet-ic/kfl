Currently `DecodeScalar` derive is only implemented for enums

# Enums

Only enums that contain no data are supported:

```rust
#[derive(kfl::DecodeScalar)]
enum Colour {
    Red,
    Blue,
    Green,
    InfraRed,
}
```

This will match scalar values in `kebab-case`. For example, this node decoder:

```
# #[derive(kfl::DecodeScalar)]
# enum Colour { Red, Blue, Green, InfraRed }
#[derive(kfl::Decode)]
struct Document {
    #[kfl(child, unwrap(arguments))]
    all_colours: Vec<Colour>,
}
```

Can be populated from the following text:

```kdl
all-colours "red" "blue" "green" "infra-red"
```
