# Manifesto
Cortada is a language designed around high-level syntax and ergonomics with native execution.

# Philosophy
- The programmer owns intent. The compiler owns complexity.
- Runtime behavior should remain predictable without requiring direct low-level control.
- The memory model is implicit and inferred.
- Error handling is explicit and performed entirely through results. Failure propogation and handling must remain visible in code.
- The language will remain strongly and statically typed
- Abstractions should have minimal or predictable runtime cost
- All design choices must keep in mind the following NO list
  - No garbage collection
  - No user-facing pointer semantics
  - No dynamic types (not to be confused with generics)
  - No exceptions
  - No inheritance
  - No hidden runtime dispatch mechanisms


---

# Conventions

- Variables in Cortada are bindings, not mutable memory locations.
- Bindings are lexically scoped.
- Shadowing destroys the previous binding before creating the new binding.

- Composition is the primary abstraction mechanism.
- Structures will have field-level visibility control to preserve encapsulation
- Enumerations behave as tagged unions and provide safe, exhaustive handling of variant data.

- Variable and function names must use `snake_case`
- Type names must use `PascalCase`

