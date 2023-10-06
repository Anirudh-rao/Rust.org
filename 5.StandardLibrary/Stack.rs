fn foo() {
    //Y is allocated into Stack
    let y = 999;
    //Z is allocated into Stack
    let z = 333;

}

fn main() {
    //X is executed in Stack(Ram)
    let x = 111;

    //Function is Called
    foo();
    //Y and  is Deallocated
}