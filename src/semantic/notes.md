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
[x] Function Table
[ ] Type Alias Table

Resolution
----------
[x] Name Resolution
[x] Scope Resolution
[x] Binding shadowing

Types
-----
[x] Arithmetic Expressions
[x] Boolean Expressions
[x] Assignment Checking
[x] Implicit Cast Insertion
[x] Identifier usage
[ ] Type Alias Resolution
[ ] Function Call Checking

Control Flow
------------
[x] Control Flow Annotation

Diagnostics
-----------
[x] Undefined Identifier
[x] Type Mismatch
[ ] Invalid Function Call

# ERRORS
* Circular reference

* Assignment to undecleared variable => add a note "if you are trying to declare, do :="

* Parameter name duplicate
* Function name / variable name collision
* Function call arity
* Unknown function
* Unable to determine overload
* Missing return
* Multiple incompatible returns
* Return outside function
* Break / continue outside loop 

# WARNINGS
* Unused variable
* Unused functions
* Unused parameter
* Variable shadowing (if child scope re-declares variable in parent)
* Assigned but never read 
* Value overwritten before use 
* Infinite loop
* Constant conditions
* Unreachable code
* Statement after break / continue / return
* Identifier closest match
* Recursive warning if no base condition

# REMAINING
* Pretty print tables
* Warn printing for diagnostics

* Return statements
* Break / Continue
* Control flow analysis
   - Every path returns
   - Unreachable code
   - Definite assignment

* Function calls 
* Function overloading
