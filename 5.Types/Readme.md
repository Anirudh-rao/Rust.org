# Types

Rust provides several mechanisms to change or define the type of primitive and user defined types. The following sections cover:

1. Casting between primitive types
2. Specifying the desired type of literals
3. Using type inference
4. Aliasing types


## Casting

Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.

Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust.

## Literals
Numeric literals can be type annotated by adding the type as a suffix. As an example, to specify that the literal 42 should have the type i32, write 42i32.

The type of unsuffixed numeric literals will depend on how they are used. If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers.