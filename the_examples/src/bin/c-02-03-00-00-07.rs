// https://doc.rust-lang.org/rust-by-example/primitives/array.html
//
// This is an example of an empty slice. There's
// no context on when you'd want that. So, this is something
// to punt down the road. 

// The code is straight from the example page, but 
// I'm not sure what it does or how `assert_eq!` works
// I tried println!("{:?}", assert...) but it printed
// and empty `()` so not sure what's up with that. 

fn main() {
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose
}
