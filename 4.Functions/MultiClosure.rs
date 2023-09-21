fn main(){
    //Define a Multi Line Clousre
    let squared_sum = |x:i32,y:i32| {
        //find the sum of two parameters
        let mut sum :i32 = x + y;

        //find the squared value of the sum
        let mut result:i32 =  sum * sum;

        return result
    };
    //Call Closure
    let result =  squared_sum(5,3);
    println!("Result = {}", result);
}