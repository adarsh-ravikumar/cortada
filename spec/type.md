# Types

## Productions

```text
type_expression -> type_union

type_union      -> type_primary ("|" type_primary)*

type_primary    -> "int"
                 | "float"
                 | "bool"
                 | "(" type_expression ")"
```

---

## Referenced By

* `function.md`
* `variables.md`

## See Also

* `param`
* `var_decl`
* `fn_stmt`

# Examples

## Union Types

### Primitive Type

```cortada
int
```

```cortada
float
```

### Simple Union

```cortada
int | float
```

### Parenthesized Union

```cortada
(int | float)
```

### Nested Union

```cortada
(int | float) | int
```

### Notes

* Union types represent values which may belong to one of several types.
* The order of variants within a union has no semantic significance.
* Parentheses may be used to group type expressions.

### Diagnostics

* Expected type after `|`.
* Expected type expression.
* Expected `)` to close parenthesized type expression.

---

## Type Atoms

### Primitive Types

```cortada
int
```

```cortada
float
```

### Parenthesized Type

```cortada
(int)
```

### Nested Parenthesized Type

```cortada
((int | float))
```

### Notes

* Type atoms are the fundamental building blocks of type expressions.
* Parenthesized type expressions are semantically equivalent to the enclosed type expression.
* Type atoms form the leaves of type expressions.

### Diagnostics

* Expected type expression.
* Expected `)` to close parenthesized type expression.

