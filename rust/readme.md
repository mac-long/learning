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
- You can use shadowing to redefine a variable using `let` again, this leaves the variable in an immutable state.
  - The other use of shadowing is changing the type of a variable while keeping the same name.
- a `scalar` type represents a single value, Rust has 4 primary `scalar` types: `Integers`, `Floating-Point Numbers`, `Booleans` and `characters`.
  - An `integer` is a number without a fractional component.
    - You can utilise `u8|i8` throught to `u128|i128` to define the length of the `integer` in a Signed or Unsigned format.
    - `signed|unsigned` refers to whether the number can be negative, in other words whether the number needs to have a sign with it (signed) or whether it will only be positive and can therefore be represented without a sing (unsigned).
  - `floating-point` types are numbers with decimal points, i.e `6.9`.
  - Rust supports basical mathemetical operations for all number types: addition, subtraction, multiplication, division, and remainder.
  - As in most programming languages a Boolean type in rust has two possible values: `true|false`. Booleans are one byte in size.
  - The `char` type is the languages most primitve alphabetic type. Specified with single quotes: `let kitty='😻';`. a `char` is four bytes in size.
  - Compound types can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.
  - A tuple can is a way of grouping together a number of values with a variety of types. `let x: (i32, f64, u7) = (500, 6.4, 1);`.
  - An array is a way to group multiple values with the same type. `let a = [1, 2, 3, 4, 5];`.
