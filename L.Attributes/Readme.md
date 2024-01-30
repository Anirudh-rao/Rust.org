# Attributes
An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:

1. conditional compilation of code
2. set crate name, version and type (binary or library)
3. disable lints (warnings)
4. enable compiler features (macros, glob imports, etc.)
5. link to a foreign library
6. mark functions as unit tests
7. mark functions that will be part of a benchmark
8. attribute like macros
Attributes look like #[outer_attribute] or #![inner_attribute], with the difference between them being where they apply.

1.  `#[outer_attribute]` applies to the item immediately following it. Some examples of items are: a function, a module declaration, a constant, a structure, an enum. Here is an example where attribute #[derive(Debug)] applies to the struct Rectangle:
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```
2. `#![inner_attribute]` applies to the enclosing item (typically a module or a crate). In other words, this attribute is intepreted as applying to the entire scope in which it's place. Here is an example where #![allow(unusude_variables)] applies to the whole crate (if placed in main.rs):
```
#![allow(unused_variables)]

fn main() {
    let x = 3; // This would normally warn about an unused variable.
}
```
Attributes can take arguments with different syntaxes:

1. #[attribute = "value"]
2. #[attribute(key = "value")]
3. #[attribute(value)]
Attributes can have multiple values and can be separated over multiple lines, too:
```
#[attribute(value, value2)]


#[attribute(value, value2, value3,
            value4, value5)]
```

## dead_code
The compiler provides a dead_code lint that will warn about unused functions.
An attribute can be used to disable the lint.

## Crates
The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library), and the crate_name attribute can be used to set the name of the crate.

However, it is important to note that both the crate_type and crate_name attributes have no effect whatsoever when using Cargo, the Rust package manager. Since Cargo is used for the majority of Rust projects, this means real-world uses of crate_type and crate_name are relatively limited.

## cfg
Configuration conditional checks are possible through two different operators:

the cfg attribute: #[cfg(...)] in attribute position
the cfg! macro: cfg!(...) in boolean expressions
While the former enables conditional compilation, the latter conditionally evaluates to true or false literals allowing for checks at run-time. Both utilize identical argument syntax.

cfg!, unlike #[cfg], does not remove any code and only evaluates to true or false. For example, all blocks in an if/else expression need to be valid when cfg! is used for the condition, regardless of what cfg! is evaluating.