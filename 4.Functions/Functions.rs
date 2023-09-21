//Function to Greet
fn greet(){
    println!("Hello World");
}

//Function to Add Numbers
fn Add(){
    let a = 5;
    let b = 10;

    let sum  = a +b;
    println!("Sum of a and b = {}", sum);
}

//Function with Input Parmeters
fn Add1(a:i32,b:i32){
    let sum =  a +b;
    println!("Sum of a and b = {}", sum);
}

//Function with Input and Return Parmeters
// define an add function that takes in two parameters with a return type
fn Add2(a: i32, b: i32) -> i32 {
    let sum = a + b;

    // return a value from the function
    return sum;
}

fn main(){
    greet();
    Add();
    Add1(3,5);

    let sum = Add2(4,5);

    println!("Sum = {}",sum);
}