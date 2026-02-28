# TO-DO CLI - Rust Mini Project

A beginner-friendly command-line to do manager built in Rust, demonstrating fundamental concepts and best practices.

---

## Project Overview

The To-Do CLI - is a simple interactive To-Do list manager that allows users to add, list, complete, and remove tasks. It serves as a beginner learning project. 
Stores the tasks as ID (monotonic counter), offers persistence using JSON file.

---

## Run Project & Tests
Run these commands from root directory
- cargo run
- cargo test
- cargo tarpaulin (check coverage)

## Project Structure
```todoCli/
├── src/
│   ├── main.rs
│   ├── todo.rs
│   └── todo
│       └── tests.rs (unit test)
├── tests/
│   └── test_calculator.rs (integration tests)
├── Cargo.toml
└── README.md
```
