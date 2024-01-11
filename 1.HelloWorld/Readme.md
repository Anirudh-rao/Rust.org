# Intdroduction:
This will be our first deep dive into rust. We will cover the following topics:

1. Hello World 
2. Comments
3. Formatted Input

Lets Start `Rusting`:

# Hello World

Welcome to Introduction to Rust Programming.
With start lets check the `HelloWorld.rs `File.

We see a Println! Function that prints some value.
`println!` is a `macro` that prints text to the console.

To Run any rust Program:

```
> rustc  helloWorl.rs
```
A binary can be generated using the Rust compiler: `rustc`.
`rustc` will produce a `hello` binary that can be executed as following:
```
$./helo
> Hello World!
```

# Comments

Any program requires comments, and Rust supports a few different varieties:

1. Regular comments which are ignored by the compiler:
   1. `//` `Line comments which go to the end of the line.
   2. `/*` `Block comments which go to the closing delimiter. */
2. Doc comments which are parsed into HTML library documentation:
   1. `///` Generate library docs for the following item.
   2. `//!` Generate library docs for the enclosing item.


# Formatted print
Printing is handled by a series of macros defined in std::fmt some of which include:

1. `format!` : write formatted text to String
2. `print!`: same as format! but the text is printed to the console (io::stdout).
3. `println!`: same as print! but a newline is appended.
4. `eprint!`: same as print! but the text is printed to the standard error (io::stderr).
5. `eprintln!`: same as eprint! but a newline is appended.

All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

`std::fmt` contains many traits which govern the display of text. The base form of two important ones are listed below:

1. `fmt::Debug`: Uses the {:?} marker. Format text for debugging purposes.
2. `fmt::Display`: Uses the {} marker. Format text in a more elegant, user friendly fashion.
Here, we used fmt::Display because the std library provides implementations for these types. To print text for custom types, more steps are required.

Implementing the fmt::Display trait automatically implements the ToString trait which allows us to convert the type to String.

In line 43, #[allow(dead_code)] is an attribute which only apply to the module after it.