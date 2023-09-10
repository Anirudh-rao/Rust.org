fn main() {
    // outer loop counter
    let mut i = 1;
    
    // outer loop
    while i <= 5 {
        // inner loop counter
        let mut j = 1;
        
        // inner loop
        while j <= 5 {
            print!("*");
            
            // increase inner loop counter
            j += 1;
        }
        
        println!("");
        
        // increase outer loop counter
        i += 1;
    }
}