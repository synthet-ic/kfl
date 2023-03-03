**TODO**

- Can `ignore` be represented by Map?
- Can flags, infos and assertions be represented by Map?
- Derivatives and lookaheads

**`Map`**

- Capturing/grouping

**`Zero`**

- `Zero` acts as both an additive and multiplicative unit?

**`Div`** (quotients)

- Case insensitivity

**Comparisons**

| name | regex | op | kfl (exp.) |
| - | - | - | - |
| concat | `ab` | `a & b` | `and { a b }` |
| alternation | `a\|b` | `a \| b` | `or { a b }` |
| kleen star | `a*` | `[a]` | `star { a }`
| optional | `a?` | `a?` | `opt { a }` |
| repetition | `a{n,m}` | `a * (n..m)` | `rep n m { a }` |
| class | `[a-z]` | `'a'..'z'` | `range a z` |
| negation | `[^a-z]` | `not { range { a z } }` |
