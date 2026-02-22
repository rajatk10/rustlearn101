# Basic Calculator - Rust Mini Project

A beginner-friendly command-line calculator built in Rust, demonstrating fundamental concepts and best practices.

---

## Project Overview

The Basic Calculator CLI - is a simple interactive calculator that performs arithmetic operations on two integers. It serves as a beginner learning project.
- **Operations supported**: Addition, Subtraction, Multiplication, Division, Modulus
- Inputs - Only 2 integers and an operator
- Error Handling - division by zero and loop for invalid inputs.
- Testing - Comprehensive unit tests with coverage reporting

---

## Run Project & Tests
- cargo run
- cargo test
- cargo tarpaulin -p basicCalculator (check coverage)

## Project Structure
basicCalculator/
├── src/
│   ├── main.rs
│   ├── calculator.rs
│   └── lib.rs
├── tests/
│   └── test_calculator.rs
├── Cargo.toml
└── README.md
