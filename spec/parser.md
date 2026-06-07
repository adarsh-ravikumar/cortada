# Parser

## Productions

```text
program -> statements EOF
```

---

## References

| Document       | Productions  |
| -------------- | ------------ |
| `statement.md` | `statements` |

## Referenced By

None.

## See Also

* `statement.md`
* `expression.md`
* `type.md`
* `variables.md`
* `function.md`
* `control_flow.md`

# Examples

## Empty Program

```cortada
```

### Notes

* A program may be empty.
* An empty program performs no actions.

### Diagnostics

None.

---

## Simple Program

```cortada
x: int = 42

print(x)
```

### Notes

* Statements are executed sequentially.
* Execution begins at the first statement.
* Program execution terminates when the final statement completes.

### Diagnostics

* Unexpected token after end of file.

---

## Function Definitions

```cortada
fn add(a: int, b: int) -> int:
  return a + b

result = add(3, 4)

print(result)
```

### Notes

* Functions may appear anywhere a statement is permitted.
* Programs are composed of a sequence of statements.

### Diagnostics

* Expected statement.
* Unexpected indentation without a block header.
