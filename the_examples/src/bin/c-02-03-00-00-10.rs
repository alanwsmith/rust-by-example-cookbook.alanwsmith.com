// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// This intentially calls an index that's
// not in an array to show the crash
//

fn main() {
    let bravo: [i32; 4] = [16, 23, 35, 41];

    println!("This won't show up {}", bravo[5])
}


// The error produced is:

/*
error: this operation will panic at runtime
 --> src/bin/c-02-03-00-00-10.rs:9:39
  |
9 |     println!("This won't show up {}", bravo[5])
  |                                       ^^^^^^^^ index out of bounds: the length is 4 but the index is 5
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: could not compile `the_examples` due to previous error
*/

