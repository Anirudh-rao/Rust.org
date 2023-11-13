// function to find a user by their username which return an Option enum
fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }

    return Some(username);
}

fn main() {
    // use of unwrap method to get the result of Option enum from get_user function
    let result = get_user("Hari").unwrap();

    // print the result
    println!("user = {:?}", result);
}