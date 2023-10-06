/*Vector is a dynamic (resizable) data structure
that can store a list of elements of the same type.
Being a resizable data structure, vectors can grow and shrink at runtime.
*/

fn main() {
    // vector creation with vec! macro
    let v = vec![1, 2, 3];

    println!("v2= {:?}", v);
    let colors = vec!["blue", "red", "green"];

    // method 1: access vector elements using vector index
    println!("first color = {}", colors[0]);
    println!("second color = {}", colors[1]);
    println!("third color = {}", colors[2]);

    // method 2: access vector elements using get() method and vector index
    println!("first color = {:?}", colors.get(0));
    println!("second color = {:?}", colors.get(1));
    println!("third color = {:?}", colors.get(2));

    let mut even_numbers = vec![2, 4, 6, 8, 10];

    println!("original vector = {:?}", v);

    // push values at the end of the vector
    even_numbers.push(12);
    even_numbers.push(14);

    println!("changed vector = {:?}", v);

    // remove value from the vector in its second index
    even_numbers.remove(2);

    println!("changed vector = {:?}", even_numbers);


    // loop through a vector to print its index and value
    for index in 0..3 {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }

    // vector creation with Vec::new() method
    let mut v: Vec<i32> = Vec::new();

    // push values to a mutable vector
    v.push(10);
    v.push(20);

    println!("v = {:?}", v);
}
