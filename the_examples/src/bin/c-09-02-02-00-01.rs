// https://doc.rust-lang.org/rust-by-example/fn/closures/input_parameters.html
//
// I don't understand all this stuff yet. 
// This is straight copy from the site. 
//

// TODO: Figure out this syntax. 

fn apply<T>(f: T) where 
    T: FnOnce() {
        f();
}


fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farwell = "goodbye".to_owned();

    let dairy = || {
        println!("I said {}.", greeting);
        farwell.push_str("!!!");
        println!("Then I screamed {farwell}");
        mem::drop(farwell);
    };

    apply(dairy);

}
