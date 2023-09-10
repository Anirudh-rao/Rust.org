fn main() {
    let number = -2;
    
    if number < 0 {
        // if outer condition evaluates to true evaluate the inner condition
        if number == -2 {
            println!("The current number is -2");
        } else {
            println!("The current number is {}", number);
        }
    }
}