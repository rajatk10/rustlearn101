
# Rust Concepts: Beginner to Intermediate

A structured guide to learning Rust concepts from absolute basics to intermediate proficiency.

---

## Phase 1: Beginner Foundations

### 1.1 Getting Started

#### Installation & Setup
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Create a new project
cargo new my_project
cd my_project
cargo run
```

#### Cargo Basics
- `cargo new` - Create new project
- `cargo build` - Compile project
- `cargo run` - Compile and run
- `cargo test` - Run tests
- `cargo check` - Check code without compiling
- `cargo clippy` - Linting tool
- `cargo fmt` - Code formatter

---

### 1.2 Variables and Data Types

#### Variables and Mutability
```rust
// Immutable by default
let x = 5;
// x = 6; // Error! Cannot mutate immutable variable

// Mutable variable
let mut y = 5;
y = 6; // OK

// Constants (always immutable, type required)
const MAX_POINTS: u32 = 100_000;

// Shadowing (creating new variable with same name)
let z = 5;
let z = z + 1;  // OK - this is shadowing, not mutation
let z = "six";  // OK - can even change type
```

#### Scalar Types
```rust
// Integers: i8, i16, i32, i64, i128, isize (signed)
//           u8, u16, u32, u64, u128, usize (unsigned)
let signed: i32 = -42;
let unsigned: u32 = 42;

// Floating point
let x: f64 = 2.0;
let y: f32 = 3.0;

// Boolean
let is_active: bool = true;

// Character (Unicode, 4 bytes)
let letter: char = 'A';
let emoji: char = '😊';
```

#### Compound Types
```rust
// Tuples (fixed length, mixed types)
let tuple: (i32, f64, &str) = (500, 6.4, "hello");
let (x, y, z) = tuple;  // Destructuring
let first = tuple.0;     // Access by index

// Arrays (fixed length, same type)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

---

### 1.3 Functions

```rust
// Basic function
fn greet() {
    println!("Hello!");
}

// With parameters
fn add(x: i32, y: i32) -> i32 {
    x + y  // No semicolon = return value (expression)
}

// Explicit return
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;  // With semicolon when using return keyword
}

// Multiple return values using tuple
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// Unit type (returns nothing, like void)
fn print_number(x: i32) {
    println!("{}", x);
}  // Implicitly returns ()
```

---

### 1.4 Control Flow

#### If Expressions
```rust
// if is an expression in Rust
let number = 6;
if number % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}

// if in assignment
let condition = true;
let number = if condition { 5 } else { 6 };

// else if
let num = 7;
if num < 5 {
    println!("less than 5");
} else if num == 5 {
    println!("equals 5");
} else {
    println!("greater than 5");
}
```

#### Loops
```rust
// loop - infinite loop
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // loop can return value
    }
};

// while
let mut number = 3;
while number != 0 {
    println!("{}", number);
    number -= 1;
}

// for loop (most common)
for i in 0..5 {
    println!("{}", i);  // 0, 1, 2, 3, 4
}

for i in 0..=5 {
    println!("{}", i);  // 0, 1, 2, 3, 4, 5 (inclusive)
}

let arr = [10, 20, 30];
for element in arr.iter() {
    println!("{}", element);
}
```

---

### 1.5 Stack vs Heap Memory (FOUNDATIONAL CONCEPT)

Understanding stack and heap is crucial for understanding Rust's ownership system and performance characteristics.

#### What are Stack and Heap?

**Stack**:
- Fast, automatic memory management
- LIFO (Last In, First Out) structure
- Fixed-size data only
- Automatically cleaned up when function returns

**Heap**:
- Slower, manual memory management (handled by Rust's ownership)
- Dynamic size, can grow/shrink
- Data persists until explicitly freed
- Accessed via pointers

#### Visual Representation

```rust
fn main() {
    // Stack allocation
    let x = 5;           // Integer stored directly on stack
    let y = true;        // Boolean stored on stack
    let z = 3.14;        // Float stored on stack

    // Heap allocation
    let s = String::from("hello");  // String data on heap, pointer on stack
    let v = vec![1, 2, 3];          // Vector data on heap, pointer on stack

    // Memory layout:
    //
    // STACK                           HEAP
    // ┌─────────────────┐            ┌──────────────┐
    // │ x: 5            │            │              │
    // │ y: true         │            │              │
    // │ z: 3.14         │            │              │
    // │                 │            │              │
    // │ s (pointer) ────┼───────────→│ "hello"      │
    // │   len: 5        │            │              │
    // │   capacity: 5   │            │              │
    // │                 │            │              │
    // │ v (pointer) ────┼───────────→│ [1, 2, 3]    │
    // │   len: 3        │            │              │
    // │   capacity: 3   │            │              │
    // └─────────────────┘            └──────────────┘
}  // Stack frame cleaned up here
   // Heap memory freed via ownership rules
```

#### Stack vs Heap: Detailed Comparison

| Aspect | Stack | Heap |
|--------|-------|------|
| **Speed** | ⚡ Very fast (just move stack pointer) | 🐢 Slower (must find space, manage fragmentation) |
| **Size** | Fixed at compile time | Dynamic, can grow/shrink at runtime |
| **Allocation** | Automatic (push/pop) | Manual request (Rust handles via ownership) |
| **Access** | Direct | Indirect (via pointer, one extra step) |
| **Lifetime** | Function scope only | Until explicitly freed (or owner drops) |
| **Typical Size** | Small (few MB, varies by OS) | Large (GBs available) |
| **Data Structure** | LIFO (Last In First Out) | Unordered, fragmented |
| **Cache Friendly** | ✅ Very (data locality) | ❌ Less (scattered in memory) |
| **Thread Safety** | ✅ Each thread has own stack | ⚠️ Shared, needs synchronization |
| **Overflow Risk** | Stack overflow if too deep | Out of memory if too much allocated |

#### What Goes on the Stack?

```rust
// All these are STACK allocated:
let num: i32 = 42;                    // Integer (4 bytes)
let flag: bool = true;                // Boolean (1 byte)
let letter: char = 'A';               // Character (4 bytes, Unicode)
let tuple: (i32, f64) = (5, 3.14);   // Tuple (12 bytes)
let array: [i32; 5] = [1, 2, 3, 4, 5]; // Array (20 bytes)

// Stack-only structs (all fields are stack types)
struct Point {
    x: i32,
    y: i32,
}
let point = Point { x: 10, y: 20 };   // 8 bytes on stack

// References/pointers (just addresses)
let r: &i32 = &num;                   // 8 bytes (64-bit pointer)
```

#### What Goes on the Heap?

```rust
// All these use HEAP allocation:
let s: String = String::from("hello");           // String content on heap
let v: Vec<i32> = vec![1, 2, 3];                // Vector content on heap
let b: Box<i32> = Box::new(5);                   // Boxed value on heap
let map: HashMap<String, i32> = HashMap::new();  // HashMap data on heap

// Structs with heap fields
struct Person {
    name: String,    // Heap (String content)
    age: i32,        // Stack (integer)
}

let person = Person {
    name: String::from("Alice"),
    age: 30,
};

// Memory layout:
// Stack: person { name: [ptr, len, cap], age: 30 }
// Heap:  "Alice"
```

#### Performance Implications

```rust
// Stack allocation - FAST ⚡
fn stack_example() {
    let x = 5;           // Instant: just move stack pointer
    let y = x;           // Instant: copy 4 bytes
    let arr = [0; 100]; // Instant: 400 bytes on stack
}  // Instant cleanup: just move stack pointer back

// Heap allocation - SLOWER 🐢
fn heap_example() {
    let s = String::from("hello");  // Must find space on heap
    let s2 = s.clone();              // Must allocate + copy heap data
    let v = vec![0; 100];            // Must allocate heap space
}  // Must free heap allocations (Rust does this automatically)

// Benchmark comparison (approximate):
// Stack allocation:  ~1 nanosecond
// Heap allocation:   ~50-100 nanoseconds
// Stack is 50-100x faster!
```

#### Why Stack is Faster: The Details

1. **Allocation**:
   - Stack: Move pointer down by N bytes (1 instruction)
   - Heap: Search for free block, update metadata, fragmentation handling (many instructions)

2. **Access**:
   - Stack: Direct access, data nearby in memory
   - Heap: Follow pointer → find data (cache miss more likely)

3. **Deallocation**:
   - Stack: Move pointer back up (1 instruction)
   - Heap: Mark block as free, possibly merge with neighbors (many instructions)

4. **Cache locality**:
   - Stack: Data stored together, CPU cache friendly
   - Heap: Data scattered, more cache misses

```rust
// Example: Cache-friendly stack code
fn sum_array_stack() -> i32 {
    let arr = [1, 2, 3, 4, 5];  // All data adjacent in memory
    arr.iter().sum()             // CPU can cache entire array
}

// vs. Heap with scattered data
fn sum_vector_heap() -> i32 {
    let v = vec![1, 2, 3, 4, 5];  // Data on heap, might not be contiguous
    v.iter().sum()                 // Potential cache misses
}
```

#### Stack Overflow

```rust
// This will cause stack overflow:
fn recursive_forever() {
    let huge_array = [0; 1_000_000];  // 4MB on stack
    recursive_forever();               // Stack grows with each call
}  // Eventually: stack overflow!

// Solution: use heap
fn recursive_safe() {
    let huge_vec = vec![0; 1_000_000];  // 4MB on heap
    recursive_safe();                    // Stack only grows by pointer size
}  // Will run out of heap memory, but won't crash stack
```

#### When to Use Each

| Use Stack When | Use Heap When |
|----------------|---------------|
| Size known at compile time | Size unknown or varies at runtime |
| Small data (< few KB) | Large data (MB+) |
| Short-lived (function scope) | Needs to outlive function |
| Performance critical | Flexibility more important |
| No sharing between threads | Sharing data between threads |

```rust
// Good stack usage:
fn calculate(x: i32, y: i32) -> i32 {
    let temp = x + y;  // Small, short-lived → stack
    temp * 2
}

// Good heap usage:
fn read_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path).unwrap();
    contents  // Size unknown, potentially large, returned → heap
}

// Good heap usage: sharing
fn create_shared_data() -> Vec<i32> {
    let data = vec![1, 2, 3, 4, 5];
    data  // Returned, outlives function → heap
}
```

#### Rust's Approach vs Other Languages

| Language | Memory Management |
|----------|-------------------|
| **C** | Manual: `malloc()`/`free()` - you track everything |
| **C++** | Manual + RAII (smart pointers) - mostly manual |
| **Java/C#/Go** | Garbage Collector - runtime tracking/cleanup |
| **Python/JS** | Garbage Collector + reference counting |
| **Rust** | Ownership system - compile-time tracking, no runtime cost |

```rust
// In C (manual, error-prone):
// char* s = malloc(100);
// // use s...
// free(s);  // Must remember to free!

// In Java/Go (GC, runtime cost):
// String s = "hello";
// // use s...
// // GC will clean up eventually (pause your program)

// In Rust (ownership, zero runtime cost):
{
    let s = String::from("hello");
    // use s...
}  // Dropped automatically here, no GC needed!
```

#### Box: Explicit Heap Allocation

```rust
// Force something onto the heap with Box
let x = 5;           // Stack
let y = Box::new(5); // Heap

// Why use Box?
// 1. Large data
struct HugeStruct {
    data: [i32; 1_000_000],
}
let big = Box::new(HugeStruct { data: [0; 1_000_000] });
// Avoids stack overflow

// 2. Trait objects (dynamic dispatch)
trait Animal {
    fn speak(&self);
}
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

// 3. Recursive types
enum List {
    Cons(i32, Box<List>),  // Box breaks infinite size
    Nil,
}
```

#### Key Takeaways

1. **Stack is default in Rust** - use it whenever possible for performance
2. **Heap is for flexibility** - use when you need dynamic size or longer lifetime
3. **Rust makes heap safe** - ownership prevents memory leaks and double-frees
4. **Performance difference is real** - stack is ~50-100x faster
5. **Copy vs Move is related** - stack data can be Copy, heap data usually moves

```rust
// Mental model for Rust:
//
// If data is stack-only → cheap to copy → implements Copy
// If data uses heap → expensive to copy → moves ownership
//
// This is why:
let x = 5;               // Stack → Copy
let y = x;               // x still valid
let s = String::from("hello");  // Heap → Move
let s2 = s;              // s now invalid
```

#### Memory Safety Without GC

```rust
// Rust's ownership achieves memory safety at compile time:

// Problem in C:
// int* ptr = malloc(sizeof(int));
// free(ptr);
// *ptr = 5;  // Use after free! 💥

// Rust prevents this:
{
    let s = String::from("hello");
}  // s dropped here
// println!("{}", s);  // Compile error! ✅

// Problem in C:
// int* ptr = malloc(sizeof(int));
// // forget to free
// // Memory leak! 💧

// Rust prevents this:
{
    let s = String::from("hello");
}  // Automatically freed, can't forget! ✅

// All checked at compile time - ZERO runtime cost!
```

---

### 1.6 Ownership (CRITICAL CONCEPT)

#### The Three Rules
1. Each value in Rust has a variable that's called its **owner**
2. There can only be **one owner** at a time
3. When the owner goes out of scope, the value is **dropped**

```rust
// Simple ownership
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved to s2
// println!("{}", s1);  // Error! s1 no longer valid

// Clone for deep copy
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // OK - both valid

// Copy trait (stack-only data)
let x = 5;
let y = x;  // Copy, not move
println!("{} {}", x, y);  // OK - integers implement Copy
```

#### Function Ownership
```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // s is moved
    // println!("{}", s);  // Error!

    let x = 5;
    makes_copy(x);  // x is copied
    println!("{}", x);  // OK - integers are Copy
}

fn takes_ownership(s: String) {
    println!("{}", s);
}  // s is dropped here

fn makes_copy(x: i32) {
    println!("{}", x);
}
```

#### Understanding Copy vs Move (IMPORTANT!)

**Copy Types** (automatically copied, no ownership transfer):
- All integer types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, etc.
- Floating point: `f32`, `f64`
- Boolean: `bool`
- Character: `char`
- Tuples containing only Copy types: `(i32, i32)`, `(bool, f64)`
- Arrays of Copy types: `[i32; 5]`

**Move Types** (ownership transfers on assignment):
- `String` - heap-allocated, growable string
- `Vec<T>` - heap-allocated, growable array
- `Box<T>` - heap-allocated value
- Structs and enums (unless they explicitly implement Copy)

```rust
// COPY BEHAVIOR (stack data)
let x = 5;
let y = x;  // x is COPIED to y (both are independent)
println!("x: {}, y: {}", x, y);  // Both valid!
// Why? Integers are small, stored on stack, cheap to copy

// MOVE BEHAVIOR (heap data)
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED to s2 (ownership transferred)
// println!("{}", s1);  // ERROR! s1 is no longer valid
println!("{}", s2);  // Only s2 is valid now
// Why? String data is on heap, expensive to copy, so Rust moves ownership

// If you want both to be valid, explicitly clone
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy of heap data
println!("s1: {}, s2: {}", s1, s2);  // Both valid!
```

#### Quick Reference Table: Copy vs Move by Data Type

| Type | Implements Copy? | Behavior on Assignment | Stored | Example |
|------|-----------------|------------------------|--------|---------|
| **Integers** (`i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `isize`, `usize`) | ✅ Yes | **Copied** | Stack | `let y = x;` // x still valid |
| **Floats** (`f32`, `f64`) | ✅ Yes | **Copied** | Stack | `let y = x;` // x still valid |
| **Boolean** (`bool`) | ✅ Yes | **Copied** | Stack | `let y = x;` // x still valid |
| **Character** (`char`) | ✅ Yes | **Copied** | Stack | `let y = x;` // x still valid |
| **String** (`String`) | ❌ No | **Moved** | Heap | `let s2 = s1;` // s1 invalid |
| **String slice** (`&str`) | ✅ Yes* | **Reference copied** | Stack (pointer) | `let s2 = s1;` // both valid |
| **Array** (`[T; N]`) | ✅ If T is Copy | **Copied if T is Copy** | Stack | `let y = x;` // depends on T |
| **Tuple** (`(T, U, ...)`) | ✅ If all types are Copy | **Copied if all Copy** | Stack | `let y = x;` // depends on T, U |
| **Vector** (`Vec<T>`) | ❌ No | **Moved** | Heap | `let v2 = v1;` // v1 invalid |
| **HashMap** (`HashMap<K, V>`) | ❌ No | **Moved** | Heap | `let m2 = m1;` // m1 invalid |
| **Box** (`Box<T>`) | ❌ No | **Moved** | Heap | `let b2 = b1;` // b1 invalid |
| **Struct** (custom) | ❌ No (by default) | **Moved** | Stack/Heap | `let s2 = s1;` // s1 invalid |
| **Enum** (custom) | ❌ No (by default) | **Moved** | Stack/Heap | `let e2 = e1;` // e1 invalid |
| **References** (`&T`, `&mut T`) | ✅ Yes (for `&T`)** | **Reference copied** | Stack (pointer) | `let r2 = r1;` // both valid |
| **Slice** (`&[T]`) | ✅ Yes* | **Reference copied** | Stack (ptr + len) | `let s2 = s1;` // both valid |

**Notes:**
- \* References and slices copy the **pointer**, not the data they point to
- \*\* `&T` (immutable ref) is Copy, but `&mut T` (mutable ref) is NOT Copy (prevents multiple mutable references)
- Custom structs/enums can implement Copy by adding `#[derive(Copy, Clone)]`, but only if all fields are Copy

#### Comparison with Go (for Go developers)

| Concept | Go | Rust |
|---------|----|----- |
| **Integers** | Copied | ✅ Copied (implements Copy) |
| **Structs** | Copied by value | ❌ **Moved** by default (unless you derive Copy) |
| **Slices** | Copied by reference (header only) | ❌ **Moved** (`Vec<T>` owns data, moves ownership) |
| **Slice references** | Pass around freely | ✅ `&[T]` copied (just a reference) |
| **Strings** | Copied by reference | ❌ `String` moved; `&str` copied (reference) |
| **Pointers** | Can be copied | References follow borrow rules |
| **Garbage Collection** | GC manages memory | No GC - ownership tracks lifetime |

**Key Difference**: In Go, structs are copied but slices/maps are "reference types" (copy the header). In Rust, there's no such distinction - types either implement `Copy` (cheap to copy) or don't (ownership moves).

#### Rule of Thumb for Rust

```rust
// Simple rule to remember:
// - If ALL data is on the STACK → likely Copy
// - If ANY data is on the HEAP → likely Move

// Stack only = Copy
let x: i32 = 5;        // 4 bytes on stack → Copy
let y: (i32, bool) = (5, true);  // All stack → Copy

// Heap data = Move
let s: String = String::from("hello");  // Pointer on stack, data on heap → Move
let v: Vec<i32> = vec![1, 2, 3];       // Pointer on stack, data on heap → Move

// References = Copy (just the pointer)
let r: &str = "hello";    // Pointer to data → Copy the pointer
let slice: &[i32] = &[1, 2, 3];  // Pointer + length → Copy
```

#### Making Custom Types Copy

```rust
// This struct can be Copy (all fields are Copy)
#[derive(Copy, Clone)]  // Must also implement Clone
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 5, y: 10 };
let p2 = p1;  // Copied!
println!("p1: ({}, {})", p1.x, p1.y);  // p1 still valid

// This struct CANNOT be Copy (String is not Copy)
struct Person {
    name: String,  // String is heap-allocated
    age: i32,
}

let person1 = Person {
    name: String::from("Alice"),
    age: 30,
};
let person2 = person1;  // Moved!
// println!("{}", person1.name);  // Error! person1 is moved

// To keep person1 valid, you must clone
#[derive(Clone)]
struct Person {
    name: String,
    age: i32,
}

let person1 = Person {
    name: String::from("Alice"),
    age: 30,
};
let person2 = person1.clone();  // Deep copy
println!("{}", person1.name);  // OK! Both valid
```

#### Common Pitfalls for Go Developers

```rust
// Go: Slices are "reference types" (cheap to copy)
// Rust: Vec<T> owns its data (expensive, so it moves)

// Go-style thinking (WRONG in Rust):
let v1 = vec![1, 2, 3];
let v2 = v1;  // Expecting v1 to still work...
// println!("{:?}", v1);  // ERROR! v1 was moved

// Rust way: either borrow or clone
let v1 = vec![1, 2, 3];

// Option 1: Borrow (if you just need to read)
let v2 = &v1;  // v1 still valid
println!("{:?}", v1);

// Option 2: Clone (if you need independent copy)
let v2 = v1.clone();  // Both valid
println!("{:?} {:?}", v1, v2);

// Option 3: Use slices (like Go slices, but explicit)
let v1 = vec![1, 2, 3];
let slice: &[i32] = &v1;  // Slice reference (cheap, like Go)
do_something(slice);  // v1 still valid
```

#### Memory Safety Guarantees

```rust
// Why Rust is strict about Copy vs Move:

// Problem in C/C++:
// char* str1 = malloc(...);
// char* str2 = str1;  // Both point to same memory
// free(str1);         // Free memory
// free(str2);         // Double-free! 💥 Crash

// Rust prevents this at compile time:
let s1 = String::from("hello");
let s2 = s1;  // s1 MOVED to s2
// Only s2 can free the memory
// s1 is invalid, can't be used
// No double-free possible! ✅

// For Copy types (no heap allocation):
let x = 5;
let y = x;  // Simple bit copy, no pointers involved
// Both valid, no memory safety issues
```

#### String vs &str (CRITICAL DISTINCTION)

```rust
// String - owned, mutable, heap-allocated
let mut s = String::from("hello");
s.push_str(" world");  // Can modify!
println!("{}", s);  // "hello world"

// String does NOT implement Copy
let s1 = String::from("hello");
let s2 = s1;  // MOVED, not copied
// s1 is now invalid!

// &str - borrowed, immutable string slice, reference to string data
let s: &str = "hello";  // String literal (stored in binary)
// s.push_str(" world");  // ERROR! &str is immutable

let string = String::from("hello world");
let slice: &str = &string[0..5];  // Slice of String
let slice2: &str = &string;  // Reference to entire String as &str

// &str does NOT own the data, just points to it
// String owns its data on the heap
```

#### When to Use String vs &str

```rust
// Use String when:
// - You need to own the string data
// - You need to modify the string
// - Building strings at runtime

fn create_greeting(name: String) -> String {
    let mut greeting = String::from("Hello, ");
    greeting.push_str(&name);
    greeting.push('!');
    greeting
}

// Use &str when:
// - You only need to read the string
// - You want to reference existing string data
// - Writing flexible functions

fn print_greeting(name: &str) {  // Accepts both &str and &String
    println!("Hello, {}!", name);
}

fn main() {
    let string = String::from("Alice");
    let str_slice = "Bob";

    print_greeting(&string);  // String can be borrowed as &str
    print_greeting(str_slice);  // &str works directly
}
```

#### Complete Ownership Example

```rust
fn main() {
    // Copy types (integers)
    let x = 5;
    let y = x;  // Copied
    println!("x: {}, y: {}", x, y);  // Both valid

    // Move types (String)
    let s1 = String::from("hello");
    let s2 = s1;  // Moved
    // println!("{}", s1);  // Error: s1 moved
    println!("{}", s2);  // OK: s2 owns the data

    // Clone to have two owners
    let s3 = String::from("world");
    let s4 = s3.clone();  // Deep copy
    println!("{} {}", s3, s4);  // Both valid

    // String references (borrowing)
    let s5 = String::from("rust");
    let s6 = &s5;  // Borrow, don't move
    println!("{} {}", s5, s6);  // Both valid! s5 still owns, s6 borrows

    // String slice (&str)
    let s7: &str = "literal";  // Points to binary data
    let s8 = s7;  // Copy the reference (not the data)
    println!("{} {}", s7, s8);  // Both valid! Just copying a reference
}
```

#### Memory Layout Visualization

```rust
// Stack vs Heap
let x = 5;
// Stack: [x: 5]

let s = String::from("hello");
// Stack: [ptr, len, capacity] -> Heap: ['h','e','l','l','o']
// ptr points to heap memory
// len = 5 (current length)
// capacity = 5 (allocated space)

let y = x;
// Stack: [x: 5, y: 5]  // Simple copy

let s2 = s;
// Stack: [s: INVALID, s2: [ptr, len, cap]] -> Heap: ['h','e','l','l','o']
// Ownership moved! Only one owner allowed
// s is no longer valid to prevent double-free

let s3 = s2.clone();
// Stack: [s2: [ptr1, len, cap], s3: [ptr2, len, cap]]
// Heap: ['h','e','l','l','o'] at ptr1
//       ['h','e','l','l','o'] at ptr2
// Two separate copies on heap
```

#### Why Rust Does This

```rust
// In languages with garbage collection (JavaScript, Python, Java):
let s1 = "hello";
let s2 = s1;  // Both reference same data
// GC tracks and cleans up when no references remain

// In Rust (no GC):
let s1 = String::from("hello");
let s2 = s1;  // Ownership moved
// Only one owner, so Rust knows exactly when to free memory
// No runtime overhead, no GC pauses
// Prevents: double-free, use-after-free, memory leaks
```

---

### 1.6 References and Borrowing

```rust
// Immutable reference (borrowing)
let s1 = String::from("hello");
let len = calculate_length(&s1);  // Borrow, don't take ownership
println!("'{}' has length {}", s1, len);  // s1 still valid

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the data

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}

// Borrowing Rules:
// 1. At any time, you can have EITHER:
//    - One mutable reference, OR
//    - Any number of immutable references
// 2. References must always be valid

let mut s = String::from("hello");
let r1 = &s;      // OK
let r2 = &s;      // OK
// let r3 = &mut s;  // Error! Cannot have mutable reference while immutable exist
println!("{} {}", r1, r2);

let r3 = &mut s;  // OK - r1 and r2 no longer used after this point
```

#### Slices
```rust
// String slices
let s = String::from("hello world");
let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"

// Shorthand
let hello = &s[..5];    // from start
let world = &s[6..];    // to end
let entire = &s[..];    // entire string

// Array slices
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];  // [2, 3]

// String literals are slices
let s: &str = "Hello, world!";  // &str is a string slice
```

---

## Phase 2: Beginner to Intermediate

### 2.1 Structs

```rust
// Classic struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Creating instance
let mut user1 = User {
    email: String::from("user@example.com"),
    username: String::from("someuser"),
    active: true,
    sign_in_count: 1,
};

// Access fields
user1.email = String::from("newemail@example.com");

// Struct update syntax
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // Use remaining fields from user1
};

// Tuple structs
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);

// Unit structs (no fields)
struct AlwaysEqual;
let subject = AlwaysEqual;
```

#### Methods
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function (no self, like static method)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Usage
let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());
let sq = Rectangle::square(25);
```

---

### 2.2 Enums and Pattern Matching

```rust
// Simple enum
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// Complex enum
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },   // Anonymous struct
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Tuple
}

impl Message {
    fn call(&self) {
        // Method on enum
    }
}
```

#### Option Enum (No Null in Rust!)
```rust
// Option is built-in: Some(T) or None
let some_number: Option<i32> = Some(5);
let absent_number: Option<i32> = None;

// Must handle None case
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
```

#### Match Expression
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Match with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Catch-all pattern
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),  // _ is catch-all
}

// if let (shorthand for single pattern)
let some_value = Some(3);
if let Some(3) = some_value {
    println!("three");
}
```

---

### 2.3 Collections

#### Vectors
```rust
// Create vector
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];  // Using macro

// Modify vector
let mut v = Vec::new();
v.push(5);
v.push(6);
v.pop();  // Returns Option<T>

// Access elements
let v = vec![1, 2, 3, 4, 5];
let third = &v[2];       // Panics if out of bounds
let third = v.get(2);    // Returns Option<&T>

match v.get(2) {
    Some(third) => println!("Third: {}", third),
    None => println!("No third element"),
}

// Iterate
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// Iterate and modify
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // Dereference to modify
}

// Store different types using enum
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

#### Strings
```rust
// Create string
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");

// Update string
let mut s = String::from("foo");
s.push_str("bar");  // Append string slice
s.push('!');        // Append single char

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 is moved, s2 is borrowed

// Format macro (doesn't take ownership)
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);

// Cannot index strings (UTF-8 complexity)
// let h = s[0];  // Error!

// Iterate over chars
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// Iterate over bytes
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

#### HashMap
```rust
use std::collections::HashMap;

// Create
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Create from vectors
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// Access values
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Returns Option<&V>

match scores.get(&team_name) {
    Some(score) => println!("Score: {}", score),
    None => println!("Team not found"),
}

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Ownership: owned types are moved
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid here

// Update strategies
let mut scores = HashMap::new();

// Overwrite
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);  // Won't insert

// Update based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

---

### 2.4 Error Handling

#### Result Type
```rust
// Result<T, E> is built-in
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Using Result
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
}
```

#### Shortcuts: unwrap and expect
```rust
// unwrap: panics on error
let f = File::open("hello.txt").unwrap();

// expect: panics with custom message
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

#### Propagating Errors with ?
```rust
use std::fs::File;
use std::io::{self, Read};

// Verbose way
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Using ? operator (much cleaner)
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter with chaining
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Shortest with fs utility
use std::fs;
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

---

## Phase 3: Intermediate Concepts

### 3.1 Generics

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Multiple type parameters
struct Point<T, U> {
    x: T,
    y: U,
}

let p = Point { x: 5, y: 4.0 };

// Implement for specific types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

---

### 3.2 Traits

```rust
// Define trait
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implement trait
pub struct NewsArticle {
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }
}

// Default implementation
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Trait bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// where clause (cleaner for complex bounds)
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}

// Return types that implement trait
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win!"),
        content: String::from("The Pittsburgh Penguins won!"),
    }
}
```

#### Common Traits
```rust
// Debug - for {:?} formatting
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Clone - explicit copy
#[derive(Clone)]
struct MyStruct {
    data: String,
}

// Copy - implicit copy (only for stack data)
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// PartialEq - equality comparison
#[derive(PartialEq)]
struct User {
    name: String,
}

// Automatically derive multiple traits
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}
```

---

### 3.3 Lifetimes

```rust
// Every reference has a lifetime
// Most of the time, lifetimes are implicit

// Explicit lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime in struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Lifetime elision rules (compiler infers lifetimes)
// 1. Each parameter gets its own lifetime
// 2. If one input lifetime, it's assigned to all outputs
// 3. If &self or &mut self, its lifetime is assigned to all outputs

// Static lifetime (lives entire program)
let s: &'static str = "I have a static lifetime.";
```

---

### 3.4 Closures

```rust
// Closure syntax
let add_one = |x| x + 1;
let sum = add_one(5);  // 6

// Type annotations (optional)
let add_one = |x: i32| -> i32 { x + 1 };

// Capturing environment
let x = 4;
let equal_to_x = |z| z == x;  // Captures x
let y = 4;
assert!(equal_to_x(y));

// Fn traits (how closures capture)
// FnOnce - consumes captured variables (takes ownership)
// FnMut - mutably borrows
// Fn - immutably borrows

// Using closures
let v = vec![1, 2, 3];
let v_doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// Store closure in struct
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

---

### 3.5 Iterators

```rust
// Three forms of iteration
let v = vec![1, 2, 3];

// iter() - borrows immutably
for val in v.iter() {
    println!("{}", val);
}

// iter_mut() - borrows mutably
for val in v.iter_mut() {
    *val += 1;
}

// into_iter() - takes ownership
for val in v.into_iter() {
    println!("{}", val);
}
// v is no longer valid here

// Iterator adapters (lazy)
let v = vec![1, 2, 3];
let v2: Vec<i32> = v.iter().map(|x| x + 1).collect();

// Chaining
let sum: i32 = v
    .iter()
    .filter(|x| **x > 1)
    .map(|x| x * 2)
    .sum();

// Common iterator methods
let v = vec![1, 2, 3, 4, 5];
let sum: i32 = v.iter().sum();
let product: i32 = v.iter().product();
let max = v.iter().max();
let min = v.iter().min();
let any_even = v.iter().any(|x| x % 2 == 0);
let all_positive = v.iter().all(|x| *x > 0);

// Creating custom iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

### 3.6 Smart Pointers

#### Box<T> - Heap allocation
```rust
// Store data on heap
let b = Box::new(5);
println!("{}", b);

// Recursive types (impossible without Box)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
```

#### Rc<T> - Reference counting
```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);  // Increase reference count
let c = Rc::clone(&a);

println!("Count: {}", Rc::strong_count(&a));  // 3

// All three pointers share ownership
// Data dropped when last Rc goes out of scope
```

#### RefCell<T> - Interior mutability
```rust
use std::cell::RefCell;

let x = RefCell::new(5);
*x.borrow_mut() += 1;  // Mutable borrow at runtime

// Combining Rc and RefCell for multiple owners of mutable data
use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));
let a = Rc::clone(&value);
let b = Rc::clone(&value);

*value.borrow_mut() += 10;
```

---

### 3.7 Concurrency

#### Threads
```rust
use std::thread;
use std::time::Duration;

// Spawn thread
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// Wait for thread to finish
handle.join().unwrap();

// Move closure to transfer ownership
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("vector: {:?}", v);
});
handle.join().unwrap();
```

#### Message Passing
```rust
use std::sync::mpsc;
use std::thread;

// Create channel
let (tx, rx) = mpsc::channel();

// Sender in thread
thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

// Receiver in main thread
let received = rx.recv().unwrap();
println!("Got: {}", received);

// Multiple messages
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    let vals = vec!["hi", "from", "thread"];
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

#### Shared State with Mutex
```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Arc - atomic reference counting (thread-safe Rc)
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

---

## Next Steps

After mastering these concepts, move on to:
- **Async/Await** - Asynchronous programming with Tokio
- **Web Development** - Axum, Actix-web frameworks
- **Macros** - Declarative and procedural macros
- **Unsafe Rust** - When you need to break the rules
- **Advanced Types** - Type aliases, never type, DSTs
- **FFI** - Calling C from Rust and vice versa

---

## Learning Resources

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: Interactive exercises
- **Rust Standard Library Docs**: https://doc.rust-lang.org/std/
- **This Week in Rust**: Weekly newsletter
- **r/rust**: Reddit community
- **Rust Users Forum**: Community discussions

---

**Key Takeaway**: Rust's learning curve is steep because it teaches you to think about memory safety, ownership, and concurrency in new ways. The compiler is your teacher - read error messages carefully!
