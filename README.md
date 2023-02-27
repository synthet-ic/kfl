# KFL

[mdbook](https://synthet-ic.github.io/kfl/)

- Nominal Typing
- Trait-Based
- Two-Level
- Decode-Encode Dual
- Reflective

A [KDL](https://kdl.dev) file format parser with great error reporting and convenient derive macros.

# About KDL

To give you some background on the KDL format. Here is a small example:

```kdl
foo 1 "three" key="val" {
    bar
    (role)baz 1 2
}
```

Here is what are annotations for all the datum as described by the [specification] and this guide:

```text
foo 1 "three" key="val" {                           ╮
─┬─ ┬ ───┬─── ────┬────                             │
 │  │    │        ╰───── property (can be multiple) │
 │  │    │                                          │
 │  ╰────┴────────────── arguments                  │
 │                                                  │
 ╰── node name                                      ├─ node "foo", with
                                                    │  "bar" and "baz"
    bar                                             │  being children
    (role)baz 1 2                                   │
     ──┬─                                           │
       ╰────── type name for node named "baz"       │
}                                                   ╯
```

# Usage

Most common usage of this library is using `derive` and [parse] function:

```rust
use kfl::{Decode, DecodePartial, Encode};
use std::path::PathBuf;
use http::Uri;

#[derive(DecodePartial, Default)]
struct Document {
    #[kfl(children)]
    routes: Vec<Route>,
    #[kfl(children)]
    plugins: Vec<Plugin>,
}

#[derive(Decode, Encode)]
struct Route {
    #[kfl(argument)]
    path: PathBuf,
    #[kfl(children)]
    subroutes: Vec<Route>,
}

#[derive(Decode, Encode)]
struct Plugin {
    #[kfl(argument)]
    name: String,
    #[kfl(property)]
    url: Uri,
}

# fn main() -> miette::Result<()> {
let document = kfl::decode_children::<Document>("example.kdl", r#"
    route /api {
        route /api/v1
    }
    plugin "http" url=https://example.org/http
"#)?;
# Ok(())
# }
```

License
=======

Licensed under either of

* Apache License, Version 2.0,
  (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
  at your option.

Contribution
------------

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[specification]: https://github.com/kdl-org/kdl/blob/main/SPEC.md
[miette]: https://docs.rs/miette/
[miette guide]: https://docs.rs/miette/latest/miette/#-handler-options
