# Tutorials

## Case 1: All Arguments

### Case 1.1: With Different Types

```kdl
node 1 "hoge"
```

#### Pattern 1.1.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(argument)] i32, #[kfl(argument)] String);
```

#### Pattern 1.1.2 Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(argument)]
    a: i32,
    #[kfl(argument)]
    b: String
}
```

NOTE: `a` and `b` are redundant.

### Case 1.2: With The Same Type

```kdl
node 1 2
```

#### Pattern 1.2.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(arguments)] Vec<i32>);
```

#### Pattern 1.2.2 Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(arguments)]
    a: Vec<i32>
}
```

NOTE: `a` is redundant.

## Case 2: All Properties

### Case 2.1: With Different Types

```kdl
node a=1 b="hoge"
```

#### Pattern 2.1.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(property(name = "a"))] i32, #[kfl(property(name = "b"))] String);
```

NOTE: `name = "a"` and `name = "b"` are necessary.

### Pattern 2.1.2: Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(property)]
    a: i32,
    #[kfl(property)]
    b: String
}
```

### Case 2.2: With The Same Type

```kdl
node a=1 b=2
```

#### Pattern 2.2.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(properties)] HashMap<String, i32>);
```

### Pattern 2.2.2: Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(properties)]
    a: HashMap<String, i32>
}
```

NOTE: `a` is redundant.

## Case 3: Mixture of Singulars and A Plural

### Case 3.1: Arguments

```kdl
node 1 2 3
```

#### Pattern 3.1.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(argument)] i32, #[kfl(arguments)] Vec<i32>);
```

#### Pattern 3.1.2: Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(argument)]
    a: i32,
    #[kfl(arguments)]
    b: Vec<i32>
}
```

### Case 3.2: Properties

```kdl
node a=1 b=2 c=3
```

#### Pattern 3.2.1: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(property(name = "a")] i32, #[kfl(properties)] HashMap<String, i32>);
```

NOTE: `name = "a"` is necessary.

#### Pattern 3.2.2: Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(property)]
    a: i32,
    #[kfl(properties)]
    b: HashMap<String, i32>
}
```

### Case 4: Mix of Arguments and Properties

### Case 5: Child

```kdl
node {
  child 1
}
```

#### Pattern 5.: Tuple Struct

```rust
#[derive(Decode)]
struct Node(#[kfl(child)] A);

// Or Struct Struct
#[derive(Decode)]
struct A(#[kfl(argument)] i32);
```

#### Pattern 5.: Struct Struct

```rust
#[derive(Decode)]
struct Node {
    #[kfl(child)]
    child: A
}

// Or Struct Struct
#[derive(Decode)]
struct A(#[kfl(argument)] i32);
```

NOTE: `A` is redundant.

### Case ?: Children

### Case ?: Alteration

```kdl
node0 1
```

```kdl
node1 "hoge"
```

```rust
#[derive(Decode)]
enum Node {
    Node0(#[kfl(argument)] i32),
    Node1(#[kfl(argument)] String)
}
```

```rust
// #[derive(Debug, kfl::Decode)]
// struct Document {
//     #[kfl(children(name = "plugin"))]
//     plugins: Vec<String>,
//     #[kfl(children(name = "file"))]
//     files: Vec<String>,
// }

// #[derive(Debug, kfl::Decode)]
// struct Node {
//     #[kfl(property)] a: String,
//     #[kfl(property)] b: String,
//     // #[kfl(child)] c: Child
// }

// #[derive(Debug, kfl::Decode)]
// struct Node {
//     #[kfl(argument)] a: String,
//     #[kfl(argument)] b: String,
//     #[kfl(child)] c: Child
// }

// #[derive(Debug, kfl::Decode)]
// struct Child(
//     #[kfl(argument)] String,
//     #[kfl(argument)] String
// );

fn main() {
    // let doc: Document = kfl::parse("test", &document).unwrap();
    let doc: Vec<Node> = kfl::parse("test", &document).unwrap();
    println!("{:?}", &doc);
}
```
