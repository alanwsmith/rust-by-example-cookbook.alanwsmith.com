// https://doc.rust-lang.org/rust-by-example/custom_types/constants.html
//
// This is the second kind of constant `static`. 
// Which can be declared in any scope including
// global. 
//
// The documentaiton says: static is "A possibly 
// mutable variable with 'static lifetime. The 
// static lifetime is inferred and does not have 
// to be specified. Accessing or modifying a 
// mutable static variable is unsafe."
//
// Maybe don't show static the first time throuhg. 
// Push it down the pipe a little. 

static BRAVO: i32 = 17;

fn main() {
    println!("The value is {}", BRAVO)
}

// TODO: Show what else you can do with this
// and what's possible with it that's not possible
// with const
//
