# Rust

A type safe language best used for systems programming, backend api development and cli tools.

## Basic Commands

- `cargo build` - builds the program.
- `cargo run` - builds the program if needed and runs the program.
- `cargo update` - updates all dependencies defined in `Cargo.toml`.
- `cargo add` - followed by an array of package names will install all packages in array.

## Other Useful Information

- `let name = "Mac"` - defining a variable without `mut` means it can't be changed later on in the program.
- `let mut name = "Mac"` - defining a variable with `mut` means it can be changed later on in the program.
- `const DAYS_OF_WEEK: u32 = 7` - consts are immutable by default and cannot be mutable ever.
  - consts can be defined in any scope including the global scope, meaning they are useful for values needed in various functions.
- 
