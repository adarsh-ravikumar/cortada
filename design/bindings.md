# Bindings
Bindings (variables) are staticly typed.

# Semantics
- Initialize always creates a new binding in the local scope only.
- Access will always read the binding in the nearest (local or ancestor) scope containing a definition for that binding  only.
- Assignment will always assign to the nearest (local or ancestor) scope containing an existing declaration for that binding only.
- Assignment across closure boundaries is erroneous unless the binding is explicitly captured.
- Shadowing will occur only if a declared binding has the same name as an existing binding in the local scope only.
- Shadowing drops the existing binding before creating the new binding.

# Notes

### Initialize
To initialize a variable, use the `var_name: T = value` syntax
- To create a variable without a value, i.e to declare a variable, do `var_name: T` or `var_name: auto`
- To create a variable with a value but inferred type, do `var_name: auto = value` or `var_name := value`

### Access & Assign
- Access is done using syntax `var_name`
- Assign is done using the `var_name = delta` syntax.
  - If the variable `var_name` was created with syntax `var_name: T = value`, then `delta` can only be of the type `T`

  - If the variable was created using the `var_name := value` syntax, i.e variable was declared with some initial value without specifying type, the type of `value`, then the type is auto inferred to be the same as that of `value`, say `T'`. `delta` is expected to have the same type, `T'`. Any other type is invalid

  - If the variable was created with syntax `var_name: T`, i.e, variable was only decalred to be of type `T` and not set a default value, `delta` can only be of the type `T`

  - If the variable was created with syntax `var_name: auto`, i.e variable was only decalred with the type to be auto inferred:
      - If this is the first assignment after the deceleration, the variable will now be set to have the same type as `delta`, say `T'`
      - If this is NOT the first assignment, `delta` is expected to have the type `T'`. Any other type is invalid

- Cortada is a block-scoped language. Meaning, every block regardless of being a function or a control-block, creates its own lexical scope.
- Structural control-flow blocks (`if`, `for`, `while`, etc.) may naturally access and assign variables from ancestor scopes.
- Closure boundaries (`fn`, lambdas, async blocks, etc.) do not implicitly inherit mutable access to ancestor scopes.
- This affects how assignments and access work. For example:

    ```
    x := 3.14     ~ decl.. 1
    y := 5        ~ decl.. 2

    fn foo:
      x := 5      ~ decl.. 3
      x = 3       ~ assi.. 1
      y = 2       ~ assi.. 2

      if x > 2:   ~ access 1
        print(y)  ~ access 2

      print(x)    ~ access 3
    ```

- In this example, we have two variables, `x` (float) and `y` (int) declared with some initial value (`3.14` and `5` respectively) in the global scope.
- Inside the function `foo`, we declare variable `x`. This is a completely different variable from that declared in `decl.. 1`
- This is because, `foo` creates a closure boundary and its own scope
- `assi.. 1` then assigns this variable `x` in `foo`'s scope the value `3`
- `assi.. 2` however, attempts to perform assignment on variable `y`
- Such a variable does not exist in the current scope
- The compiler then looks at the parent scope, which in this case, is the global scope.
- It discovers the variable `y` decalred through `decl.. 2` and assigns that the new value of `2`
- Notice how the global scope `x` through `decl.. 1` has type `float` and the one inside `foo` from `decl.. 3` has type `int`. These are two seperate entities in memory.
- Notice how the `assi.. 2` assigns an integer to the `y` decalred in the global scope through `decl.. 2`. The variable is inferred to be as type `int` and hence assignments to that variable must be of type `int`
- The `if` statement now creates a new structural scope of its own.
- Now, we perform `access 1`. This accesses the variable `x` declared in `foo`'s scope through `decl.. 3`, and NOT the one declared in the global scope through `decl.. 1`
- `access 2` however yields no variable `y` inside of the `if` statement's scope. So we go up a scope. Again, we do not find it and hence go up another scope. We finally find `y` in the global scope declared through `decl.. 2`. This is the variable we end up accessing
- Finally, we return back to the scope of `foo` to then perform `access 3` which again refers to the `x` declared within `foo`'s scope.


### Shadowing
- If a variable `var_name` was declared in the current scope with type `T`, and there is a need for `var_name` to be of a different data type `T'`,
the variable can be re-declared with the new type `T'`. The variable is hence shadowed.
- It is allowed for `T'` to be the same as `T`. The behavior is the exact same as if they were different.
- Only variable within the same scope as the second decleration will be shadowed.
- Take for example:


```
fn foo:
  x := 5      ~ decl.. 1
  x = 3       ~ assi.. 1

  if x > 2:   ~ access 1
    x := 4.5  ~ decl.. 2
    print(x)  ~ access 2

  x := 3.14   ~ decl.. 3

  print(x)    ~ access 3
```

- `foo` creates a scope for itself. `decl.. 1` creates `x` of inferred type `int` in this scope with the value `5`
- `assi.. 1` then assigns to this `x` in `foo`'s scope
- The `if` statement creates a structural scope of its own.
- `access 1` reads variable `x` created in `foo`'s scope through `decl.. 1`
- `decl.. 2` creates a new variable `x` in the scope of the `if` statement. The vairable `x` in the parent scope continues to stay alive. It is *NOT* shadowed
- `access 2` accesses `x` declared in the `if` statement's scope through `decl.. 2`.
- We now arrive back at the scope of `foo`
- `decl.. 3` attempts to declare a variable `x` with inferred type `float`.
- But, a variable of the same name `x` exists in `foo`'s scope with type `int`
- We hence *shadow* `x`
- This means, the existing variable `x` declared in `decl.. 1` is first dropped, and a new one of type `float` is created with the same name `x`
- The variable `x` declared through `decl.. 1` in this scope is now dead
- All future accesses / assigns within this scope will refer to the `x` declared through `decl.. 3` and not `decl.. 1`
- `access 3` reads the value of `x` in `foo`'s scope, the one created in `decl.. 3` with type `float`

