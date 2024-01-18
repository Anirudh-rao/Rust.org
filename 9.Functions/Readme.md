# Functions

This Topic covers Functions in rust
We will cover topics like

1. Function
2. Associated functions & Methods
3. Closures
4. Higher Order Functions
5. Diverging Functions

## Functions
Functions are declared using the `fn` keyword. Its arguments are type annotated, just like variables, and, if the function returns a value, the return type must be specified after an arrow `->`.

The final expression in the function will be used as `return` value. Alternatively, the return statement can be used to return a value earlier from within the function, even from inside loops or if statements.

## Associated functions & Methods
Some functions are connected to a particular type. These come in two forms: associated functions, and methods. Associated functions are functions that are defined on a type generally, while methods are associated functions that are called on a particular instance of a type.