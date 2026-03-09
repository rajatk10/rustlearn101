# rustlearn101

Hands-on Rust learning workspace with progressive practice from fundamentals to practical CLI projects.

## Repository Overview

This repository is organized as a learning path:

- `basics/`: foundational Rust syntax and language primitives
- `intermediate/`: modular demos for core intermediate Rust concepts
- `concepts/`: focused notes and references (debugging, testing, memory, etc.)
- `miniProjects/`: applied projects that combine multiple concepts

---

## Folder Summaries

### `basics/`
Beginner-friendly practice covering:
- variables and mutability
- primitive and compound data types,
- collections - vectors and hash maps
- control flow (`if`, `loop`, `while`, `for`)

### `intermediate/`
Module-based demos for:
- references and borrowing
- structs and enums
- error handling with `Result` and `Option`
- generics and traits

### `concepts/`
Theory + reference notes including:
- `Unit-Tests.md`
- `Debugging.md`
- `MEMORY_MANAGEMENT.md`
- `RUST_CONCEPTS.md`

### `miniProjects/`
Current mini-projects:
- `basicCalculator/`: arithmetic CLI with validation, error handling, and tests
- `toDoCli/`: todo manager with JSON persistence, unit and integration tests

---

## Note on Symlinks

If you use symlinks to organize shortcuts to this repo or its subfolders, keep this repository as the source of truth and run Cargo commands from the actual crate directory containing `Cargo.toml`.

---

## Running the Code

From repository root (workspace crates):

```bash
cargo run -p basics
cargo run -p intermediate
```

For mini-projects (standalone crates), run from each project directory:

```bash
cargo run
cargo test
```

---