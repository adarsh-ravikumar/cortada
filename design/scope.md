# Scope

- Cortada is a block-scoped language
- This means, every block (indent) creates its own lexical scope
- There are two types of scope: Control-flow scopes and Closure scopes

## Control-flow Scope
- A control-flow scope, such as `if`, `for`, `while`, etc. creates its own lexical scope
- Initializing a binding within this type of scope makes it accessible only within the scope. It is not accessible from the parent scope.
- Access and assignment operations first attempt to resolve the binding within the local scope
- If the binding is not found in the current scope, lexical lookup continues recursively upward through ancestor scopes until a matching binding is found

## Closure Scope
- A closure scope, such as a nested function or lambda, also creates its own lexical scope
- Closure scopes do not implicitly inherit access to bindings from ancestor scopes. Access must be explicit.
- Any binding created within the closure cannot be accessed from the parent scope
- To allow a binding from the parent scope to be used in the local scope, one must explicitly declare intent using the `nonlocal var_name` syntax
- This captures the binding from the ancestor scope while preserving ownership and lifetime within the original scope
