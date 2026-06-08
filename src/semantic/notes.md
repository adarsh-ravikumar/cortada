goals:

1)  build the symbol tree given an AST
      -> walk the tree. if ident node or assign node, see if it already exists, other wise err. if decl node, then create it.

2) resolve type by walking expressions
      -> add to the previous process by evaluating the expression and determine if the variable is being used in the right context / infer type of symbol

3) resolve scope
       -> finally, address scopes. functions vs control flow vs global etc.

to summarize, it is
1 answers "does it exist?"
2 answers "if it exists, is it used correctly?"
3 answers "does it exist, given the current scope?"

# TODO
Declarations
-----------
[x] Symbol Table
[x] Binding Table
[ ] Function Table
[ ] Type Alias Table

Resolution
----------
[ ] Name Resolution
[ ] Scope Resolution
[ ] Binding shadowing

Types
-----
[x] Arithmetic Expressions
[x] Boolean Expressions
[x] Assignment Checking
[x] Implicit Cast Insertion
[ ] Identifier usage
[ ] Type Alias Resolution
[ ] Function Call Checking

Control Flow
------------
[ ] Control Flow Annotation

Diagnostics
-----------
[x] Undefined Identifier
[x] Type Mismatch
[ ] Invalid Shadowing (if applicable)
[ ] Invalid Function Call


# order
1) identifiers
2) scope
3) shadowing
4) control flow
5) functions 
6) function calls
7) type alias
