// https://doc.rust-lang.org/rust-by-example/custom_types/enum/enum_use.html
//
// This is how to setup to use 'use' to load in
// all the enum parts of an enum with a `*`
// /

#![allow(dead_code)]

enum Alfa {
    First, 
    Second
}

fn main() {
    use crate::Alfa::*;
    let status = First;

    match status {
        First => println!("This is first"),
        Second => println!("This is second")
    }
}

