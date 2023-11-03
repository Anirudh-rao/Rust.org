//Import hashset from Rust Standard Collections Library
use std::collections::HashSet;

fn main(){
    //Create a New Hashset
    let mut color : HashSet<String> =  HashSet::new();

    println!("HashSet = {:?}", color);

    let mut colors: HashSet<&str> = HashSet::new();

    // insert elements to hashset
    colors.insert("Red");
    colors.insert("Yellow");
    colors.insert("Green");

    println!("Colors after Insertions:{:?}", colors);

    // check for a value in a HashSet
    if colors.contains("Red") {
        println!("We have the color \"Red\" in the HashSet.")
    }

    println!("colors before remove operation = {:?}", colors);

    // remove value from a HashSet
    colors.remove("Yellow");

    println!("colors after remove operation = {:?}", colors);

    // iterate over a hashset
    for color in colors {
        // print each value in the hashset
        println!("{}", color);
    }

}