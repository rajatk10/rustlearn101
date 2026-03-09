# Rust Debugging Guide (LLDB)

When debugging Rust on macOS or Linux (LLVM-based), the default debugger is `lldb`. Rust provides a wrapper called `rust-lldb` that nicely formats Rust data types (like Enums, Vecs, and Options).
The two main supproted debuggers are LLDB and GDB.
`Writing a debugger from scratch for a language requires a lot of work, especially if debuggers have to be supported on various platforms. GDB and LLDB, however, can be extended to support debugging a language`

## 1. Preparing for Debugging
- Always debug using an unoptimized "debug" build. This is Cargo's default.
```bash
cargo build
```
- The compiled binary is in `target/debug/` folder. Use lldb to start debugger
```bash
rust-lldb target/debug/your_program 
```
- Once inside lldb , use below command line to set breakpoint
```bash
breakpoint set -f <file> -l <line>
```
- Other most used functions (mostly similar to pdb)
    - `run` : Run the program
    - `next` or n : Step over the current line to the next one
    - `step` or s : Step into the function called on the current line
    - `continue` or c : Resume execution until the next breakpoint or exit
    - `p <variable>` : Print variable value
    - `bt` : Print backtrace
    - `frame variable` <var> or print <var> : Print variable value
    - `thread backtrace` : Print backtrace
    - `expression <expr>`
    - `quit` : Exit debugger

## 2. More info on debugging
- [LLDB Tutorial](https://lldb.llvm.org/use/tutorial.html)
- [Deubgging in rust](https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html)