fn main() {
    // string creation using String::from() method
    let word = String::from("Hello, World!");

    println!("word = {}", word);
    //Mutable String
    let mut word2 = String::from("cat");

    println!("original string = {}", word2);

    // push a new string at the end of the initial string
    word2.push_str(" dog");

    println!("changed string = {}", word2);

    // slicing a string
    let slice = &word[0..5];

    println!("string = {}", word);
    println!("slice = {}", slice);

    // Loop through each character in a string using chars() method
    for char in word2.chars() {
        println!("{}", char);
    }
}