# Custom Types

Welcome to 3rd Part in this repository
In this  Section we will cover custom data types in Rust like
1. Struct
2. Enums
3. constants

## Structures

There are three types of structures ("structs") that can be created using the struct keyword:

1. Tuple structs, which are, basically, named tuples.
2. The classic C structs
3. Unit structs, which are field-less, are useful for generics.


## Enums

The `enum` keyword allows the creation of a type which may be one of a few different variants. Any variant which is valid as a struct is also valid in an `enum`.

**Type aliases**

If you use a type alias, you can refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it.
```
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}
```

The most common place you'll see this is in impl blocks using the Self alias.

```
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

###  use
The use declaration can be used so manual scoping isn't needed.

### C-like
`enum` can also be used as C-like enums.


