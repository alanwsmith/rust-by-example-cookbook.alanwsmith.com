// https://doc.rust-lang.org/rust-by-example/conversion/from_into.html
//
// This shows how to create a custom `From` that lets
// you convert one type into another type


use std::convert::From;

struct Widget {
    value: i32
}

impl From<i32> for Widget {
    fn from(input: i32) -> Self {
        Widget { value: input}
    }
}

fn main() {
    let alfa: i32 = 123;

    let bravo = Widget::from(alfa);

    println!("My value is {}", bravo.value);
}

