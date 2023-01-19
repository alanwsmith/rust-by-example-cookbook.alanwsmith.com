// https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
//
// You can return a tuple from a funciton in 
// order to pass multiple values back. This 
// defines a function that returns a tuple 
// with two `i32` values in it

fn basic_return() -> (i32, i32) {
    (74, 83)
}

fn main() {
    println!(
        "Values {:?}", 
        basic_return()
    )
}
