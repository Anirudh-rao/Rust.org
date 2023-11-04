use std::fs::File;

fn main(){
    let data_result = File::open("data.txt");
    let text = "Hello, World!";

    let character_option = text.chars().nth(15);

    //Using match for result type
    let data_file =  match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem openning the data file:{:?}", error),
    };
    // using match for Option type
    let character = match character_option {
        None => "empty".to_string(),
        Some(c) => c.to_string()
    };
    println!("Character at index 15 is {}", character);
    println!("Data file :{:?}",  data_file);
}