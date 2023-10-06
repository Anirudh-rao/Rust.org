fn main() {
    //BOX<T> is used to Allocate in Heap
    let x = Box::new(100);
    let y = 222;

    println!("x = {}, y = {}", x, y);
}