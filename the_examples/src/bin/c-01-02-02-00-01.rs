// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

// This is how to implement your own display formatter.
// This needs to be moved a lot further down the pipe
// since we don't know how to do struts yet. Or 
// `impl` either, or what traits are. 

use std::fmt;

struct Alfa(i32);

impl fmt::Display for Alfa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    println!("Alfa is {}", Alfa(6262));
}

// I don't totally undestand what's happening there. 
// The notes from the page say:

/*
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}
*/





// NOTE that the Rust By Example page doesn't actually 
// use the `impl` example code. I had to guess how to 
// use it in main. 

// OUTPUTS:
// Alfa is 6262


