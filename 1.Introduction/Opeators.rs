fn main(){
    let a = 20;
    let b = 2;

    // Addition in Rust
    let add = a +b;
    println!("{} + {} = {}", a,b, add);

    // Subtraction in rust
    let sub =  a - b;
    println!("{} - {} = {}", a,b, sub);

    //Mulitiplication in Rust
    let mul = a * b;
    println!("{} * {} = {}", a,b, mul);

    let dividend = 21;
    let divisor = 8;

    // arithmetic division using / operator with integers
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    let dividend2 = 21.0;
    let divisor2 = 8.0;

    // arithmetic division using / operator with floating point values
    let division2 = dividend2 / divisor2;

    println!("{} / {} = {}", dividend2, divisor2, division2);


    let remainder = dividend % divisor;
  
    println!("{} % {} = {}", dividend, divisor, remainder);
}