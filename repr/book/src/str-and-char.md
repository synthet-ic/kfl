# `str` and `char`

- `str` ~ `[u8]` with boundary check
- `char` ~ `u32` ~ `[u8, u8, u8, u8]`

Encoding/decoding (UTF-8, UTF-16), for the most time, do not matter. What matters is the fact that `str` and `char` is not container/collection and its element type. The two are both slices, and `str` is a flattened one of a sequence of `char`s. `u8` mode is fixation of n to 0 as in `[u8][n]`, when, in `char` mode, n varies from 1 to 4.
