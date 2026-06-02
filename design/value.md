# Value

A value is a semantically meaningful piece of data

Bindings merely referes to that value

A value has the following properties:

1) Type
    A value can either be of a fundamental type, container type, or compound type instance

2) Ownership
    A value in Cortada is owned by one binding at any given time
    To own a value means to assume all responsibility over the life cycle of the value
    A value can be in one of two states.
    - A value is bound if it is assigned to a binding (variable)
    - The lifetime and other properties of such a value will then be dependant on the scope behaviour of the binding itself
    - A value is a transient if is not bound to a binding
    - In such a case, the value is either copied or moved based on its type and other properties

```
  fn bar (y: int) -> int:
    return y ** 2   ~ the expression y ** 2 yields a value that is a transient

  fn foo:
    x := 5   ~ value 5 of type int is bound to binding x
    ~ x now owns that value 5

    z := bar(x)  ~ the transient value returned by the function now gets bound to z
    ~ that value is now owned by z
```

3) Lifetime
    A value has a lifetime, beyond which it is dropped.
    A transient is either copied or moved
    A bound value exists, at the minimum, till the last use of it in the local scope
    This means that if a binding is not moved out elsewhere, the earliest that it can be dropped is after the last use in the local scope 
    But, it can live longer if it is moved away to a seperate scope


```
  fn bar (y: int) -> int:
    ~ the x that was passed arrives here
    ~ y is now resposible for whatever value was in x, which in this case is the int 5

    a := 2  ~ the binding 'a' holds some value
    print(a) ~ it is last used here
    ~ hence, a is dropped right here

    some_other_func()
    some_more_functions()
  
    return y ** 2   ~ the expression y ** 2 yields a value that is a transient

    ~ we see that the expression y ** 2 got us a new transient that was returned
    ~ but the binding y itself is not passed anywhere
    ~ this means, y can safely die here
    ~ hence, y is dropped at this point

  fn foo:
    x := 5   ~ value 5 of type int is bound to binding x
    ~ x now owns that value 5

    z := bar(x)  ~ the transient value returned by the function now gets bound to z
    ~ that value is now owned by z

    ~ notice how the variable x is not used anywhere after the bar call
    ~ this implies that the ownership that x holds can be transfered over to bar
    ~ so x is no longer a valid binding from this point on

    print(z)

    ~ the scope ends here, and z is dropped
```

4) Mutability
    All values and bindings are mutable by default
    There are no explicit const declerations in Cortada

5) Identity
    Fundamental types DO NOT have an identity
    i.e:
      ```
      x := 5
      y := 5
      ```
    both the value `5`s are equal 

    But, all containers and compound types have an identity
    i.e:
    ```
      x := [3.14, 5]
      y := [3.14, 5]

      upper = random.range(0, 12)

      for i in 0..upper:
        x.push(4)
        y.push(2)

    ```
    both the values `[3.14, 5]` are different heap allocations
    and hence are not the same

    Note: here, optimizations do not occur as the maximum size of arrays are unknown, and hence they live on the heap


6) Copyability
    For function calls, when a bidning is used as an argument:
      All values of a fundamental type are copied
    
7) Movability
    For function calls, when a bidning is used as an argument:
      All values of a container or compound type are moved if
      they are no longer used after that point in the scope (i.e after their last use)
      Otherwise, they are passed by reference.

    ```
    fn foo (name: String):
      print(`Name is ${name}`)

      ~ name (x) will not / cannot be dropped as it is a reference

    fn bar (scores: int[]):
      for score in score:
        print (score)

      ~ y (score) will be dropped here as it was moved, and this is the end of scope
      
    fn main:
      x := "Hello world!"                 ~ new String is created and is bound to the binding x

      y := [30, 60, 20, 10, 50]           ~ new Array is created and is bound to the binding y
      
      foo (x)                             ~ reference to x is passed here, as x is used later as well
      bar (y)                             ~ y is moved as it is no longer needed in this scope

      print (x)
    ```

8) Destruction
    A value is destructed upon the end of its lifetime
    On compound types, `on_drop` function is called (if defined)
    All heap allocated resources are freed

9) Value semantics
    Cortada follows value semantics as opposed to reference semantics
    This means, binding aliasing DOES NOT exist

    ```
    x := [5, 7]
    y := x
    ```

    in this case, y is NOT an alias for `x`

    This is only for assignment and initilalizing operations

    For function calls, value is either moved or referenced based on context, as discussed before


A more indepth discussion on copy, move, reference, ownership, and lifetimes are discussed in their own documents

