fn main() {
    let mut number = 0;

    loop {
        number += 1;

        // condition to skip the iteration
        if number == 3 {
            continue;
        }
        
        // condition to exit the loop
        if number > 5 {
            break;
        }
        
        println!("{}", number);
    }
}