fn main(){
    let mut counter = 1;
    while counter < 5 {
        println!("We loop a while...");
        let counter = counter + 1;
        break;
    }

    //For loops using iter
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }
    //range notation in rust a..b 
    for number in 0..5 {
        println!("{}", number * 2);
    }
}