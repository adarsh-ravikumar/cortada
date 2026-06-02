# Phase I
Deliverable:
  A developer can install Cortada, write programs naturally, compile them and trust the result.
  Concrete philosphy documentation

Breakdown:
1) Design
   Deliverables: Syntax, semantics and philosphy

2) Emit C
  2.1) Lexer
  2.2) Parser
  2.3) IR
  2.4) Codegen
   Deliverables: A dumb compiler that kinda works. Toy programs can be compiled

3) Scope based destruction + Strings
   Deliverables: One step closer towards usability and memory safety

4) Types, structs and enums
   Deliverables: Concrete type philosphy, and compile to C

5) Real memory contracts
   Deliverables: Proper scope-bound memory management. Freeing and creating where required, and deciding what goes on stack and heap

6) Escape analysis
   Deliverables: Handle complex lifetimes
