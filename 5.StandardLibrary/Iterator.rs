// An iterator in Rust is responsible
// for creating a sequence of values and allows us to iterate over each item of the sequence.
fn main(){
    let numbers = [2,1,17,8,99,34,56];

    //iteratorr
    let number_iterator = numbers.iter();
    for number in number_iterator {
        println!("{}", number);
    }

    let colors = vec!["Red", "Yellow", "Green"];

    // iterator
    let mut colors_iterator = colors.iter();
    println!("colors iterator = {:?}", colors_iterator);

    // fetch values from iterator one by one using next() method
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    println!("{:?}", colors_iterator.next());
    // using iter() to iterate through a collection
    for color in colors.iter() {
        // reference to the items in the iterator
        println!("{}", color);
    }

    // the collection is untouched and still available here
    println!("colors = {:?}", colors);

    // using into_iter() to iterate through a collection
    for color in colors.into_iter() {
        // the items in the collection move into this scope
        println!("{}", color);
    }
    // end of scope of for loop

    // error
    // the collection is not available here as the for loop scope ends above
    //println!("colors = {:?}", colors);
}