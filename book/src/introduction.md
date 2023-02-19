# Introduction

<table>
  <tr>
    <td>
      <pre lang="kdl"><code>node0</code></pre>
    </td>
    <td>
      <pre lang="kdl"><code>node1 1 "hoge"</code></pre>
    </td>
    <td>
      <pre lang="kdl"><code>node2 a=1 b="hoge"</code></pre>
    </td>
  </tr>
  <tr>
    <td colspan=3><i>structs</i></td> 
  </tr>
  <tr>
    <td rowspan=2>
      <pre>#[derive(Decode)]
struct Node0;</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node1(
    #[kfl(argument)] i32,
    #[kfl(argument)] String
);</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node2(
    #[kfl(property(name = "a"))] i32,
    #[kfl(property(name = "b"))] String
);</pre>
    </td>
  </tr>
  <tr>
    <td>
      <pre>#[derive(Decode)]
struct Node1 {
    #[kfl(argument)] a: i32,
    #[kfl(argument)] b: String
}</pre>
    </td>
    <td>
      <pre>#[derive(Decode)]
struct Node2 {
    #[kfl(property)] a: i32,
    #[kfl(property)] b: String
}</pre>
    </td>
  </tr>
  <tr>
    <td colspan=3><i>enums</i></td> 
  </tr>
  <tr>
    <td colspan=3>
      <pre>                          #[derive(Decode)]
                            enum Node {
                                Node0,
                                Node1(
                                    #[kfl(argument)] i32,
                                    #[kfl(argument)] String
                                ),
                                Node2(
                                    #[kfl(property(name = "a"))] i32,
                                    #[kfl(property(name = "b"))] String
                                )
                            }</pre>
    </td>
  </tr>
  <tr>
    <td colspan=3>
      <pre>                          #[derive(Decode)]
                            enum Node {
                                Node0,
                                Node1 {
                                    #[kfl(argument)] a: i32,
                                    #[kfl(argument)] b: String
                                },
                                Node2 {
                                    #[kfl(property)] a: i32,
                                    #[kfl(property)] b: String
                                }
                            }</pre>
    </td>
  </tr>
</table>

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
let config = kfl::decode_children::<Config>("example.kdl", r#"
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

let config = kfl::decode_children::<Document>("example.kdl", r#"
    version "2.0"
    route "/api" {
        route "/api/v1"
    }
    plugin "http" url="https://example.org/http"
"#)?;
```

See description of [Decode](derive@Decode) and [DecodeScalar](derive@DecodeScalar) for the full reference on allowed attributes and parse modes.
