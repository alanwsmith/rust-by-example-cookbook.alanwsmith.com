// https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
//
// Constants
//
// There are two different kinds of constants:
// `const` and `static`. This is `const` which is
// unchangeable and the most common case. Both
// can be declared in any scope, including global
// 
// Constands require explity type annotation. 

const ALFA: i32 = 19;

fn main() {
    println!("The value is {}", ALFA)
}

// If you tried to set the value again:
// e.g. `ALFA = 27` it would cause an error
//
// TODO: Figure out what the difference
// is between this and using an immutalbe 
// variable. Maybe that it's global?



