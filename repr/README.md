| name | regex | op | kfl |
| - | - | - | - |
| concat | `ab` | `a & b` | `and { a b }` |
| alternation | `a\|b` | `a \| b` | `or { a b }` |
| kleen star | `a*` | `[a]` | `star { a }`
| optional | `a?` | `a?` | `opt { a }` |
| repetition | `a{n,m}` | `a * (n..m)` | `rep n m { a }` |
| class | `[a-z]` | `'a'..'z'` | `range a z` |
| negation | `[^a-z]` | `not { range { a z } }` |

**TODO**

- BitXor for classes
- Fn for Pat to map
- Mul and Div (quotient) for Pat
- BitXor
- ignore
- star
