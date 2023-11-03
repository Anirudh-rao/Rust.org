use std::collections::HashMap;

fn main(){
    //create a new hashmap
    let mut info:HashMap<i32, String> = HashMap::new();

    println!("HashMap = {:?}", info);

    //Inserting Elements into HashMap
    info.insert(1, String::from("Apple"));
    info.insert(2, String::from("Banana"));

    println!("HashMap = {:?}", info);

    //Accessing Elements of HashMap
    let firstInfo = info.get(&1);
    let SecondInfo = info.get(&2);
    let ThirdInfo = info.get(&3);

    println!("First Element  = {:?}", firstInfo);
    println!("Second Element = {:?}", SecondInfo);
    println!("Third Element = {:?}", ThirdInfo);

    //Deleting elements of the HashMap
    let mut fruits: HashMap<i32, String> = HashMap::new();

    fruits.insert(1, String::from("Apple"));
    fruits.insert(2, String::from("Banana"));
    println!("fruits  = {:?}", fruits);

    //After Removing First element
    fruits.remove(&1);
    println!("fruits after remove operation = {:?}", fruits);


    // update the value of the element with key 1
    fruits.insert(1, String::from("Mango"));
    println!("fruits after inserting  operation = {:?}", fruits);

}