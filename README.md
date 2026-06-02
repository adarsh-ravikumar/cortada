# Cortada
A high-level compiled language with deterministic memory management.

---

Cortada is an experimental programming language focused on native performance, predictable memory behavior, and high-level ergonomics.

Write code like a scripting language.  
Run it like a native language.

Cortada aims to combine:
- Python-like readability
- TypeScript-inspired type ergonomics
- Deterministic memory management
- Native compilation

without exposing low-level memory semantics directly to the programmer.

The compiler is responsible for:
- Ownership tracking
- Lifetime analysis
- Destruction insertion
- Allocation optimization

The programmer focuses on:
- Behavior
- Structure
- Intent

---

## Why Cortada Exists

Modern scripting languages are productive and expressive, but often depend on garbage collection and heavyweight runtimes.

Systems languages provide deterministic performance and low-level control, but frequently expose memory-management concepts directly to the programmer.

Cortada explores a middle ground:

> A high-level native language where the compiler manages memory complexity automatically.

The goal is not to expose ownership semantics to the user, but to let the compiler reason about them internally while preserving deterministic cleanup and predictable performance.

---

## Philosophy

Cortada is built around a few core ideas.

### 1) High-Level Ergonomics

Programs should read naturally.

The language favors:
- Low punctuation
- Readable flow
- Strong type inference
- Expression-oriented APIs

---

### 2) Deterministic Memory Management

Cortada does not use a tracing garbage collector.

Instead, the compiler performs ownership and lifetime analysis to determine:
- when values die
- where destruction should occur
- what memory can be reclaimed safely

Cleanup is inserted automatically during compilation.

The goals are:
- Predictable performance
- No GC pauses
- No manual free calls

---

### 3) Compiler-Managed Complexity

Ownership, borrowing, and allocation are implementation details of the compiler — not concepts the user should constantly think about.

Cortada intentionally avoids:
- Raw pointers
- Manual memory management
- Explicit lifetime syntax

> The compiler owns complexity.  
> The programmer owns intent.

---

### 4) Native Performance

Cortada compiles to C as an intermediate backend target during early development.

The long-term goal is to produce:
- Efficient native binaries
- Predictable memory behavior
- Minimal runtime overhead

while preserving scripting-language ergonomics.

---

## Planned Features

- Static typing with inference
- Structs and enums
- Union types
- Flow-sensitive type narrowing
- Deterministic destruction
- Ownership inference
- Modules and packages
- Native compilation
- Expressive diagnostics

---

## Current Status

Cortada is currently in active design and early compiler development.

### Phase I Goals

- Lexer and parser
- Semantic analysis frontend
- Typed intermediate representation
- Deterministic scope-based destruction
- Structs and enums
- Ownership contracts
- Escape analysis
- C backend

### Phase I Deliverable

> A developer can install Cortada, write programs naturally, compile them, and trust the result.

---

## Project Goals

Cortada is primarily aimed at:
- Game development
- Tooling
- Realtime applications
- Scripting-heavy native software
- Performance-sensitive applications

The language is intentionally **not** designed for:
- Low-level systems programming
- Kernel interfacing
- Explicit manual memory control

---

## Non-Goals

Cortada currently does not aim to:
- Replace major systems languages like Rust or C++
- Expose manual ownership control
- Support raw pointer arithmetic
- Become a GC language

---

## Inspiration

Cortada draws inspiration from:
- Python
- TypeScript
- Rust
- Swift
- Nim

while pursuing a distinct philosophy centered around:
- High-level ergonomics
- Deterministic memory management
- Native performance
- Compiler-managed ownership

---

## Status

Experimental.

Everything is subject to change.

---

## License 

Cortada is licensed under the MIT license.

---

## Contributions

Cortada is currently in a highly experimental stage and is not yet open to external contributions.

The language semantics, compiler architecture, and memory model are still evolving rapidly.
