fn main(){
    let i = 2;
    let mut j = 1;

    while j <= 10{
        let mult = i * j;

        println!("{} * {} = {}", i, j, mult);
        j += 1; 
    }
}