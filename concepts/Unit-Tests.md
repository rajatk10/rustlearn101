# Unit Testing in Rust

---
## What is unit testing?
Unit tests verify individual functions work correctly. In Rust, tests are written in the same crate using `#[test]` attribute.
_(For Example Refer miniProjects/basicCalculator)_

## Why unit testing?
- Ensures code works as expected
- Catch bugs early
- Document expected behavior
- Make refactoring safer

---
## Different ways and recommendations
Tests can be arranged in same file with #[cfg[test]] or divide into different files. Rust convention is to mark it in tests/folder.
- Use `#[cfg(test)]` attribute to mark test files. Compiles code **only during testing** (not in release builds).
- Use `#[test]` attribute to mark test functions
- Use `#[should_panic]` attribute to test functions that should panic
- Use `#[ignore]` attribute to ignore tests
- Use `#[test_case]` attribute to test functions with multiple test cases


## Directory Structure
While the official [guidance from rust programming](https://doc.rust-lang.org/book/ch11-03-test-organization.html) is to have unit tests in same file as src code.
However, it is recommended to have unit tests in separate files for better organization, especially for bigger projects.

### Type 1 (Beginner)
Refer to the `miniProjects/basicCalculator` directory for the project structure.
- code in `src/` folder
- tests in `tests/` folder. This compiles tests as separate crate. 
- Note to make variables, functions, modules as public so tests can import them
- Refer `miniProjects/basicCalculator/src/lib.rs` for more details

### Type 2
- code and tests in same file. 
- Use `#[cfg(test)]` attribute to mark test files. Compiles code **only during testing** (not in release builds).
- No extra files however src file becomes long.

### Type 3
Refer to the `miniProjects/toDoCli` directory for the project structure.
- code in `src/` folder
- unit tests in `src/todo/tests.rs` (unit tests for todo.rs)
- Here modules are plumbed by default (no need to import)
- integration tests in `tests/test_todo.rs` (Make sure to import modules)
---

## How to run tests
Run `cargo test` in the root directory of the project.
**Examples** : 
- cargo test -p basicCalculator
- cargo test -p basicCalculator test_addition
- cargo test -- --test-threads=1  (Run tests sequentially)

## Assert Functions
Rust provides several assert functions to verify conditions in tests:
- **assert_eq!**(left, right) : Asserts that left and right are equal
- **assert_ne!**(left, right) : Asserts that left and right are not equal
- **assert!**(condition) : Asserts that condition is true
- **assert!**(condition, message) : Asserts that condition is true with a custom message

## Check Coverage 
- Other packages can be installed for detailed reports 
- `cargo install tarpaulin` or `cargo install cargo-llvm-cov`
- `cargo tarpaulin -p basicCalculator` or `cargo cargo-llvm-cov -p basicCalculator`
- While running cargo test, it runs parallely by default. Use `--test-threads=1` to run sequentially
- Types for tests
    - Unit Tests : Tests that test individual functions. Lives inside src folder.
    - Integration Tests : Tests that test multiple functions i.e test public API (/tests folder). Each file is a crate.
    - Doc Tests : Tests that test documentation . Folder /docs

### Example Test files
```
use basicCalculator::calculator;

#[test]
fn test_add() {
let result = calculator::calculate(5, calculator::Operation::Add, 3);
assert_eq!(result, Ok(8));
}

#[test]
fn test_divide_by_zero() {
let result = calculator::calculate(10, calculator::Operation::Divide, 0);
assert!(result.is_err());
}
```