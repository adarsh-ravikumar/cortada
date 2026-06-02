# Control Flow

The following ways exist in Cortada to allow control flow:
- `if-elif-else`
  Conditional branching.

  `if condition:
    statements
  elif condition:
    statements
  else condition:
    statements`

- `for`
  Iterative loop

  `for ITERATOR:
    statements`
    
  An iterator can either be 
  - A range defined using the `start..end` for end-exclusive and `start..=end` for end-inclusive ranges
  - An object that implements the iterator trait, such as Strings, Arrays, Tuples etc.
  
- `while`
  Conditional loop

  `while condition:
    statements
  else:
    statements`

  Runs until condition is false. If the loop exits upon exhausting the condition, the else clause is executed.
  If the loop is broken out from, by the programmer, then the else clause is not invoked
  
