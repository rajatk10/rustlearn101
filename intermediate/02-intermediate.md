# 02 - Intermediate (Rust101 / day1-2026)

This section covers intermediate Rust concepts implemented in the `intermediate/` crate, organized as modules and executed from [intermediate/src/main.rs](cci:7://file:///Users/kumar.rajat/Documents/Repos/l1/Rust101/day1-2026/intermediate/src/main.rs:0:0-0:0).

## How to run

From the repo/workspace root:

- `cargo run -p intermediate`

Or:

- `cd intermediate`
- `cargo run`

Note: `cargo run some/path/file.rs` does not “run that file”. Cargo runs a binary crate and passes the string as a CLI argument.

## Project structure (modules)

[intermediate/src/main.rs](cci:7://file:///Users/kumar.rajat/Documents/Repos/l1/Rust101/day1-2026/intermediate/src/main.rs:0:0-0:0) wires modules like:

- `ref_borrow`
- `structs`
- `enums`
- `collections`
- `error_handling`

Each module exposes a `pub fn demo()` that is called from [main()](cci:1://file:///Users/kumar.rajat/Documents/Repos/l1/Rust101/day1-2026/intermediate/src/main.rs:4:0-14:1).

## Reference vs Borrowing (`ref_borrow`)

Covered:

- Borrowing with `&T` (read-only)
- Mutable borrowing with `&mut T`
- Passing references to functions
- Avoiding moves by borrowing
- String slicing (`&str`) basics (where applicable)

## Structs + impl (`structs`)

Covered:

- Defining structs and creating instances
- Field access and mutation (with `mut`)
- `impl` blocks for methods
- Methods vs associated functions
- Associated function example: [Rectangle::square(size)](cci:1://file:///Users/kumar.rajat/Documents/Repos/l1/Rust101/day1-2026/intermediate/src/learn_structs.rs:11:4-17:5) returning a new struct
- Calling methods on values ([rect.area()](cci:1://file:///Users/kumar.rajat/Documents/Repos/l1/Rust101/day1-2026/intermediate/src/learn_structs.rs:7:4-9:5))

## Enums + pattern matching (`enums`)

Covered:

- Enums with data in variants (tuple-like and struct-like variants)
- Destructuring with:
    - `match` (exhaustive)
    - `if let` (single-variant focus)
- Why `Debug` printing does not count as “using fields” for dead-code analysis
- Adding methods with `impl` for your enum / types
- Modeling: `struct Employee { ..., role: Role }` where `Role` is an enum
- `match &self.role` to avoid moving out of `self`

## Collections (`collections`)

Covered:

### Vectors (`Vec<T>`)
- Creating vectors with `vec![...]`
- `push`, `pop` (LIFO)
- `insert(index, value)`, `remove(index)`
- Printing vectors using `{:?}`

### HashMap (`HashMap<K, V>`)
- Creating with `HashMap::new()`
- `insert`, `get`, `remove`
- Handling missing keys safely:
    - `match h.get(key) { Some(v) => ..., None => ... }`
- Ownership note:
    - Iterating `for (k, v) in h1` consumes the map
    - Iterate with `for (k, v) in &h1` to keep using it
- Conditional insert using `entry(key).or_insert(value)`

### Strings (`String`)
- `String::from(...)`, cloning vs moving
- References (`&String`) to avoid moves
- Iterating characters via `.chars()`

## Error Handling (`error_handling`)

Covered:

- Rust does not use exceptions for normal control flow
- Core enums used for error handling:
    - `Option<T>`: value may be present/absent
    - `Result<T, E>`: success/failure with error info
- Handling `Result` without panicking:
    - `match Ok(v) / Err(e)` for `parse()` demos
- File read error handling (beginner-friendly):
    - `fs::read_to_string(path)` returns `Result<String, std::io::Error>`
    - Handling with `match` and printing error
- Relative path resolution:
    - `"input.txt"` is resolved relative to the program’s current working directory (`std::env::current_dir()`), often the repo/workspace root when running via Cargo

## What to do next

Suggested next steps after this intermediate milestone:

- Learn `?` operator for propagating `Result` errors cleanly
- Return `Result` from functions instead of printing everywhere
- Introduce custom error types (start simple: `Result<T, String>`, then `Box<dyn std::error::Error>`)
- Practice more `match` expressions that return values (not just printing)