# Expressions

## Productions

```text
expression            -> or_expression

or_expression         -> and_expression ("or" and_expression)*

and_expression        -> not_expression ("and" not_expression)*

not_expression        -> "not" not_expression
                       | boolean_expression

boolean_expression    -> arithmetic_expression
                         (("<" | "<=" | ">" | ">=" | "==")
                         arithmetic_expression)*

arithmetic_expression -> term
                         (("+" | "-") term)*

term                  -> factor
                         (("*" | "/") factor)*

factor                -> ("+" | "-") factor
                         | postfix

postfix               -> atom (call_suffix)*

call_suffix           -> "(" (expression ("," expression)*)? ","? ")"

atom                  -> INTEGER
                       | FLOAT
                       | IDENTIFIER
                       | "true"
                       | "false"
                       | "null"
                       | "(" expression ")"
```

---

## Referenced By

* `statement.md`
* `variables.md`
* `function.md`
* `control_flow.md`
* `postfix.md`

## See Also

* `type_expression`
* `call_suffix`

# Examples

## Binary Expressions

### Arithmetic Expressions

```cortada
a + b

x - y

width * height

distance / 2
```

### Comparison Expressions

```cortada
age >= 18

x < y

name == other_name
```

### Logical Expressions

```cortada
is_valid and is_connected

x > 0 or y > 0
```

### Compound Expressions

```cortada
(a + b) * c

count > 0 and is_running

x < y or z > 3 and enabled
```

### Notes

* Binary operators are evaluated according to operator precedence.
* Operators of equal precedence are left-associative.
* Parentheses may be used to override precedence.

### Diagnostics

* Expected expression after operator.
* Expected expression before operator.
* Expected `)` to close parenthesized expression.

---

## Unary Expressions

### Arithmetic Negation

```cortada
-value

+x
```

### Logical Negation

```cortada
not is_valid

not (x > 0)
```

### Notes

* Unary operators have higher precedence than binary operators.
* Unary operators are right-associative.

### Diagnostics

* Expected expression after unary operator.

---

## Postfix Expressions

### Function Call

```cortada
print()
```

### Function Call with Arguments

```cortada
max(a, b)

distance(x1, y1, x2, y2)
```

### Nested Calls

```cortada
foo()(x)

bar(a)(b, c)
```

### Notes

* Postfix operators bind more tightly than unary and binary operators.
* Trailing commas are permitted in argument lists.
* Function calls associate from left to right.

### Diagnostics

* Expected expression after `,`.
* Expected `)` to close argument list.

---

## Atoms

### Literals

```cortada
42

3.14

null
```

### Identifier

```cortada
player_name
```

### Parenthesized Expression

```cortada
(x + y)

(a and b)
```

### Notes

* Atoms are the fundamental building blocks of expressions.
* Parenthesized expressions evaluate to the value of the enclosed expression.

### Diagnostics

* Expected expression.
* Expected `)` to close parenthesized expression.

