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

## Closures
Closures are functions that can capture the enclosing environment. For example, a closure that captures the x variable:
```
|val| val + x
```
The syntax and capabilities of closures make them very convenient for on the fly usage. Calling a closure is exactly like calling a function. However, both input and return types can be inferred and input variable names must be specified.

Other characteristics of closures include:

1. using || instead of () around input variables.
2. optional body delimitation ({}) for a single expression (mandatory otherwise).
3. the ability to capture the outer environment variables.

### Capturing
Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation. This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing. Closures can capture variables:

1. by reference:` &T`
2. by mutable reference: `&mut T`
3. by value: `T`
They preferentially capture variables by reference and only go lower when required.

## Higher Order Functions
Rust provides Higher Order Functions (HOF). These are functions that take one or more functions and/or produce a more useful function. HOFs and lazy iterators give Rust its functional flavor.