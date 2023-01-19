// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// Access individual elements of an array by index. 
//
// Indexes start at 0
//
// TODO: Show exmaples of zero based index.
// 
// This uses `{}` instead of `{:?}` since it can get
// to the indivdual number. That probably depends on
// the value stored in the array though. 

fn main() {

    let delta_array: [i32; 4] = [14, 28, 34, 47];

    println!("Index 1 Value {}", delta_array[1]);

}

// OUTPUT
// Index 1 Value 28
//
