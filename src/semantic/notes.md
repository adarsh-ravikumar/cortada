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

the actual table, this is what i have in mind:
for now, i am not tracking functions and stuff, only variables. so:

hashmap that maps
symbol (string) -> vec<type, source span of where it is declared, source span of the symbol itself>

why the two spans? idk i feel like i might need the symbol's span for diagnosis, but the entire decleration span i know i will use in diagnosis (err: declared here as ...)

it is a vector, because i allow shadowing, and again, for diagnosis purposes, it makes sense to maintain the history. 

so i have the entire history like a stack, with the top being the latest, and the one i care about.

does that all make sense?
