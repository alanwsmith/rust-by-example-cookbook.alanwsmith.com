// 
//
// This example is directly from the page
//

#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

}
