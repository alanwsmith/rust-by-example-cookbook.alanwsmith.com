// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
//
//

// From the page: "The From trait allows for a type to define 
// how to create itself from another type, hence providing 
// a very simple mechanism for converting between several 
// types. There are numerous implementations of this trait 
// within the standard library for conversion of primitive 
// and common types. For example we can easily convert a 
// `str` into a `String`

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("The value is {}", my_string)
}


