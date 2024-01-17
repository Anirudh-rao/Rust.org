fn main(){
    let mut count = 0u32;

    println!("Lets Count until Infinity");

    // Infinite loop 
    loop{
        count += 1;

        if count ==3 {
            println!("Three");
            continue;
        }
        println!("{}",count);
        if count == 5 {
            println!("OK Thatts Enough");
            break;
        }
    }
}