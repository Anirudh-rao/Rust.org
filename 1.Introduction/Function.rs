fn add(a:u32, b:u32) -> u32 {
    return a + b;
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

fn goodbye(message: &str) {
    println!("\n{}", message);
}


fn main(){
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let a = 1;
    let b = 2;
    println!("{} + {} = {}", a,b,add(1,2));

    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));

}