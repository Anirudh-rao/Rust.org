# Generics 

`Generics` is the topic of generalizing types and functionalities to broader cases. This is extremely useful for reducing code duplication in many ways, but can call for rather involved syntax. Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid. The simplest and most common use of generics is for type parameters.

A type parameter is specified as generic by the use of angle brackets and upper camel case: `<Aaa, Bbb, ...>`. "Generic type parameters" are typically represented as `<T>`. In Rust, "generic" also describes anything that accepts one or more generic type parameters `<T>`. Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic).

For example, defining a generic function named foo that takes an argument T of any type:
```
fn foo<T>(arg: T) { ... }
```
Because `T ` has been specified as a generic type parameter using <T>, it is considered generic when used here as ` (arg: T)`. This is the case even if T has previously been defined as a `struct`.

## Functions
The same set of rules can be applied to functions: a type T becomes generic when preceded by `<T>`.

Using generic functions sometimes requires explicitly specifying type parameters. This may be the case if the function is called where the return type is generic, or if the compiler doesn't have enough information to infer the necessary type parameters.

A function call with explicitly specified type parameters looks like: `fun::<A, B, ...>()`.

## Implementation
Similar to functions, implementations require care to remain generic.