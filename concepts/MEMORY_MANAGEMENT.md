# Memory Management: From Fundamentals to Rust

## Part 1: General Memory Management Concepts

### Program Memory Layout

When a program runs, the operating system allocates memory in different segments:

```
+------------------+  <- High Memory Address
|      Stack       |  (grows downward)
|        ↓         |
+------------------+
|                  |
|   Free Memory    |
|                  |
+------------------+
|        ↑         |
|      Heap        |  (grows upward)
+------------------+
|   BSS Segment    |  (uninitialized data)
+------------------+
|   Data Segment   |  (initialized data)
+------------------+
|   Text Segment   |  (code/instructions)
+------------------+  <- Low Memory Address
```

---

### Memory Access Order & Performance

**CPU Cache Hierarchy** (fastest to slowest):
1. **CPU Registers** (nanoseconds) - Direct CPU access
2. **L1 Cache** (~1 ns) - Per-core cache
3. **L2 Cache** (~3-10 ns) - Per-core or shared
4. **L3 Cache** (~10-50 ns) - Shared across cores
5. **RAM** (~100 ns) - Main memory
6. **Disk/SSD** (milliseconds) - Persistent storage

**Memory Segment Access Speed**:
- **Stack**: Fastest (usually in cache, sequential access)
- **Data/BSS**: Fast (loaded at startup, often cached)
- **Text**: Fast (read-only, heavily cached)
- **Heap**: Slower (random access, potential cache misses, fragmentation)

---

### Complete Memory Storage Map in Rust

| What | Where Stored | Lifetime | Example |
|------|--------------|----------|---------|
| **Function code** | Text Segment | Program lifetime | `fn main() {}` |
| **Constants** | Inlined or Text/Data | Compile-time | `const PI: f64 = 3.14` |
| **Static variables (initialized)** | Data Segment | Program lifetime | `static NAME: &str = "Rust"` |
| **Static variables (uninitialized)** | BSS Segment | Program lifetime | `static mut COUNT: i32 = 0` |
| **Local variables (fixed size)** | Stack | Function scope | `let x = 5` |
| **Function parameters** | Stack | Function scope | `fn foo(x: i32)` |
| **Return addresses** | Stack | Function call | Automatic |
| **String literals** | Data Segment (read-only) | Program lifetime | `"hello"` |
| **Dynamic strings** | Heap (data) + Stack (metadata) | Owner's scope | `String::from("hello")` |
| **Vectors** | Heap (data) + Stack (metadata) | Owner's scope | `Vec::new()` |
| **Box** | Heap (data) + Stack (pointer) | Owner's scope | `Box::new(5)` |
| **HashMap** | Heap (data) + Stack (metadata) | Owner's scope | `HashMap::new()` |
| **Rc/Arc** | Heap (data + counter) + Stack (pointer) | Last reference | `Arc::new(data)` |

---

### 1. Text Segment (Code Segment)

**What it stores**: Compiled machine code (your program instructions)

**Characteristics**:
- Read-only (prevents accidental code modification)
- Shareable (multiple instances can share same code)
- Fixed size at compile time

**Example**:
```rust
fn add(a: i32, b: i32) -> i32 {  // This function code lives in text segment
    a + b
}
```

---

### 2. Data Segment (Initialized Data)

**What it stores**: Global and static variables with initial values

**Characteristics**:
- Read-write access
- Fixed size at compile time
- Exists for entire program lifetime

**Example**:
```rust
static GREETING: &str = "Hello";  // Lives in data segment
static mut COUNTER: i32 = 0;      // Lives in data segment
```

---

### 3. BSS Segment (Block Started by Symbol)

**What it stores**: Uninitialized global and static variables

**Characteristics**:
- Automatically initialized to zero
- Doesn't take space in executable file (optimization)
- Fixed size at compile time

**Example**:
```rust
static mut BUFFER: [u8; 1024] = [0; 1024];  // Lives in BSS
```

**Why separate from Data?**: Saves disk space - no need to store 1024 zeros in the executable!

---

### 4. Heap

**What it stores**: Dynamically allocated memory at runtime

**Characteristics**:
- Grows upward (toward higher addresses)
- Manual allocation and deallocation
- Slower than stack (requires system calls)
- Can cause fragmentation
- Flexible size

**Languages with Manual Management** (C/C++):
```c
int* ptr = malloc(sizeof(int) * 100);  // Allocate
free(ptr);                              // Must manually free!
```

**Languages with Garbage Collection** (Java/Python/Go):
```java
List<Integer> list = new ArrayList<>();  // Allocate
// GC automatically frees when no references exist
```

**Common Issues**:
- Memory leaks (forgot to free)
- Double free (freed twice)
- Use after free (accessing freed memory)
- Dangling pointers

---

### 5. Stack

**What it stores**: 
- Local variables
- Function parameters
- Return addresses
- Function call frames

**Characteristics**:
- Grows downward (toward lower addresses)
- LIFO (Last In, First Out)
- Very fast allocation/deallocation (just move stack pointer)
- Fixed size per thread (typically 1-8 MB)
- Automatic cleanup when function returns

**Stack Frame Structure**:
```
+------------------+
| Return Address   |  <- Where to go after function ends
+------------------+
| Previous Frame   |  <- Saved frame pointer
+------------------+
| Local Variables  |  <- Function's local data
+------------------+
| Parameters       |  <- Function arguments
+------------------+
```

**Example Flow**:
```rust
fn main() {              // Stack frame 1
    let x = 10;          // x on stack
    let y = calculate(x); // New frame created
}                        // Frame destroyed, x deallocated

fn calculate(n: i32) -> i32 {  // Stack frame 2
    let result = n * 2;         // result on stack
    result                      // Frame destroyed, result deallocated
}
```

**Stack Overflow**: Happens when stack grows too large (deep recursion, large local arrays)

---

### Memory Allocation Comparison

| Feature | Stack | Heap |
|---------|-------|------|
| **Speed** | Very fast (pointer bump) | Slower (find free block) |
| **Size** | Limited (1-8 MB) | Large (limited by RAM) |
| **Lifetime** | Function scope | Manual or GC |
| **Allocation** | Automatic | Explicit |
| **Deallocation** | Automatic | Manual or GC |
| **Fragmentation** | No | Yes |
| **Thread Safety** | Per-thread | Shared, needs sync |

---

### Complete Example: Where Everything Lives

```rust
// TEXT SEGMENT: Function code
fn main() {
    // TEXT/DATA SEGMENT: String literal (read-only)
    let greeting = "Hello";  // greeting (pointer) on STACK, "Hello" in DATA
    
    // STACK: All these variables
    let x = 5;           // i32 on stack
    let y = 10;          // i32 on stack
    let arr = [1, 2, 3]; // array on stack
    
    // HEAP + STACK: Dynamic data
    let s = String::from("world");  // Stack: ptr+len+cap, Heap: "world"
    let v = vec![1, 2, 3];          // Stack: ptr+len+cap, Heap: [1,2,3]
    let b = Box::new(100);          // Stack: pointer, Heap: 100
    
    // TEXT SEGMENT: Function call
    process_data(&s);  // Stack: return address pushed
    
    // All stack variables dropped here
    // All heap allocations freed here
}

// TEXT SEGMENT: Function code
fn process_data(data: &str) {
    // STACK: parameter (reference/pointer)
    println!("{}", data);
}

// DATA SEGMENT: Static with initial value
static COUNTER: i32 = 0;

// BSS SEGMENT: Static without initial value (zero-initialized)
static mut BUFFER: [u8; 1024] = [0; 1024];

// INLINED: Constant (no memory address, replaced at compile time)
const MAX_SIZE: usize = 100;
```

---

### Detailed Memory Layout for a Rust Program

```
MEMORY ADDRESS    SEGMENT         CONTENT
═══════════════════════════════════════════════════════════════

0x00000000        TEXT            fn main() { ... }
0x00000100                        fn process_data() { ... }
0x00000200                        Standard library code

───────────────────────────────────────────────────────────────

0x00400000        DATA            static COUNTER: i32 = 0
0x00400004                        String literal: "Hello"
0x00400010                        String literal: "world"

───────────────────────────────────────────────────────────────

0x00600000        BSS             static mut BUFFER: [u8; 1024]
                                  (initialized to zeros)

───────────────────────────────────────────────────────────────

0x10000000        HEAP            ┌─ String data: "world" (5 bytes)
                  (grows up)      ├─ Vec data: [1,2,3] (12 bytes)
                                  └─ Box data: 100 (4 bytes)

───────────────────────────────────────────────────────────────

0x7FFFFFFFFFFF    STACK           ┌─ main() frame:
                  (grows down)    │  - greeting: ptr to "Hello"
                                  │  - x: 5
                                  │  - y: 10
                                  │  - arr: [1,2,3]
                                  │  - s: {ptr, len:5, cap:5}
                                  │  - v: {ptr, len:3, cap:3}
                                  │  - b: ptr to heap
                                  │
                                  └─ process_data() frame:
                                     - data: ptr (reference)
```

---

### Access Order and Fetch Priority

When CPU needs data, it checks in this order:

**1. Registers** (if already loaded)
```rust
let x = 5;
let y = x + 10;  // x likely in register, y computed in register
```

**2. L1/L2/L3 Cache** (if recently accessed)
```rust
for i in 0..1000 {
    arr[i] = i;  // Sequential access, stays in cache (FAST)
}
```

**3. RAM - Different segments accessed**

**Stack access** (fastest from RAM):
```rust
let x = 5;           // Stack - very fast
let y = x * 2;       // Stack - very fast
```

**Data/BSS segment** (fast, loaded at startup):
```rust
static CONFIG: &str = "production";  // Data segment - fast
```

**Heap access** (slower, random access):
```rust
let v = vec![1, 2, 3];  // Heap allocation - slower
v.push(4);              // Heap reallocation - even slower
```

---

### Module and Function Storage

```rust
// File: src/config.rs

// TEXT SEGMENT: Function code
pub fn get_config() -> &'static str {
    CONFIG
}

// DATA SEGMENT: Module-level static
static CONFIG: &str = "production";

// INLINED: Constant (no storage, replaced at compile time)
pub const MAX_RETRIES: u32 = 3;

// TEXT SEGMENT: Another function
pub fn validate(input: &str) -> bool {
    // STACK: Local variables
    let len = input.len();
    let is_valid = len > 0;
    is_valid
}
```

**When module is used**:
```rust
// src/main.rs
mod config;  // Tells compiler to include config module

fn main() {
    // TEXT SEGMENT: config::get_config function already loaded
    // DATA SEGMENT: config::CONFIG already loaded
    // COMPILE TIME: config::MAX_RETRIES replaced with 3
    
    let cfg = config::get_config();  // Returns pointer to DATA segment
    let retries = config::MAX_RETRIES;  // Value 3 inlined here
}
```

---

### Storage by Variable Type in Rust

#### Primitive Types (Stack)
```rust
let x: i32 = 5;           // Stack: 4 bytes
let y: f64 = 3.14;        // Stack: 8 bytes
let b: bool = true;       // Stack: 1 byte
let c: char = 'A';        // Stack: 4 bytes (UTF-8)
```

#### Arrays (Stack if size known)
```rust
let arr: [i32; 3] = [1, 2, 3];  // Stack: 12 bytes (3 * 4)
```

#### Tuples (Stack if all elements are stack types)
```rust
let tuple: (i32, f64) = (5, 3.14);  // Stack: 12 bytes
```

#### String Literals (Data Segment)
```rust
let s: &str = "hello";  // Stack: pointer (8 bytes)
                        // Data: "hello" (5 bytes, read-only)
```

#### String (Heap + Stack)
```rust
let s: String = String::from("hello");
// Stack: 24 bytes (ptr: 8, len: 8, cap: 8)
// Heap: 5 bytes ("hello")
```

#### Vec (Heap + Stack)
```rust
let v: Vec<i32> = vec![1, 2, 3];
// Stack: 24 bytes (ptr: 8, len: 8, cap: 8)
// Heap: 12 bytes ([1, 2, 3])
```

#### Box (Heap + Stack)
```rust
let b: Box<i32> = Box::new(5);
// Stack: 8 bytes (pointer)
// Heap: 4 bytes (5)
```

#### HashMap (Heap + Stack)
```rust
use std::collections::HashMap;
let mut map: HashMap<String, i32> = HashMap::new();
// Stack: 48 bytes (internal structure)
// Heap: dynamic (grows with entries)
```

#### Rc/Arc (Heap + Stack)
```rust
use std::rc::Rc;
let rc: Rc<i32> = Rc::new(5);
// Stack: 8 bytes (pointer)
// Heap: 12 bytes (4 for value + 8 for ref counts)
```

---

### Function Call Memory Flow

```rust
fn main() {                          // STACK FRAME 1
    let x = 10;                      // Stack: x = 10
    let result = calculate(x, 20);   // Push return address
                                     // Push parameters
                                     // Jump to calculate
    println!("{}", result);          // Stack: result = 30
}                                    // Frame 1 destroyed

fn calculate(a: i32, b: i32) -> i32 {  // STACK FRAME 2
    let sum = a + b;                    // Stack: a=10, b=20, sum=30
    sum                                 // Return value copied to caller
}                                       // Frame 2 destroyed
```

**Stack during execution**:
```
Before calculate():
┌──────────────┐
│ x = 10       │ <- main frame
│ result = ?   │
└──────────────┘

During calculate():
┌──────────────┐
│ a = 10       │ <- calculate frame
│ b = 20       │
│ sum = 30     │
├──────────────┤
│ return addr  │
├──────────────┤
│ x = 10       │ <- main frame
│ result = ?   │
└──────────────┘

After calculate():
┌──────────────┐
│ x = 10       │ <- main frame
│ result = 30  │
└──────────────┘
```

---

### Module Memory Organization

```rust
// src/database.rs
pub struct DbPool {
    connections: Vec<Connection>,  // Heap: vector data
}

static mut GLOBAL_POOL: Option<DbPool> = None;  // BSS: uninitialized

pub fn init() {
    unsafe {
        GLOBAL_POOL = Some(DbPool::new());  // Heap allocation
    }
}

// src/main.rs
mod database;

fn main() {
    database::init();  // Allocates heap memory for GLOBAL_POOL
}
```

**Memory layout**:
- `database::init` function code → **Text Segment**
- `GLOBAL_POOL` static variable → **BSS Segment** (initially)
- `DbPool` struct instance → **Heap** (after init)
- `connections` Vec data → **Heap**

---

### Compile-Time vs Runtime: Memory Classification

Understanding when memory is allocated and what happens at each phase is crucial for performance optimization.

---

#### **Compile-Time (Before Program Runs)**

**What Happens**:
- Code is compiled to machine instructions
- Memory layout is determined
- Constants are resolved and inlined
- Static variables are placed in binary
- Type checking and size calculations

**Memory Segments Prepared**:

| Segment | Compile-Time Activity | Size |
|---------|----------------------|------|
| **Text** | Function code compiled to machine instructions | Fixed |
| **Data** | Initialized static variables embedded in binary | Fixed |
| **BSS** | Uninitialized statics recorded (not stored in binary) | Fixed |
| **Stack** | Size determined per thread (OS default) | Fixed per thread |
| **Heap** | Not allocated yet | 0 bytes |

**Examples**:

```rust
// COMPILE TIME: Constant inlined (no memory allocation)
const MAX_SIZE: usize = 100;
// Compiler replaces every use of MAX_SIZE with 100

// COMPILE TIME: Static allocated in Data segment
static GREETING: &str = "Hello";
// Binary contains "Hello" in Data segment

// COMPILE TIME: Function compiled to machine code
fn add(a: i32, b: i32) -> i32 {
    a + b
}
// Machine instructions stored in Text segment

// COMPILE TIME: Type and size known
struct User {
    id: u32,      // 4 bytes
    age: u8,      // 1 byte
    active: bool, // 1 byte
}
// Compiler knows: User = 8 bytes (with padding)
```

**Binary File Contains**:
```
┌─────────────────────┐
│ Text Segment        │ <- Compiled machine code
├─────────────────────┤
│ Data Segment        │ <- Initialized statics: "Hello"
├─────────────────────┤
│ BSS Info            │ <- Size info (not actual zeros)
└─────────────────────┘
```

---

#### **Runtime (When Program Executes)**

**What Happens**:
- OS loads binary into memory
- Stack allocated for main thread
- Heap manager initialized
- Dynamic allocations occur
- Variables created and destroyed

**Memory Segments at Runtime**:

| Segment | Runtime Activity | Dynamic? |
|---------|-----------------|----------|
| **Text** | Loaded once, never changes | No |
| **Data** | Loaded once, can be read/written | No (size fixed) |
| **BSS** | Initialized to zeros by OS | No (size fixed) |
| **Stack** | Grows/shrinks with function calls | Yes (within limit) |
| **Heap** | Allocates/deallocates dynamically | Yes (unlimited*) |

**Examples**:

```rust
fn main() {
    // RUNTIME: Stack allocation (automatic)
    let x = 5;  // 4 bytes allocated on stack
    
    // RUNTIME: Heap allocation (dynamic)
    let s = String::from("hello");  // Heap memory allocated
    
    // RUNTIME: More heap allocation
    let v = vec![1, 2, 3];  // Heap memory allocated
    
    // RUNTIME: Function call (stack frame created)
    process(x);
    
    // RUNTIME: Heap deallocation (automatic via Drop)
    // s and v freed when they go out of scope
}
```

---

#### **Complete Classification Table**

| Item | When Allocated | Where Stored | Size Known | Lifetime | Example |
|------|---------------|--------------|------------|----------|---------|
| **Constants** | Compile-time (inlined) | None (or Text) | Compile-time | N/A (inlined) | `const PI: f64 = 3.14` |
| **String Literals** | Compile-time | Data Segment | Compile-time | Program | `"hello"` |
| **Static Variables** | Compile-time | Data/BSS | Compile-time | Program | `static COUNT: i32 = 0` |
| **Function Code** | Compile-time | Text Segment | Compile-time | Program | `fn main() {}` |
| **Local Variables (primitives)** | Runtime | Stack | Compile-time | Function scope | `let x = 5` |
| **Local Variables (arrays)** | Runtime | Stack | Compile-time | Function scope | `let arr = [1,2,3]` |
| **Function Parameters** | Runtime | Stack | Compile-time | Function scope | `fn foo(x: i32)` |
| **String** | Runtime | Heap + Stack | Runtime | Owner's scope | `String::from("hi")` |
| **Vec** | Runtime | Heap + Stack | Runtime | Owner's scope | `vec![1, 2, 3]` |
| **Box** | Runtime | Heap + Stack | Runtime | Owner's scope | `Box::new(5)` |
| **HashMap** | Runtime | Heap + Stack | Runtime | Owner's scope | `HashMap::new()` |
| **Rc/Arc** | Runtime | Heap + Stack | Runtime | Last reference | `Arc::new(data)` |

---

#### **Detailed Breakdown by Phase**

### **Phase 1: Compile-Time**

```rust
// File: src/main.rs

const MAX_USERS: usize = 1000;  // ✓ Inlined at compile-time

static CONFIG: &str = "prod";   // ✓ Placed in Data segment

fn calculate(x: i32) -> i32 {   // ✓ Compiled to machine code
    x * 2
}

struct User {                   // ✓ Size calculated: 16 bytes
    id: u32,
    name: String,
}
```

**Compiler Output**:
```
✓ MAX_USERS: Replaced with 1000 everywhere
✓ CONFIG: Address 0x00400000 in Data segment
✓ calculate: Machine code at 0x00000100 in Text
✓ User: Size = 16 bytes (4 for id + 12 for String metadata)
```

---

### **Phase 2: Program Load (Runtime Start)**

```
OS loads binary:
┌─────────────────────┐
│ Text: 0x00000000    │ <- Function code loaded
├─────────────────────┤
│ Data: 0x00400000    │ <- "prod" loaded
├─────────────────────┤
│ BSS:  0x00600000    │ <- Zeros initialized
├─────────────────────┤
│ Heap: 0x10000000    │ <- Empty (ready for allocations)
├─────────────────────┤
│ Stack: 0x7FFF...    │ <- Main thread stack (1-8 MB)
└─────────────────────┘
```

---

### **Phase 3: Runtime Execution**

```rust
fn main() {
    // RUNTIME: Stack frame for main() created
    
    // RUNTIME: Stack allocation (instant)
    let x: i32 = 5;  // 4 bytes on stack
    
    // RUNTIME: Heap allocation (slower, system call)
    let s = String::from("hello");
    // Stack: 24 bytes (ptr + len + cap)
    // Heap: 5 bytes ("hello")
    
    // RUNTIME: More heap allocation
    let mut v = Vec::new();
    v.push(1);  // Heap allocation
    v.push(2);  // Might reallocate if capacity exceeded
    
    // RUNTIME: Function call (new stack frame)
    let result = calculate(x);
    // Stack frame created, x copied, function executes, frame destroyed
    
    // RUNTIME: Automatic deallocation
    // v dropped -> heap freed
    // s dropped -> heap freed
    // x dropped -> stack reclaimed
    // main() frame destroyed
}
```

---

#### **Memory Timeline Visualization**

```
TIME: Compile-time
═══════════════════════════════════════════════════════════
Action: Analyze code, generate binary
Memory: Text, Data, BSS segments prepared
Size: Fixed and known

───────────────────────────────────────────────────────────

TIME: Program start (t=0)
═══════════════════════════════════════════════════════════
Action: OS loads binary, initializes runtime
Memory: Text/Data/BSS loaded, Stack allocated, Heap ready
Size: Text/Data/BSS fixed, Stack 1-8MB, Heap 0 bytes

───────────────────────────────────────────────────────────

TIME: main() starts (t=1ms)
═══════════════════════════════════════════════════════════
Action: Stack frame created
Memory: Stack grows by ~100 bytes
Size: Stack increases, Heap still 0

───────────────────────────────────────────────────────────

TIME: String::from() called (t=2ms)
═══════════════════════════════════════════════════════════
Action: Heap allocation
Memory: Heap allocates 5 bytes, Stack stores pointer
Size: Stack +24 bytes, Heap +5 bytes

───────────────────────────────────────────────────────────

TIME: Vec operations (t=3ms)
═══════════════════════════════════════════════════════════
Action: Multiple heap allocations/reallocations
Memory: Heap grows, might fragment
Size: Stack +24 bytes, Heap +variable

───────────────────────────────────────────────────────────

TIME: main() ends (t=10ms)
═══════════════════════════════════════════════════════════
Action: Automatic cleanup (Drop trait)
Memory: Heap freed, Stack reclaimed
Size: Stack back to 0, Heap back to 0

───────────────────────────────────────────────────────────

TIME: Program exit (t=11ms)
═══════════════════════════════════════════════════════════
Action: OS reclaims all memory
Memory: Everything freed
Size: 0 bytes
```

---

#### **Compile-Time vs Runtime: Performance Impact**

### **Compile-Time Decisions (Zero Runtime Cost)**

```rust
// Constant: Inlined at compile-time
const SIZE: usize = 100;
let arr = [0; SIZE];  // Compiler knows size = 100

// Generic: Monomorphization at compile-time
fn print<T: Display>(item: T) {
    println!("{}", item);
}
print(5);      // Compiler generates print_i32()
print("hi");   // Compiler generates print_str()
// No runtime dispatch!

// Type checking: All done at compile-time
let x: i32 = 5;
// let y: String = x;  // ERROR at compile-time, not runtime!
```

**Benefit**: Zero runtime overhead, maximum performance

---

### **Runtime Decisions (Has Cost)**

```rust
// Heap allocation: Runtime system call
let s = String::from("hello");  // ~100-1000 CPU cycles

// Dynamic dispatch: Runtime vtable lookup
trait Animal {
    fn speak(&self);
}
let animal: Box<dyn Animal> = Box::new(Dog {});
animal.speak();  // Runtime lookup (~10-50 cycles)

// Bounds checking: Runtime check
let v = vec![1, 2, 3];
let x = v[5];  // Runtime panic if out of bounds
```

**Cost**: Performance overhead, but provides flexibility

---

#### **Optimization: Moving Work to Compile-Time**

### **Bad: Runtime Calculation**
```rust
fn process() {
    let size = 10 * 10;  // Calculated at runtime
    let arr = vec![0; size];
}
```

### **Good: Compile-Time Calculation**
```rust
const SIZE: usize = 10 * 10;  // Calculated at compile-time
fn process() {
    let arr = [0; SIZE];  // Stack allocation, size known
}
```

---

### **Bad: Dynamic Dispatch**
```rust
fn process(animal: &dyn Animal) {  // Runtime vtable lookup
    animal.speak();
}
```

### **Good: Static Dispatch**
```rust
fn process<T: Animal>(animal: &T) {  // Compile-time monomorphization
    animal.speak();
}
```

---

#### **Memory Allocation Cost Comparison**

| Operation | When | Cost (CPU Cycles) | Speed |
|-----------|------|-------------------|-------|
| **Constant access** | Compile-time | 0 (inlined) | Instant |
| **Static access** | Compile-time (allocated) | 1-5 (memory read) | Very fast |
| **Stack allocation** | Runtime | 1-10 (pointer bump) | Very fast |
| **Stack deallocation** | Runtime | 1-5 (pointer bump) | Very fast |
| **Heap allocation** | Runtime | 100-1000 (system call) | Slow |
| **Heap deallocation** | Runtime | 50-500 (system call) | Slow |
| **Heap reallocation** | Runtime | 200-2000 (copy data) | Very slow |

---

#### **Practical Example: Compile-Time vs Runtime**

```rust
// Compile-time: Everything known
fn compile_time_example() {
    const SIZE: usize = 100;
    let arr: [i32; SIZE] = [0; SIZE];  // Stack, instant
    
    for i in 0..SIZE {  // Loop unrolled by optimizer
        // Process arr[i]
    }
}  // Instant cleanup

// Runtime: Dynamic allocation
fn runtime_example() {
    let size = read_from_config();  // Unknown at compile-time
    let mut vec = Vec::with_capacity(size);  // Heap allocation
    
    for i in 0..size {
        vec.push(i);  // Might reallocate
    }
}  // Drop trait called, heap freed
```

**Performance Difference**: Compile-time version is 10-100x faster!

---

#### **Key Takeaways**

**Compile-Time**:
- ✅ Zero runtime cost
- ✅ Maximum performance
- ✅ Errors caught early
- ❌ Less flexible
- **Use for**: Constants, type checking, generic monomorphization

**Runtime**:
- ✅ Flexible and dynamic
- ✅ Handles unknown sizes
- ❌ Performance overhead
- ❌ Potential errors at runtime
- **Use for**: User input, dynamic data structures, configuration

**Rust's Philosophy**: 
- Move as much as possible to compile-time
- Make runtime costs explicit and predictable
- Zero-cost abstractions: "What you don't use, you don't pay for"

---

### Performance Implications

**Fast Access** (Stack):
```rust
fn process_numbers() {
    let mut sum = 0;  // Stack
    for i in 0..1000 {
        sum += i;  // All in registers/cache
    }
}
```

**Slower Access** (Heap):
```rust
fn process_strings() {
    let mut strings = Vec::new();  // Heap allocation
    for i in 0..1000 {
        strings.push(format!("{}", i));  // Multiple heap allocations
    }
}
```

**Optimization** (Pre-allocate):
```rust
fn process_strings_fast() {
    let mut strings = Vec::with_capacity(1000);  // Single heap allocation
    for i in 0..1000 {
        strings.push(format!("{}", i));  // No reallocation
    }
}
```

---

## Part 2: Memory Management in Different Languages

### C/C++ - Manual Management
```c
// Heap allocation
int* arr = malloc(100 * sizeof(int));  // Must free
free(arr);

// Stack allocation
int arr[100];  // Automatic cleanup
```

**Problems**:
- Memory leaks
- Dangling pointers
- Double free
- Buffer overflows

---

### Java/Python/Go - Garbage Collection
```java
List<String> list = new ArrayList<>();  // Heap allocated
// GC automatically frees when unreachable
```

**Advantages**:
- No manual memory management
- No memory leaks (mostly)

**Disadvantages**:
- GC pauses (stop-the-world)
- Unpredictable performance
- Higher memory usage
- No control over when memory is freed

---

### Rust - Ownership System
```rust
let s = String::from("hello");  // Heap allocated
// Automatically freed when s goes out of scope
```

**Advantages**:
- No GC (no pauses)
- No manual free (no leaks)
- Memory safety guaranteed at compile time
- Zero-cost abstraction

---

## Part 3: Memory Management in Rust

### The Ownership System

Rust's revolutionary approach: **Compile-time memory management without GC**

**Three Rules**:
1. Each value has an owner
2. Only one owner at a time
3. Value is dropped when owner goes out of scope

---

### Stack vs Heap in Rust

#### Stack Allocation (Fixed Size Types)

```rust
fn main() {
    let x = 5;           // i32 on stack (4 bytes, known at compile time)
    let y = true;        // bool on stack (1 byte)
    let z = [1, 2, 3];   // array on stack (12 bytes)
}  // x, y, z automatically cleaned up
```

**Stack types**: `i32`, `f64`, `bool`, `char`, arrays, tuples (if all elements are stack types)

---

#### Heap Allocation (Dynamic Size Types)

```rust
fn main() {
    let s = String::from("hello");  // String data on heap
    let v = vec![1, 2, 3];          // Vec data on heap
    let b = Box::new(5);            // Boxed value on heap
}  // Heap memory automatically freed here!
```

**Heap types**: `String`, `Vec<T>`, `Box<T>`, `HashMap<K, V>`

**What's on Stack vs Heap**:
```rust
let s = String::from("hello");

// Stack (fixed size):
// - pointer to heap data
// - length (5)
// - capacity (5)

// Heap (dynamic size):
// - actual string data: "hello"
```

---

### Ownership in Action

#### Move Semantics (Heap Data)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 MOVED to s2, s1 is now invalid
    
    // println!("{}", s1);  // ERROR! s1 no longer owns the data
    println!("{}", s2);     // OK
}  // s2 dropped, heap memory freed
```

**Why?**: Prevents double-free (both s1 and s2 trying to free same memory)

---

#### Copy Semantics (Stack Data)

```rust
fn main() {
    let x = 5;
    let y = x;  // x COPIED to y, both valid
    
    println!("{}", x);  // OK
    println!("{}", y);  // OK
}
```

**Why?**: Stack data is cheap to copy, no heap allocation involved

---

#### Borrowing (References)

```rust
fn main() {
    let s1 = String::from("hello");
    
    let len = calculate_length(&s1);  // Borrow s1 (don't move)
    
    println!("Length of '{}' is {}", s1, len);  // s1 still valid!
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't own data, so nothing freed
```

**Borrowing Rules**:
- Any number of immutable references (`&T`)
- OR exactly one mutable reference (`&mut T`)
- References must always be valid (no dangling pointers)

---

### Memory Management in Rust Modules

#### Module-Level Static Variables

```rust
// src/config.rs
pub static MAX_CONNECTIONS: u32 = 100;  // Data segment
static mut COUNTER: u32 = 0;            // Data segment (unsafe to access)

pub fn increment_counter() {
    unsafe {
        COUNTER += 1;  // Requires unsafe block
    }
}
```

**Lifetime**: Entire program execution

---

#### Module-Level Constants

```rust
// src/constants.rs
pub const PI: f64 = 3.14159;  // Inlined at compile time (no memory address)
pub const MAX_SIZE: usize = 1024;
```

**Difference from static**: Constants are inlined, no fixed memory address

---

### Memory Management in Applications

#### Single Binary Application

```rust
// src/main.rs
fn main() {
    // Stack: local variables
    let config = load_config();  // Heap: Config struct
    
    // Stack: server variable (pointer, metadata)
    // Heap: actual server data
    let server = create_server(config);
    
    server.run();  // Runs until program exits
}  // All heap memory freed when main exits
```

---

#### Multi-Module Application

```
src/
├── main.rs       // Entry point
├── config.rs     // Configuration
├── database.rs   // DB connection pool
├── handlers.rs   // Request handlers
└── models.rs     // Data structures
```

**Memory Flow**:

```rust
// src/main.rs
use crate::database::DbPool;
use crate::handlers;

fn main() {
    // Heap: connection pool
    let pool = DbPool::new();
    
    // Heap: shared state (Arc for thread-safe sharing)
    let shared_state = Arc::new(AppState { pool });
    
    // Pass to handlers (reference counted, multiple owners)
    handlers::start_server(shared_state);
}

// src/database.rs
pub struct DbPool {
    connections: Vec<Connection>,  // Heap allocated
}

impl DbPool {
    pub fn new() -> Self {
        DbPool {
            connections: Vec::with_capacity(10),  // Heap allocation
        }
    }
}

impl Drop for DbPool {
    fn drop(&mut self) {
        // Custom cleanup when DbPool is dropped
        println!("Closing all database connections");
    }
}
```

---

### Smart Pointers for Advanced Memory Management

#### `Box<T>` - Single Ownership on Heap

```rust
let b = Box::new(5);  // Allocate i32 on heap
// Useful for large data or recursive types
```

---

#### `Rc<T>` - Reference Counted (Single Thread)

```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let data2 = Rc::clone(&data);  // Increment reference count
let data3 = Rc::clone(&data);  // Increment reference count

// Data freed when all Rc instances dropped
```

---

#### `Arc<T>` - Atomic Reference Counted (Multi-Thread)

```rust
use std::sync::Arc;

let shared = Arc::new(vec![1, 2, 3]);
let shared2 = Arc::clone(&shared);

std::thread::spawn(move || {
    println!("{:?}", shared2);  // Safe to use in another thread
});
```

---

#### `RefCell<T>` - Interior Mutability

```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;  // Runtime borrow checking
```

---

### Memory Management in Web Applications

```rust
// Typical Axum web server
use axum::{Router, extract::State};
use std::sync::Arc;

struct AppState {
    db_pool: DbPool,      // Heap: connection pool
    cache: Cache,         // Heap: cache data
}

#[tokio::main]
async fn main() {
    // Heap: shared application state
    let state = Arc::new(AppState {
        db_pool: DbPool::new(),
        cache: Cache::new(),
    });
    
    let app = Router::new()
        .route("/users", get(get_users))
        .with_state(state);  // Arc cloned for each handler
    
    // Server runs, state shared across all requests
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_users(State(state): State<Arc<AppState>>) -> String {
    // Each request gets a clone of Arc (cheap, just increments counter)
    // Stack: function parameters and local variables
    // Heap: response data
    
    let users = state.db_pool.get_users().await;
    format!("{:?}", users)
}  // Local variables dropped, Arc reference count decremented
```

---

### Memory Safety Guarantees

Rust prevents at **compile time**:

1. **Use after free**
```rust
let s = String::from("hello");
drop(s);  // Explicitly free
// println!("{}", s);  // ERROR: s is moved
```

2. **Double free**
```rust
let s = String::from("hello");
drop(s);
// drop(s);  // ERROR: s already moved
```

3. **Dangling pointers**
```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: s will be dropped, reference would be invalid
}
```

4. **Data races**
```rust
let mut data = vec![1, 2, 3];
let r1 = &data;
// let r2 = &mut data;  // ERROR: can't have mutable ref while immutable ref exists
```

---

## Part 4: Practical Memory Patterns in Rust

### Pattern 1: RAII (Resource Acquisition Is Initialization)

```rust
struct File {
    handle: FileHandle,
}

impl Drop for File {
    fn drop(&mut self) {
        // Automatically close file when File goes out of scope
        self.handle.close();
    }
}

fn process_file() {
    let file = File::open("data.txt");
    // Use file
}  // File automatically closed here, even if panic occurs!
```

---

### Pattern 2: Builder Pattern (Avoiding Temporary Allocations)

```rust
let query = QueryBuilder::new()
    .select("name, email")
    .from("users")
    .where_clause("age > 18")
    .build();  // Single allocation at the end
```

---

### Pattern 3: Cow (Clone on Write) - Optimize Allocations

```rust
use std::borrow::Cow;

fn process(input: &str) -> Cow<str> {
    if input.contains("bad") {
        Cow::Owned(input.replace("bad", "good"))  // Allocate only if needed
    } else {
        Cow::Borrowed(input)  // No allocation
    }
}
```

---

### Pattern 4: Arena Allocation (Bulk Deallocation)

```rust
// Useful for temporary data with same lifetime
struct Arena {
    data: Vec<u8>,
}

impl Arena {
    fn allocate(&mut self, size: usize) -> &mut [u8] {
        // Allocate from arena
    }
}  // All allocations freed at once when Arena dropped
```

---

## Summary: Why Rust's Memory Management is Revolutionary

| Aspect | C/C++ | Java/Go | Rust |
|--------|-------|---------|------|
| **Safety** | ❌ Manual, error-prone | ✅ GC handles it | ✅ Compiler enforces |
| **Performance** | ✅ No overhead | ❌ GC pauses | ✅ No overhead |
| **Predictability** | ✅ Deterministic | ❌ GC unpredictable | ✅ Deterministic |
| **Ease of Use** | ❌ Complex | ✅ Easy | ⚠️ Learning curve |
| **Concurrency** | ❌ Data races possible | ⚠️ GC + sync | ✅ Compile-time safety |

---

## Key Takeaways

1. **Stack**: Fast, automatic, limited size - use for small, fixed-size data
2. **Heap**: Flexible, slower, unlimited - use for dynamic, large data
3. **Ownership**: Each value has one owner, automatically freed when owner drops
4. **Borrowing**: Temporary access without ownership transfer
5. **No GC**: Rust achieves memory safety without garbage collection overhead
6. **Compile-time**: Most memory errors caught before program runs
7. **Zero-cost**: Abstractions have no runtime overhead

**The Rust Promise**: Memory safety without sacrificing performance!
