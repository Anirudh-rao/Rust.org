fn main() {
    let mut a1 = 2;
  
    // arithmetic addition and assignment
    a1 += 3;

    println!("a = {}", a1);


    let a = 7;
    let b = 3;
    
    // use of comparison operators
    let c = a > b;
    let d = a < b;
    let e = a == b;
    
    println!("{} >= {} is {}", a, b, c);
    println!("{} <= {} is {}", a, b, d);
    println!("{} == {} is {}", a, b, e);

    // Logical Operators
    let t =  true;
    let f =  false;

    // And
    let d = t && f;

    //OR
    let o = t || f;

    // Not
    let e = !t;
}