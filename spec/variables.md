# Variables

## Productions

```text
var_decl -> IDENTIFIER ":" type_expression? "=" expression

var_assign -> IDENTIFIER "=" expression
```

---

## References

| Document        | Productions       |
| --------------- | ----------------- |
| `type.md`       | `type_expression` |
| `expression.md` | `expression`      |

## Referenced By

* `statement.md`

## See Also

* `fn_stmt`
* `param`
* `expression`

# Examples

## Variable Declarations

### Typed Declaration

```cortada
count: int = 42
```

### Inferred Type

```cortada
pi = 3.14
```

### Union Type

```cortada
value: int | float = 42
```

### Expression Initializer

```cortada
area: float = width * height
```

### Notes

* Variables are bindings rather than mutable memory locations.
* Type annotations are optional.
* If omitted, the variable's type is inferred from the initializer.
* A variable declaration introduces a new binding into the current lexical scope.
* Variable declarations require an initializer.

### Diagnostics

* Expected variable name.
* Expected type after `:`.
* Expected `=` after variable declaration.
* Expected expression after `=`.

---

## Variable Assignments

### Simple Assignment

```cortada
count = count + 1
```

### Assignment from Expression

```cortada
distance = x * x + y * y
```

### Function Call Assignment

```cortada
result = compute()
```

### Notes

* Assignment updates the binding associated with an existing variable.
* The assigned value must be compatible with the variable's type.
* Assignment expressions evaluate the right-hand side before updating the binding.

### Diagnostics

* Expected variable name.
* Expected `=` after variable name.
* Expected expression after `=`.
* Assignment to an undefined variable.
* Cannot assign a value of incompatible type.

