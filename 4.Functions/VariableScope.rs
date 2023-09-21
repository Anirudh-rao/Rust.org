fn main(){
    let random = 100;

    //Start of Inner Block
    {
        println!("random variable before shadowing in inner block = {}", random);

        // this declaration shadows the outer random variable
        let random = "abc";

        println!("random after shadowing in inner block = {}", random);
    }
    //End of Inner Block
    println!("random variable in outer block = {}", random);
}