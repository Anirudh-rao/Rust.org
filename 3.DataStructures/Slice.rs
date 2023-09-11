fn main(){
    // A Rust slice is a data type used to access portions of data 
    // stored in collections like arrays, vectors and strings.
    let numbers = [1,2,3,4,5];

    // Create a slice
    let slice = &numbers[1..3];
    let slice2 = &numbers[2..];
    let slice3 = &numbers[..];

    println!("array = {:?}", numbers);
    println!("slice = {:?}", slice);
    println!("Slice2 = {:?}", slice2);
    println!("slice3 = {:?}", slice3);

    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];
    
    println!("array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];
    
    println!("original slice = {:?}", sliced_colors);

    // change the value of the original slice at the first index
    sliced_colors[1] = "purple";

    println!("changed slice = {:?}", sliced_colors);
}