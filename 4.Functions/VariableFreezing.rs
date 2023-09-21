fn main() {
    let mut age = 100;

    {
        // shadowing by immutable age variable
        let age = age;

        println!("age variable inner block = {}", age);
        // age goes out of scope
    }

    // age variable is not frozen in this scope
    age = 3;

    println!("age variable outer block = {}", age);
}