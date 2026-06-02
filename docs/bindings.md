# Bindings

Variables in Cortada behave like Bindings. This means, apart from the standard memory-location-to-variable-name mapping, a binding has the following properties:
- Owner    : Who owns a particular value, i.e who is responsible for managing the lifecycle of a value
- Borrower : Who has borrowed the value for use
- Lifetime : When will this value be dropped

To declare a binding, the following syntaxes can be used:
- `variable_name: TypeName = value`
  To explicitly declare the type of a binding

- `variable_name := value`
  To infer the type of a variable

- `variable_name: TypeName = null`
  To explicitly declare the type of a binding, and set its value to null

- `variable_name := null`
  To set the binding to null, and infer the type from the first point of use

