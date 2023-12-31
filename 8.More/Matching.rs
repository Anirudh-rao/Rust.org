fn main(){
    let x = 2;

    //Matching Enum
    enum color{
        Red,
        Green,
        Blue,
    }

    // use of match expression to pattern match against variable x
    match x {
        1 => println!("X is 1"),
        2 => println!("X is 2"),
        _ => println!("X is something else"),
    }
    let my_color = color::Green;

    // use of match expression to match against an enum variant
    match my_color {
        color::Red => println!("The color is red"),
        color::Green => println!("The color is green"),
        color::Blue => println!("The color is blue"),
    }
    let my_option : Option<i32> = Some(222);

    match my_option {
        Some(value) => println!("The option has a value of {}", value),
        None => println!("The option has no value"),
        }

    let my_result: Result<i32, &str> = Ok(100);

    // use of match expression to match Result type
    match my_result {
        Ok(value) => println!("The result is {}", value),
        Err(error) => println!("The error message is {}", error),
    }

    //If let expression
    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }
}