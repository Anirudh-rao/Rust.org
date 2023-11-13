//function to find a user by their username
//which  returns an option type

fn get_user(username:&str) -> Option <&str> {
    if username.is_empty(){
        return None;
    }
    return Some(username);
}

fn main(){
    //returns an Option
    let user_option = get_user("Hari");

    //use of match experssion to get the result out of Option
    let result  =  match user_option{
        Some(user) => user,
        None => "not found!",
    };

    println!("user = {:?}", result);
}