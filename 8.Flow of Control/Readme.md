# Flow of Control

An integral part of any programming language are ways to modify control flow: if/else, for, and others. Let's talk about them in Rust. We wil cover topics like:

1. If/Else
2. loop
3. while
4. For And Range
5. Match
6. if let
7. let-else
8. while- let


## if/else

Branching with if-else is similar to other languages. Unlike many of them, the boolean condition doesn't need to be surrounded by parentheses, and each condition is followed by a block. if-else conditionals are expressions, and, all branches must return the same type.

## loop

Rust provides a `loop` keyword to indicate an infinite loop.

The break statement can be used to exit a loop at anytime, whereas the `continue` statement can be used to skip the rest of the iteration and start a new one.


### Nesting and labels
It's possible to `break` or continue outer loops when dealing with nested loops. In these cases, the loops must be annotated with some 'label, and the label must be passed to the `break`/`continue` statement.


### Returning from loops
One of the uses of a `loop` is to retry an operation until it succeeds. If the operation returns a value though, you might need to pass it to the rest of the code: put it after the `break`, and it will be returned by the `loop` expression.

## while
The while keyword can be used to run a loop while a condition is true.

Let's write the infamous FizzBuzz using a while loop.