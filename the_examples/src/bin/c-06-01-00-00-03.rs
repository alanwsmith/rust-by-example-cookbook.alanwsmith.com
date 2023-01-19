// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
//
// This is how 'Into' gets setup
//
// From the page: 
//
// "The Into trait is simply the reciprocal of 
// the From trait. That is, if you have implemented 
// the From trait for your type, Into will call it 
// when necessary."
//
// "Using the Into trait will typically require 
// specification of the type to convert into as the 
// compiler is unable to determine this most of the 
// time. However this is a small trade-off considering 
// we get the functionality for free."
//
// NOTE: The `->` syntax hasn't been described yet.

use std::convert::From;

struct Widget {
    value: i32
}

impl From<i32> for Widget {
    fn from(input: i32) -> Self {
        Widget {
            value: input
        }
    }
}

fn main() {
    let alfa = 456;

    let bravo: Widget = alfa.into();

    println!("The value is {}", bravo.value)
}

// TODO: Look up to see how much `from` is 
// used compared to `into`. It feels like 
// from makes a little more sense becuase 
// you can see the incoming type a little 
// more easily. But TBD on that. 



