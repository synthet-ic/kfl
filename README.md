# KFL

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

(note, the order of properties doesn't matter as well as the order of properties with respect to arguments, so I've moved arguments to have less intersections for the arrows)

# Usage

Most common usage of this library is using `derive` and [parse] function:

```rust
use kfl::Decode;

#[derive(Decode)]
struct Config {
    #[kfl(children)]
    routes: Vec<Route>,
    #[kfl(children)]
    plugins: Vec<Plugin>,
}

#[derive(Decode)]
struct Route {
    #[kfl(argument)]
    path: String,
    #[kfl(children)]
    subroutes: Vec<Route>,
}

#[derive(Decode)]
struct Plugin {
    #[kfl(argument)]
    name: String,
    #[kfl(property)]
    url: String,
}

# fn main() -> miette::Result<()> {
let config = kfl::parse::<Config>("example.kdl", r#"
    route "/api" {
        route "/api/v1"
    }
    plugin "http" url="https://example.org/http"
"#)?;
# Ok(())
# }
```

This parses into a vector of nodes as enums `Config`, but you also use some node as a root of the document if there is no properties and arguments declared:

```rust,ignore
#[derive(Decode)]
struct Document {
    #[kfl(child, unwrap(argument))]
    version: Option<String>,
    #[kfl(children)]
    routes: Vec<Route>,
    #[kfl(children)]
    plugins: Vec<Plugin>,
}

let config = kfl::parse::<Document>("example.kdl", r#"
    version "2.0"
    route "/api" {
        route "/api/v1"
    }
    plugin "http" url="https://example.org/http"
"#)?;
```

See description of [Decode](derive@Decode) and [DecodeScalar](derive@DecodeScalar) for the full reference on allowed attributes and parse modes.

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
