# Variable Bindings
Welcome to  Chapter 4 of this Repository.
In this we will learn more about Variable Bindings like
1. Mutability
2. Scope and Shadowing
3. Declare first
4. Freezing


## Variable Bindings

Rust provides type safety via static typing. Variable bindings can be type annotated when declared. However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.

Values (like literals) can be bound to variables, using the let binding.


## Mutability

Variable bindings are immutable by default, but this can be overridden using the `mut` modifier.

## Scope and Shadowing

Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by braces {}.