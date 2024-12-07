# xxh3_hashid_macro

This crate provides a macro that allows XXH3 "hashed identifiers" to be created at compile time. Mostly useful for games, but you may find another use for it.

## Example

```rust
use xxh3_hashid_macro::hash;
let id = hash!("example_id");
println!("0x{:0>16x}", id); // 0xf826a9bb47f7ff34
```
