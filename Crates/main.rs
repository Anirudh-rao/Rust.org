extern crate rally; // May be required for Rust 2015 edition or earlier

fn main() {
    rally::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rally::indirect_access();
}