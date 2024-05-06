# Command Line Interpretor Parser (and validator)
Rust parsing library dealing with iterators.

The library itself declares a parsing trait and some associated structures and is useless without the derive library. The purpose of this library is to parse arguments into a struct or enum by just deriving with `TryParse`.

It has a similar purpose as [clap](https://github.com/clap-rs/clap) but tries to simplify derive usages and the overall library. However for a most advanced usage, clap would probably be more complete.


