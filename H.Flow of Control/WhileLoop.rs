fn main(){
    let mut n = 1;

    while n < 10{
        if n % 5 == 0{
            println!("Fizzbuszz");
        }else if n % 3 == 0{
            println!("Fizz");
        }else if n % 5 == 0{
            println!("Buzz");
        }else{
            println!("{}", n);
        }
        n += 1;
    }
}