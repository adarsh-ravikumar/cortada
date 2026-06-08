The symbol table has exactly two roles

a) maintain a global incrementing ID that is assigned to the entries
b) be an abstraction over all the member tables

Binding Table:
  - Table for bindings
  - Maps id (usize) -> `Vec<BindingEntry>`
  - A list is maintained so as to provide a history for shadowing of that symbol

  BindingEntry:
    - type of binding   -> usize (id for entry in Type Table)
    - deceleration span -> span of the VarDecl node
    - symbol span       -> span of the variable name, within the deceleration span

  Rules for determining the type of binding:
  - If the type is annotated (i.e `x: type = value`), `type` is used as the type 
  - If the type is not explicitly annotated (i.e `x := value`), type is inferred from `value`
  - If neither exist, i.e `x := null`, it is an erroneous decleration

Type Table:
  - Table for types
  - Maps id (usize) -> `TypeEntry`
  
  TypeEntry:
    - TypeInternal -> Internal represnetation of the type expression
    - is_builtin   -> bool
    - decleration span -> span of the type decleration (optional)
    - symbol span      -> span of the type name, within the decleration span (optional)

  TypeInternal: (enum)
    - Integer
    - Float
    - Union (Vec<TypeInternal>)

  When the table is created, all built-in entries are registered
  
  The table has two methods:

  a) `resolve_type (to_check, target)`
     Compares the type of `to_check` with `target`.

     Integer:
       to_check = target

     Float:
       to_check = target

     Union:
       target.contains(to_check)

