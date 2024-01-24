fn main(){
    //Variables can be type annotated
    let logical:bool =  true;

    let a_float:f64 = 1.0;//Regular annotation
    let an_integer =  5i32;

    //Or default will be used
    let default_float =  3.0;
    let default_integer =  7;

    //A type can also be inferred from context
    let mut  inferred_type =  12;
    inferred_type = 4294967296i64;

    //A Mutable variable value can be changes
    let mut mutable =  12;
    mutable = 21;

    //ERROR the type of a variable cant be changes
    // mutable = true;

    //Variable can be overwritten with shadowing
    let mutable  = true;
}