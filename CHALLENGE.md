# CommandPos Challenge

Your task is to define a struct named `CommandPos` that holds metadata
about the location of a command in a log file. Then, implement the `From` trait
to allow constructing a `CommandPos` from a tuple: (file ID, byte range).

Requirements:
- Define a struct `CommandPos` with the following public fields:
- `file: u64` — file identifier
- `pos: u64` — starting byte offset
- `len: u64` — length in bytes

- Implement `From<(u64, Range<u64>)>` for `CommandPos`, so you can convert a tuple like `(3, 100..150)` into a `CommandPos { file: 3, pos: 100, len: 50 }`

use std::ops::Range;

TODO: Define the CommandPos struct here

TODO: Implement From<(u64, Range<u64>)> for CommandPos here

