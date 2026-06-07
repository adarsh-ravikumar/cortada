# Functions

## Productions

```text
fn_stmt -> "fn" IDENTIFIER
           "(" (param ("," param)*)? ","? ")"
           ("->" IDENTIFIER)?
           suite

param   -> IDENTIFIER
           (":" type_expression)?
           ("=" expression)?
```

---

## References

| Document          | Productions       |
| ----------------- | ----------------- |
| `type.md`         | `type_expression` |
| `expression.md`   | `expression`      |
| `control_flow.md` | `suite`           |

## Referenced By

* `statement.md`

## See Also

* `return_stmt`
* `type_expression`
* `expression`

# Examples

## Function Statement

### Empty Function

```cortada
fn greet():
  pass
```

### Function with Parameters

```cortada
fn add(a, b):
  return a + b
```

### Typed Parameters

```cortada
fn add(a: int, b: int):
  return a + b
```

### Return Type

```cortada
fn magnitude(x: float, y: float) -> float:
  return sqrt(x * x + y * y)
```

### Default Arguments

```cortada
fn connect(host = "localhost", port = 8080):
  ...
```

### Typed Default Arguments

```cortada
fn connect(host: string = "localhost", port: int = 8080):
  ...
```

### Trailing Comma

```cortada
fn create_player(
  name: string,
  age: int,
):
  ...
```

### Notes

* Function bodies are suites and introduce a new lexical scope.
* Parameter names must be unique within a function.
* Parameters may be optionally typed.
* Parameters may specify default values.
* Trailing commas are permitted in parameter lists.
* Return types are optional.
* Functions are first-class values.

### Diagnostics

* Expected function name after `fn`.
* Expected parameter name.
* Expected `)` to close parameter list.
* Expected type after `:`.
* Expected expression after `=`.
* Expected return type after `->`.
* Expected indent after `:`.

---

## Parameters

### Untyped Parameter

```cortada
fn greet(name):
  ...
```

### Typed Parameter

```cortada
fn greet(name: string):
  ...
```

### Parameter with Default Value

```cortada
fn connect(port = 8080):
  ...
```

### Typed Parameter with Default Value

```cortada
fn connect(port: int = 8080):
  ...
```

### Notes

* Parameters are local bindings belonging to the function scope.
* Type annotations are optional.
* Default values are evaluated when the function is invoked.
* Parameters are ordered and positional.

### Diagnostics

* Expected parameter name.
* Expected type after `:`.
* Expected expression after `=`.
