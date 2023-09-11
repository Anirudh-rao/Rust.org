fn main(){
    // Initialzation of an Array
    let numbers : [i32;5] = [1,2,3,4,5];
    println!("Array is {:?}", numbers); 

    // initialization of array with default values
    let numbers2: [i32; 5] = [3; 5];
    
    println!("Array of numbers = {:?}", numbers2);

     // initialize array with default values
     let numbers3 = [3; 5];

     println!("Array of numbers = {:?}", numbers3);

     // an array without data type
    let a = [5, 4, 3, 2, 1];
    
    // an array with data type and size
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    
    // an array with default values
    let c = [3; 5];
    
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    let colors = ["red", "green", "blue"];
    
    // accessing element at index 0
    println!("1st Color: {}", colors[0]);

    // accessing element at index 1
    println!("2nd Color: {}", colors[1]);

    // accessing element at index 2
    println!("3rd Color: {}", colors[2]);


    let mut numbers4: [i32; 5] = [1, 2, 3, 4, 5];
    
    println!("original array = {:?}", colors);
    
    // change the value of the 3rd element in the array
    //numbers[2] = 0;
    
    println!("changed array = {:?}", numbers4);

     // loop through an array to print its index and value
     for index in 0..3 {
        println!("Index: {} -- Value: {}", index, colors[index]);
    }
}