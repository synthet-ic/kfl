# KFL

- Nominal Typing
- Trait-Based

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

## KDL

- KDL as algebraic data type with exponential
- KDL as token tree (≠ AST, ≠ token stream)

## TODO

- [ ] Use previous version of KFL itself when testing grammar, instead of serde/serde_json
- [ ] `-`
- [ ] Implement `DecodeScalar` for struct as the replacement of `flatten(properties)` and support `flatten(arguments)` equivalent as well
- [ ] Span
- [ ] Detect name conflicts between fields in the same struct
- [ ] Understand error categories, reconsider `Context` in the presence of `DecodePartial`
- [ ] Test organisation, provide utility macros
- [ ] `Encode`
  - [ ] `EncodePartial` as an analogous to `skip_serializing_none`
- [ ] Compare TokenStream with Scalar, TokenTree with Node
- [ ] property enum or union
