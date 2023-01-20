// https://doc.rust-lang.org/rust-by-example/fn/closures.html
//
// Closures
//


fn main() {

    let alfa = |value: i32| -> i32 {
        value + 1
    };

    let charlie = alfa(7);

    println!("The value of chralie is {charlie}")

}


// This is an example with explicit input
// and output types. The next example
// is implicit. 
