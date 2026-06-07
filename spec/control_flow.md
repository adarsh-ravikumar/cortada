# Control Flow

## Productions

```text
while_stmt -> "while" expression suite
              ("else" suite)?

if_stmt    -> "if" expression suite
              ("elif" expression suite)*
              ("else" suite)?

suite      -> ":" NEWLINE? INDENT statements DEDENT
```

---

## References

| Document        | Productions  |
| --------------- | ------------ |
| `expression.md` | `expression` |
| `statement.md`  | `statements` |

## Referenced By

* `statement.md`

## See Also

* `break_stmt`
* `continue_stmt`
* `return_stmt`

# Examples

## While Statement

### Simple Loop

```cortada
while x > 3:
  x = x - 1
```

### Compound Condition

```cortada
while is_running and count < 10:
  process()
  count = count + 1
```

### Loop with `else`

```cortada
while retries > 0:
  if try_connect():
    break

  retries = retries - 1
else:
  panic("Connection failed")
```

### Notes

* The `else` block is executed only when the loop condition becomes falsy.
* If the loop exits through `break` or `return`, the `else` block is not executed.

### Diagnostics

* Expected expression after `while`.
* Expected `:` after condition.

---

## If Statement

### Simple Condition

```cortada
if age >= 18:
  allow_access()
```

### If-Else

```cortada
if score >= 50:
  pass()
else:
  fail()
```

### Multiple Branches

```cortada
if value < 0:
  sign = -1
elif value > 0:
  sign = 1
else:
  sign = 0
```

### Nested Control Flow

```cortada
if is_authenticated:
  while has_work():
    process_next()
else:
  login()
```

### Notes

* Conditions are evaluated sequentially.
* At most one branch is executed.

### Diagnostics

* Expected expression after `if`.
* Expected expression after `elif`.
* Expected `:` after condition.
* Expected `:` after `else`.

---

## Suite

### Single Statement

```cortada
if x > 0:
  print(x)
```

### Multiple Statements

```cortada
while running:
  update()
  render()
  frame_count = frame_count + 1
```

### Nested Suites

```cortada
if connected:
  while has_messages():
    process_message()
else:
  reconnect()
```

### Notes

* A suite begins with a colon followed by an indented block.
* Suites introduce a new lexical scope.

### Diagnostics

* Expected indent after `:`.
* Unexpected indentation without a block header.

```
```
