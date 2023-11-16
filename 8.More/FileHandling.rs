use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs;
use std::fs::OpenOptions;



fn main(){
    //Open File in read only Mode with Local System File
    let data_result =  File::open("data.txt");

    //Reading a file returns a result enum
    //Result cann be a file or an error
    let data_file = match data_result{
        Ok(file) => file,
        Err(error) => panic!("Problem Opening the File with err:{:?}", error),
    };
    println!("Data File :{:?}", data_file);

    // Read a file in the local file system
    let mut data_file = File::open("data.txt").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);

    // Create a file
    let mut data_file = File::create("data.txt").expect("creation failed");

    // Write contents to the file
    data_file.write("Hello, World!".as_bytes()).expect("write failed");

    println!("Created a file data.txt");
    // Remove a file
    fs::remove_file("data.txt").expect("could not remove file");

    println!("Removed file data.txt");

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
}