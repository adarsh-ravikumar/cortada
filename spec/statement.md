
# Statements

## Productions

```text
statements -> NEWLINE* statement (NEWLINE+ statement)* NEWLINE*

statement     -> var_decl
               | var_assign
               | fn_stmt
               | while_stmt
               | if_stmt
               | return_stmt
               | break_stmt
               | continue_stmt
               | expression

return_stmt   -> "return" expression?

break_stmt    -> "break"

continue_stmt -> "continue"
```

---

## References

| Document          | Productions              |
| ----------------- | ------------------------ |
| `variables.md`    | `var_decl`, `var_assign` |
| `function.md`     | `fn_stmt`                |
| `control_flow.md` | `while_stmt`, `if_stmt`  |
| `expression.md`   | `expression`             |

## Referenced By

* `control_flow.md`

## See Also

* `while_stmt`
* `if_stmt`
* `fn_stmt`
* `var_decl`
* `expression`

# Examples

## Statement List

### Single Statement

```cortada
x = 42
```

### Multiple Statements

```cortada
name = "John"

age = 18

print(name)
```

### Mixed Statements

```cortada
value = 10

if value > 0:
  print(value)

value = value - 1

print("done")
```

### Notes

* Statements are separated by one or more newlines.
* Leading and trailing newlines are ignored.
* Statements are evaluated sequentially.

### Diagnostics

* Expected statement.
* Unexpected indentation without a block header.

---

## Return Statement

### Return without Value

```cortada
fn log():
  return
```

### Return with Value

```cortada
fn add(a, b):
  return a + b
```

### Conditional Return

```cortada
fn sign(x):
  if x < 0:
    return -1

  return 1
```

### Notes

* `return` immediately terminates the current function.
* If present, the expression becomes the function's return value.
* Statements following a `return` are unreachable.

### Diagnostics

* `return` outside a function.
* Expected expression after `return`.

---

## Break Statement

### Loop Termination

```cortada
while true:
  if should_stop():
    break

  process()
```

### Notes

* `break` immediately terminates the innermost loop.
* Control resumes at the statement following the loop.

### Diagnostics

* `break` outside a loop.

---

## Continue Statement

### Skip Iteration

```cortada
while has_items():
  item = next_item()

  if item == null:
    continue

  process(item)
```

### Notes

* `continue` skips the remainder of the current iteration.
* Execution resumes at the beginning of the next iteration.

### Diagnostics

* `continue` outside a loop.

---

## Expression Statement

### Function Call

```cortada
print("Hello, world!")
```

### Nested Call

```cortada
save(load_config())
```

### Arithmetic Expression

```cortada
x + y
```

### Notes

* Any expression may appear as a statement.
* The value produced by an expression statement is discarded.

### Diagnostics

* Expected expression.
