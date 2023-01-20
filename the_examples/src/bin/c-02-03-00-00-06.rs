// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// Example of borrowing an array as a slice
//

fn main() {

    let lima_array: [i32; 4] = [12, 28, 36, 45];

    let lima_slice = &lima_array[1 .. 2];

    println!("Values {:?}", lima_slice);

}

// OUTPUTS
// Values [28]
//



