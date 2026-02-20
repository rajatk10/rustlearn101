# Rust Basics — What is Covered

This folder/project contains beginner-friendly Rust examples covering core syntax and the mental model needed to move toward real programs.

## 1) Variables and Mutability
- `let` creates an immutable variable by default
- `let mut` enables mutation
- Reassignment rules and compiler errors for immutable bindings

## 2) Strings
- `String` vs `&str`
- `String` is owned, heap-allocated, growable (can `push_str`, `push`)
- `&str` is a borrowed string slice (often points to a string literal / read-only memory)
- Rebinding an `&str` variable vs modifying actual string contents

## 3) Data Types Overview
### Primitive types
- Integers (signed/unsigned), floats, `bool`, `char`

### Compound types
- Tuples: packing/unpacking and indexing (`t.0`, `t.1`, etc.)
- Arrays: fixed-size collection, mutation when declared `mut`
- Slices: view into arrays/strings (`&[T]`, `&str`)

### Common std library types (non-primitive)
- `Vec<T>`: growable array (heap-backed)
- `HashMap<K,V>`: key-value store

## 4) Formatting / Printing / Input
- `println!` formatting placeholders:
    - `{}` for display formatting
    - `{:?}` and `{:#?}` for debug formatting (useful for arrays, vectors, maps)
- Read, Parse, Match, Error handling, input from user 

## 5) Collections
### Vector (`Vec<T>`)
- Creating vectors using `vec![...]`
- `push` / `pop`
- `len` and `capacity` ideas (where applicable)

### HashMap (`HashMap<K,V>`)
- Creating with `HashMap::new()`
- `insert`
- `get` returns `Option`
- Iterating using `iter()`

## 6) If / Else
- Basic conditional branching
- Example usage for odd/even logic

## 7) Loops / Control Flow
- `loop {}` (infinite loop + `break`)
- `while condition {}`
- `for i in start..end` and `for i in start..=end`

## 8) Stack vs Heap (Conceptual + Observations)
- Stack: local variables / fixed-size values (fast allocation via stack frame)
- Heap: dynamic/growable data (`String`, `Vec`, `HashMap`)
- Ownership move vs clone for heap data (`String` move vs `clone`)
- Printing pointers/addresses:
    - variable address via `&var` (stack location)
    - buffer address via `as_ptr()` (heap buffer or literal bytes)
    - debug view of `&str` shows fat pointer (addr + len metadata)

## Next Recommended Topics
- Ownership and borrowing in functions (`&T`, `&mut T`)
- `Option` / `Result` + `match`
- Modules (`mod`, `pub`, splitting code into files)
- Iterators (`iter`, `iter_mut`, `into_iter`)
- Structs and `impl`