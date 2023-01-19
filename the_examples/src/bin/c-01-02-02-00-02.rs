// https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

// Example with two things passed in and some 
// other formatting. 

use std::fmt;

struct Bravo(i32, i32);

impl fmt::Display for Bravo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "First {} Second {}", self.0, self.1)
    }
}

fn main() {
    println!("Values {}", Bravo(43, 12))
}

// OUTPUTS:
// Values First 43 Second 12

// TODO: Add example showing how this doesn't automatically
// implement `{:b}` or the other `fmt` traits and each one
// requries it's own implementation. 


// TODO: Show errors in the first version of this with things like:
/*
error[E0277]: `Bravo` doesn't implement `std::fmt::Display`
  --> src/bin/c-01-02-02-00-02.rs:11:27
   |
11 |     println!("Values {}", Bravo(43, 12))
   |                           ^^^^^^^^^^^^^ `Bravo` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Bravo`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
warning: `the_examples` (bin "c-01-02-02-00-02") generated 1 warning
error: could not compile `the_examples` due to previous error; 1 warning emitted
*/



