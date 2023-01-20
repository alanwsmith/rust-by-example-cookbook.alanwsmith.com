// https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html
//
// This shows how to cast an enuma as an 
// integer. It looks like it's done based
// off the index location. 

#![allow(dead_code)]

enum Alfa {
    Charlie, 
    Delta, 
    Echo
}

fn main() {
    let numeric_value = Alfa::Echo as i32;

    println!("The value {}", numeric_value);
}
