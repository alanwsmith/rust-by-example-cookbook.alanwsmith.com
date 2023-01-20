// https://doc.rust-lang.org/rust-by-example/custom_types/enum/enum_use.html
//
// This is an example of how to use the `use` call
// to not have to use the first part of a name. This 
// example uses each name specifically. 

#![allow(dead_code)]

enum Alfa {
    First,
    Second
}

fn main() {
    use crate::Alfa::{First, Second};
    let status = First;

    match status {
        First => println!("This is first"),
        Second => println!("This is second")
    }
}


