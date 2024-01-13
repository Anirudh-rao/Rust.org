// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

enum WebEvent{
    //An Enum variant may either be 'unit-like',
    PageLoad,
    PageUnload,
    //like tuple structs
    KeyPress(char),
    Paste(String),
    //or c-like structures
    Click{x:i64, y:i64},
}

//A Function which takes a 'WebEvent' enum as an Argument and returns nothing
fn inspect(event:WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page Loded"),
        WebEvent::PageUnload => println!("Page Unloaded"),
        //Destructure 'c; from inside the 'enum' variant,
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main(){
    let pressed =  WebEvent::KeyPress('x');
    //to_Ownded() created an owned string from string slice.
    let pasted =  WebEvent::Paste("my text".to_owned());
    let click =  WebEvent::Click{x:20,y:30};
    let load =  WebEvent::PageLoad;
    let unload =  WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}