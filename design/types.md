# Typing System
Cortada features a strongly and statically typed system.
This document goes over the in-built types provided from a birds-eye view. Specific implementation, usage and behaviour details are discussed elsewhere.

# Simple types
Cortada has two categories of simple types. Fundamentals and Containers

## Fundamental types
Cortada has 7 fundamental types. Operations and other type specific functions will be documented in `docs`

- `int`
  native machine-sized signed integer type

- `uint`
  native machine-sized unsigned integer type

- `float`
  native machine-sized floating point type

- `complex`
  real part    -> float
  imaginary part -> float 
  total size = 2 * float size

- `bool`
  1 byte in memory, semantically treated as a single bit

- `byte`
  1 byte in memory

- `char`
  Unicode scalar value. Fixed 32-bit semantic value


## Containers
Cortada has 5 container types. Operations and other type specific functions will be documented in `docs`.
Iterators and slices will be discussed in `design/containers.md`

- `array<T>`
  heap-backed homogeneous dynamic container
  data structure is mutable

- `tuple<...>`
  fixed size heterogenous container, stack-preferred value type
  elements are structurally immutable. 

- `map<K,V>`
  stores values against unique keys
  lives on the heap
  data structure is mutable

- `set<V>`
  stores unique values without associated data

- `string`
  UTF-8 encoded dynamically sized text container
  internally represented as `array<byte>`


# Compound types
Cortada has 2 compound types.
Implementing functions on compound types and value semantics are discussed in `design/compound_types.md`

- `struct`
  has fields that can be other simple or compound types
  has field-level visibility control
  one can implement functions on structs

- `enum`
  semantically, tagged unions.
  an instance represents exactly one active variant at a time
  enum members themselves can hold values of simple or compound types


