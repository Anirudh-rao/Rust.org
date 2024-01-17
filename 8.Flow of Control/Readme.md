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

## for loops

### for and range

The for in construct can be used to iterate through an Iterator. One of the easiest ways to create an iterator is to use the range notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one.

### for and iterators
The for in construct is able to interact with an Iterator in several ways. As discussed in the section on the Iterator trait, by default the for loop will apply the into_iter function to the collection. However, this is not the only means of converting collections into iterators.

`into_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.

1. `iter` - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

2. `into_iter` - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.

3. `iter_mut` - This mutably borrows each element of the collection, allowing for the collection to be modified in place\


## match
Rust provides pattern matching via the match keyword, which can be used like a C switch. The first matching arm is evaluated and all possible values must be covered.


### Destructuring
A match block can destructure items in a variety of ways.

1. `tuples`
2. `Arrays/Slices`
3. `Enum`
4. `Pointers/ref`
5. `structs`


### Binding
Indirectly accessing a variable makes it impossible to branch and use that variable without re-binding. match provides the `@` sigil for binding values to names: