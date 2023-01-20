// https://doc.rust-lang.org/rust-by-example/conversion/string.html
//
// This is a good one to put kinda early after you've 
// shown traits. Maybe this is even a good one
// to use for the example of how traits work (or
// a very early one in that process)
//
// From the page

/*
Converting to String

To convert any type to a String is as simple as implementing the ToString trait for the type. Rather than doing so directly, you should implement the fmt::Display trait which automagically provides ToString and also allows printing the type as discussed in the section on print!.
*/

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}



// TODO: Figure out why this uses `&self` instead of `Self`
// from some of the prior examples
//
// TODO: Define what `write!` does (I think it's what
// fills in the value for a string, maybe?)
//
