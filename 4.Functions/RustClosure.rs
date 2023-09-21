//In Rust, closures are functions without names.
// They are also known as anonymous functions or lambdas.

fn main() {
    // define a closure and store it in a variable
    let add_one = |x: i32| x + 1;

    // call closure and store the result in a variable
    let result = add_one(2);

    println!("Result = {}", result);
}