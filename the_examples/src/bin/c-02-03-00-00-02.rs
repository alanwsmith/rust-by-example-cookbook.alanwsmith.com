// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// Initialize all elements in an array to the
// same value


fn main() {

    let bravo_array: [i32; 6] = [45; 6];

    println!("Values {:?}", bravo_array);

}

// OUTPUTS:
// Values [45, 45, 45, 45, 45, 45]

